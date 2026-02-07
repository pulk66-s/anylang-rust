use anylang_ir::{CommonIRTag, CommonIRFunCall, CommonCondIR, CommonOpIR};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{BasicValueEnum, FunctionValue, IntValue, PointerValue};
use inkwell::IntPredicate;
use std::collections::HashMap;
use crate::operations::OperationCompiler;

pub struct ExpressionCompiler<'ctx> {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    functions: &'ctx HashMap<String, FunctionValue<'ctx>>,
    variables: &'ctx HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> ExpressionCompiler<'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        functions: &'ctx HashMap<String, FunctionValue<'ctx>>,
        variables: &'ctx HashMap<String, PointerValue<'ctx>>,
    ) -> Self {
        Self {
            context,
            builder,
            functions,
            variables,
        }
    }

    pub fn compile(&mut self, node: &CommonIRTag) -> Result<IntValue<'ctx>, String> {
        match node {
            CommonIRTag::Number(n) => self.compile_number(*n),
            CommonIRTag::Var(name) => self.compile_var(name),
            CommonIRTag::Op(op) => self.compile_op(op),
            CommonIRTag::Cond(cond) => self.compile_cond(cond),
            CommonIRTag::FunCall(call) => self.compile_call(call),
            _ => Err(format!("Unsupported expression: {:?}", node)),
        }
    }

    fn compile_number(&self, n: i64) -> Result<IntValue<'ctx>, String> {
        Ok(self.context.i64_type().const_int(n as u64, true))
    }

    fn compile_var(&self, name: &str) -> Result<IntValue<'ctx>, String> {
        let ptr = self.variables.get(name)
            .ok_or_else(|| format!("Variable '{}' not found", name))?;
        let value = self.builder.build_load(self.context.i64_type(), *ptr, name)
            .map_err(|e| format!("Failed to load variable: {}", e))?;
        match value {
            BasicValueEnum::IntValue(int_val) => Ok(int_val),
            _ => Err("Expected int value".to_string()),
        }
    }

    fn compile_op(&mut self, op: &CommonOpIR) -> Result<IntValue<'ctx>, String> {
        let left = self.compile(&op.left)?;
        let right = self.compile(&op.right)?;
        
        let op_compiler = OperationCompiler::new(self.context, self.builder);
        op_compiler.compile_binary_op(&op.op, left, right)
    }

    fn compile_cond(&mut self, cond: &CommonCondIR) -> Result<IntValue<'ctx>, String> {
        let cond_value = self.compile(&cond.cond)?;
        
        let parent_fn = self.builder.get_insert_block()
            .and_then(|bb| bb.get_parent())
            .ok_or("No parent function")?;
        
        let then_bb = self.context.append_basic_block(parent_fn, "then");
        let else_bb = self.context.append_basic_block(parent_fn, "else");
        let merge_bb = self.context.append_basic_block(parent_fn, "merge");
        
        // Convert condition to i1
        let zero = self.context.i64_type().const_zero();
        let cond_bool = self.builder.build_int_compare(IntPredicate::NE, cond_value, zero, "cond_bool")
            .map_err(|e| format!("Failed to build condition: {}", e))?;
        
        self.builder.build_conditional_branch(cond_bool, then_bb, else_bb)
            .map_err(|e| format!("Failed to build conditional branch: {}", e))?;
        
        // Then branch
        self.builder.position_at_end(then_bb);
        let mut then_value = None;
        for node in &cond.then_branch {
            then_value = Some(self.compile(node)?);
        }
        let then_value = then_value.ok_or("Then branch is empty")?;
        self.builder.build_unconditional_branch(merge_bb)
            .map_err(|e| format!("Failed to build branch: {}", e))?;
        let then_bb = self.builder.get_insert_block().unwrap();
        
        // Else branch
        self.builder.position_at_end(else_bb);
        let mut else_value = None;
        for node in &cond.else_branch {
            else_value = Some(self.compile(node)?);
        }
        let else_value = else_value.ok_or("Else branch is empty")?;
        self.builder.build_unconditional_branch(merge_bb)
            .map_err(|e| format!("Failed to build branch: {}", e))?;
        let else_bb = self.builder.get_insert_block().unwrap();
        
        // Merge
        self.builder.position_at_end(merge_bb);
        let phi = self.builder.build_phi(self.context.i64_type(), "cond_result")
            .map_err(|e| format!("Failed to build phi: {}", e))?;
        phi.add_incoming(&[(&then_value, then_bb), (&else_value, else_bb)]);
        
        Ok(phi.as_basic_value().into_int_value())
    }

    fn compile_call(&mut self, call: &CommonIRFunCall) -> Result<IntValue<'ctx>, String> {
        let function = self.functions.get(&call.name)
            .ok_or_else(|| format!("Function '{}' not found", call.name))?
            .clone();
        
        let args: Result<Vec<_>, _> = call.args
            .iter()
            .map(|arg| self.compile(arg).map(|v| v.into()))
            .collect();
        let args = args?;
        
        let call_result = self.builder.build_call(function, &args, "call")
            .map_err(|e| format!("Failed to build call: {}", e))?;
        
        match call_result.try_as_basic_value().left() {
            Some(BasicValueEnum::IntValue(int_val)) => Ok(int_val),
            Some(_) => Err("Expected int value from call".to_string()),
            None => Err("Function call did not return a value".to_string()),
        }
    }
}
