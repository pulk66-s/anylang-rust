use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{FunctionValue, IntValue};
use inkwell::module::Module;
use serde_json::Value;
use std::collections::HashMap;

use crate::codegen::CodeGenerator;

pub struct FunctionCompiler;

impl FunctionCompiler {
    pub fn compile_function_def<'ctx>(
        items: &[Value],
        functions: &mut HashMap<String, FunctionValue<'ctx>>,
        module: &Module<'ctx>,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
    ) -> Result<FunctionValue<'ctx>, String> {
        let func_name = Self::extract_function_name(items)?;
        let params = Self::extract_parameters(items)?;

        let function = Self::create_function_signature(&func_name, &params, module, context);
        
        // Register function before compiling body (allows recursion)
        functions.insert(func_name, function);

        let param_map = Self::create_parameter_map(&params, function);
        
        Self::compile_function_body(items, &param_map, function, functions, context, builder)?;

        Ok(function)
    }

    fn extract_function_name(items: &[Value]) -> Result<String, String> {
        items
            .get(1)
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get("Atom"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "Expected function name".to_string())
    }

    fn extract_parameters(items: &[Value]) -> Result<Vec<String>, String> {
        let params = items
            .get(2)
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get("List"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        v.as_object()
                            .and_then(|o| o.get("Atom"))
                            .and_then(|s| s.as_str())
                            .map(|s| s.to_string())
                    })
                    .collect()
            })
            .unwrap_or_else(Vec::new);

        Ok(params)
    }

    fn create_function_signature<'ctx>(
        name: &str,
        params: &[String],
        module: &Module<'ctx>,
        context: &'ctx Context,
    ) -> FunctionValue<'ctx> {
        let i64_type = context.i64_type();
        let param_types: Vec<BasicMetadataTypeEnum> =
            params.iter().map(|_| i64_type.into()).collect();

        let fn_type = i64_type.fn_type(&param_types, false);
        module.add_function(name, fn_type, None)
    }

    fn create_parameter_map<'ctx>(
        params: &[String],
        function: FunctionValue<'ctx>,
    ) -> HashMap<String, IntValue<'ctx>> {
        params
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let param_value = function
                    .get_nth_param(i as u32)
                    .unwrap()
                    .into_int_value();
                (name.clone(), param_value)
            })
            .collect()
    }

    fn compile_function_body<'ctx>(
        items: &[Value],
        param_map: &HashMap<String, IntValue<'ctx>>,
        function: FunctionValue<'ctx>,
        functions: &HashMap<String, FunctionValue<'ctx>>,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
    ) -> Result<(), String> {
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let body = items.get(3).ok_or("Expected function body")?;
        
        let codegen = CodeGenerator::new(context, builder, functions);
        let result = codegen.compile_expression(body, param_map, function)?;

        builder
            .build_return(Some(&result))
            .map_err(|e| format!("Failed to build return: {}", e))?;

        Ok(())
    }
}
