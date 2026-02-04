use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use serde_json::Value;
use std::collections::HashMap;


pub struct LLVMBackend<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    functions: HashMap<String, FunctionValue<'ctx>>,
    top_level_expressions: Vec<Value>,
}

impl<'ctx> LLVMBackend<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("main");
        let builder = context.create_builder();

        Self {
            context,
            module,
            builder,
            functions: HashMap::new(),
            top_level_expressions: Vec::new(),
        }
    }

    pub fn compile(&mut self, ast_json: &str, output_path: &str) -> Result<(), String> {
        let ast = self.parse_json(ast_json)?;
        Ok(())
    }

    fn parse_json(&self, ast_json: &str) -> Result<Value, String> {
        serde_json::from_str(ast_json).map_err(|e| format!("Invalid JSON AST: {}", e))
    }
}
