use anylang_ir::{CommonIRTag, CommonIRFunDef};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use std::collections::HashMap;
use crate::function;

pub struct LLVMBackend<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    functions: HashMap<String, FunctionValue<'ctx>>,
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
        }
    }

    pub fn compile(&mut self, ast_json: &str, output_path: &str) -> Result<(), String> {
        let ir: Vec<CommonIRTag> = serde_json::from_str(ast_json)
            .map_err(|e| format!("Invalid JSON AST: {}", e))?;
        
        // First pass: declare all functions
        self.declare_functions(&ir)?;
        
        // Second pass: compile function bodies and top-level calls
        self.compile_ir(&ir)?;
        
        // Verify and write output
        self.finalize(output_path)?;
        
        Ok(())
    }

    fn declare_functions(&mut self, ir: &[CommonIRTag]) -> Result<(), String> {
        for node in ir {
            if let CommonIRTag::FunDef(fun_def) = node {
                function::declare_function(
                    self.context,
                    &self.module,
                    &mut self.functions,
                    fun_def,
                )?;
            }
        }
        
        Ok(())
    }

    fn compile_ir(&mut self, ir: &[CommonIRTag]) -> Result<(), String> {
        // First handle function definitions
        for node in ir {
            if let CommonIRTag::FunDef(fun_def) = node {
                self.compile_function_def(fun_def)?;
            }
        }
        
        // Then handle top-level calls
        for node in ir {
            if let CommonIRTag::FunCall(fun_call) = node {
                self.compile_top_level_call(fun_call)?;
            }
        }
        
        Ok(())
    }

    fn compile_top_level_call(&mut self, fun_call: &anylang_ir::CommonIRFunCall) -> Result<(), String> {
        // Create a main function if it doesn't exist
        if !self.functions.contains_key("main") {
            let i64_type = self.context.i64_type();
            let fn_type = i64_type.fn_type(&[], false);
            let main_fn = self.module.add_function("main", fn_type, None);
            self.functions.insert("main".to_string(), main_fn);
            
            let entry = self.context.append_basic_block(main_fn, "entry");
            self.builder.position_at_end(entry);
        }
        
        // Compile the call
        function::compile_call_expression(
            self.context,
            &self.builder,
            &self.functions,
            fun_call,
        )
    }

    fn compile_function_def(&mut self, fun_def: &CommonIRFunDef) -> Result<(), String> {
        function::compile_function(
            self.context,
            &self.builder,
            &self.functions,
            fun_def,
        )
    }

    fn finalize(&self, output_path: &str) -> Result<(), String> {
        // Verify the module
        if let Err(e) = self.module.verify() {
            return Err(format!("Module verification failed: {}", e.to_string()));
        }
        
        // Write LLVM IR to file
        let ir_output = format!("{}.ll", output_path);
        self.module.print_to_file(&ir_output)
            .map_err(|e| format!("Failed to write LLVM IR: {}", e.to_string()))?;
        
        println!("LLVM IR written to: {}", ir_output);
        
        Ok(())
    }
}
