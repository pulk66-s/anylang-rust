use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{FunctionValue, IntValue};
use inkwell::IntPredicate;
use serde_json::Value;
use std::collections::HashMap;

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    functions: &'ctx HashMap<String, FunctionValue<'ctx>>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        functions: &'ctx HashMap<String, FunctionValue<'ctx>>,
    ) -> Self {
        Self {
            context,
            builder,
            functions,
        }
    }

    pub fn compile_expression(
        &self,
        expr: &Value,
        vars: &HashMap<String, IntValue<'ctx>>,
        function: FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, String> {
        match expr {
            Value::Object(obj) => {
                if let Some(Value::String(s)) = obj.get("Atom") {
                    self.compile_variable(s, vars)
                } else if let Some(n) = obj.get("Number") {
                    self.compile_number(n)
                } else if let Some(Value::Array(arr)) = obj.get("List") {
                    self.compile_list(arr, vars, function)
                } else {
                    Err("Unknown object type".to_string())
                }
            }
            _ => Err("Invalid expression type".to_string()),
        }
    }

    fn compile_variable(
        &self,
        var_name: &str,
        vars: &HashMap<String, IntValue<'ctx>>,
    ) -> Result<IntValue<'ctx>, String> {
        vars.get(var_name)
            .copied()
            .ok_or_else(|| format!("Unknown variable: {}", var_name))
    }

    fn compile_number(&self, n: &Value) -> Result<IntValue<'ctx>, String> {
        let int_val = if let Some(u) = n.as_u64() {
            u
        } else if let Some(i) = n.as_i64() {
            i as u64
        } else if let Some(f) = n.as_f64() {
            f as u64
        } else {
            return Err("Invalid number format".to_string());
        };
        
        Ok(self.context.i64_type().const_int(int_val, false))
    }

    fn compile_list(
        &self,
        arr: &[Value],
        vars: &HashMap<String, IntValue<'ctx>>,
        function: FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, String> {
        if arr.is_empty() {
            return Err("Empty expression".to_string());
        }

        let op = arr
            .get(0)
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get("Atom"))
            .and_then(|v| v.as_str())
            .ok_or("Invalid operator")?;

        self.compile_operation(op, arr, vars, function)
    }

    fn compile_operation(
        &self,
        op: &str,
        args: &[Value],
        vars: &HashMap<String, IntValue<'ctx>>,
        function: FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, String> {
        match op {
            "if" => self.compile_if(args, vars, function),
            "<=" => self.compile_comparison(IntPredicate::SLE, args, vars, function),
            "*" => self.compile_mul(args, vars, function),
            "-" => self.compile_sub(args, vars, function),
            "+" => self.compile_add(args, vars, function),
            _ => self.compile_function_call(op, args, vars, function),
        }
    }

    fn compile_if(
        &self,
        args: &[Value],
        vars: &HashMap<String, IntValue<'ctx>>,
        function: FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, String> {
        let cond_expr = args.get(1).ok_or("Expected condition")?;
        let then_expr = args.get(2).ok_or("Expected then branch")?;
        let else_expr = args.get(3).ok_or("Expected else branch")?;

        let cond_val = self.compile_expression(cond_expr, vars, function)?;

        let then_block = self.context.append_basic_block(function, "then");
        let else_block = self.context.append_basic_block(function, "else");
        let merge_block = self.context.append_basic_block(function, "merge");

        // Compare condition to zero (false if zero)
        let zero = self.context.i64_type().const_int(0, false);
        let cond = self
            .builder
            .build_int_compare(IntPredicate::NE, cond_val, zero, "ifcond")
            .map_err(|e| format!("Failed to build compare: {}", e))?;

        self.builder
            .build_conditional_branch(cond, then_block, else_block)
            .map_err(|e| format!("Failed to build branch: {}", e))?;

        // Then block
        self.builder.position_at_end(then_block);
        let then_val = self.compile_expression(then_expr, vars, function)?;
        self.builder
            .build_unconditional_branch(merge_block)
            .map_err(|e| format!("Failed to build branch: {}", e))?;
        let then_end_block = self.builder.get_insert_block().unwrap();

        // Else block
        self.builder.position_at_end(else_block);
        let else_val = self.compile_expression(else_expr, vars, function)?;
        self.builder
            .build_unconditional_branch(merge_block)
            .map_err(|e| format!("Failed to build branch: {}", e))?;
        let else_end_block = self.builder.get_insert_block().unwrap();

        // Merge block
        self.builder.position_at_end(merge_block);
        let phi = self
            .builder
            .build_phi(self.context.i64_type(), "iftmp")
            .map_err(|e| format!("Failed to build phi: {}", e))?;
        phi.add_incoming(&[(&then_val, then_end_block), (&else_val, else_end_block)]);

        Ok(phi.as_basic_value().into_int_value())
    }

    fn compile_comparison(
        &self,
        predicate: IntPredicate,
        args: &[Value],
        vars: &HashMap<String, IntValue<'ctx>>,
        function: FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, String> {
        let left = self.compile_expression(args.get(1).ok_or("Expected left operand")?, vars, function)?;
        let right = self.compile_expression(args.get(2).ok_or("Expected right operand")?, vars, function)?;
        
        let cmp = self
            .builder
            .build_int_compare(predicate, left, right, "cmp")
            .map_err(|e| format!("Failed to build compare: {}", e))?;
        
        // Extend boolean to i64
        self.builder
            .build_int_z_extend(cmp, self.context.i64_type(), "cmpext")
            .map_err(|e| format!("Failed to build extend: {}", e))
    }

    fn compile_mul(
        &self,
        args: &[Value],
        vars: &HashMap<String, IntValue<'ctx>>,
        function: FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, String> {
        let left = self.compile_expression(args.get(1).ok_or("Expected left operand")?, vars, function)?;
        let right = self.compile_expression(args.get(2).ok_or("Expected right operand")?, vars, function)?;
        
        self.builder
            .build_int_mul(left, right, "multmp")
            .map_err(|e| format!("Failed to build mul: {}", e))
    }

    fn compile_sub(
        &self,
        args: &[Value],
        vars: &HashMap<String, IntValue<'ctx>>,
        function: FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, String> {
        let left = self.compile_expression(args.get(1).ok_or("Expected left operand")?, vars, function)?;
        let right = self.compile_expression(args.get(2).ok_or("Expected right operand")?, vars, function)?;
        
        self.builder
            .build_int_sub(left, right, "subtmp")
            .map_err(|e| format!("Failed to build sub: {}", e))
    }

    fn compile_add(
        &self,
        args: &[Value],
        vars: &HashMap<String, IntValue<'ctx>>,
        function: FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, String> {
        let left = self.compile_expression(args.get(1).ok_or("Expected left operand")?, vars, function)?;
        let right = self.compile_expression(args.get(2).ok_or("Expected right operand")?, vars, function)?;
        
        self.builder
            .build_int_add(left, right, "addtmp")
            .map_err(|e| format!("Failed to build add: {}", e))
    }

    fn compile_function_call(
        &self,
        func_name: &str,
        args: &[Value],
        vars: &HashMap<String, IntValue<'ctx>>,
        function: FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, String> {
        let func = self
            .functions
            .get(func_name)
            .ok_or_else(|| format!("Unknown function: {}", func_name))?;

        let call_args: Result<Vec<_>, String> = args[1..]
            .iter()
            .map(|arg| {
                self.compile_expression(arg, vars, function)
                    .map(|v| v.into())
            })
            .collect();

        let call_args = call_args?;
        let call = self
            .builder
            .build_call(*func, &call_args, "calltmp")
            .map_err(|e| format!("Failed to build call: {}", e))?;

        Ok(call.try_as_basic_value().left().unwrap().into_int_value())
    }
}
