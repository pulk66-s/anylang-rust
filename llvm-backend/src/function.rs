use anylang_ir::{CommonIRTag, CommonIRFunDef, CommonIRFunCall};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use inkwell::types::BasicMetadataTypeEnum;
use std::collections::HashMap;
use crate::expression::ExpressionCompiler;

pub fn declare_function<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
    functions: &mut HashMap<String, FunctionValue<'ctx>>,
    fun_def: &CommonIRFunDef,
) -> Result<(), String> {
    let i64_type = context.i64_type();
    let param_types: Vec<BasicMetadataTypeEnum> = fun_def.params
        .iter()
        .map(|_| i64_type.into())
        .collect();
    
    let fn_type = i64_type.fn_type(&param_types, false);
    let function = module.add_function(&fun_def.name, fn_type, None);
    
    functions.insert(fun_def.name.to_string(), function);
    
    Ok(())
}

pub fn compile_function<'ctx>(
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    functions: &'ctx HashMap<String, FunctionValue<'ctx>>,
    fun_def: &CommonIRFunDef,
) -> Result<(), String> {
    let mut variables: HashMap<String, PointerValue<'ctx>> = HashMap::new();
    
    let function = functions.get(&fun_def.name)
        .ok_or_else(|| format!("Function '{}' not declared", fun_def.name))?
        .clone();
    
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);
    
    // Allocate space for parameters
    for (i, param_name) in fun_def.params.iter().enumerate() {
        let param_value = function.get_nth_param(i as u32)
            .ok_or_else(|| format!("Parameter {} not found", i))?
            .into_int_value();
        
        let alloca = builder.build_alloca(context.i64_type(), param_name)
            .map_err(|e| format!("Failed to allocate parameter: {}", e))?;
        builder.build_store(alloca, param_value)
            .map_err(|e| format!("Failed to store parameter: {}", e))?;
        
        variables.insert(param_name.clone(), alloca);
    }
    
    // Compile function body
    let mut expr_compiler = ExpressionCompiler::new(
        context,
        builder,
        functions,
        &variables,
    );

    let mut last_value = None;
    for node in &fun_def.body {
        last_value = Some(expr_compiler.compile(node)?);
    }
    
    // Return the last value
    if let Some(value) = last_value {
        builder.build_return(Some(&value))
            .map_err(|e| format!("Failed to build return: {}", e))?;
    } else {
        return Err("Function body is empty".to_string());
    }
    
    Ok(())
}

pub fn compile_call_expression<'ctx>(
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    functions: &'ctx HashMap<String, FunctionValue<'ctx>>,
    fun_call: &CommonIRFunCall,
) -> Result<(), String> {
    // Compile the call
    let variables = HashMap::new();
    let mut expr_compiler = ExpressionCompiler::new(
        context,
        builder,
        functions,
        &variables,
    );
    let result = expr_compiler.compile(&CommonIRTag::FunCall(fun_call.clone()))?;
    
    // Return the result as exit status code
    builder.build_return(Some(&result))
        .map_err(|e| format!("Failed to build return: {}", e))?;
    
    Ok(())
}
