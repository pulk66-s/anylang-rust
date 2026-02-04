use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use serde_json::Value;
use std::collections::HashMap;
use std::process::Command;

use crate::ast_helper::AstHelper;
use crate::codegen::CodeGenerator;
use crate::function::FunctionCompiler;

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
        self.parse_program(&ast)?;
        self.create_main_function()?;
        self.generate_output(output_path)?;
        Ok(())
    }

    fn parse_json(&self, ast_json: &str) -> Result<Value, String> {
        serde_json::from_str(ast_json).map_err(|e| format!("Invalid JSON AST: {}", e))
    }

    fn generate_output(&self, output_path: &str) -> Result<(), String> {
        self.write_ir(output_path)?;
        self.compile_to_object(output_path)?;
        self.link_executable(output_path)?;
        Ok(())
    }

    fn parse_program(&mut self, ast: &Value) -> Result<(), String> {
        if let Some(items) = AstHelper::extract_list_items(ast) {
            self.process_program_items(&items)?;
        } else if AstHelper::is_function_call(ast) {
            self.top_level_expressions.push(ast.clone());
        }
        Ok(())
    }

    fn process_program_items(&mut self, items: &[Value]) -> Result<(), String> {
        if items.len() == 1 && AstHelper::is_function_definition(&items[0]) {
            return self.compile_function(&items[0]);
        }
        if self.all_items_are_valid(items) {
            return self.process_mixed_items(items);
        }
        self.compile_function_from_items(items)
    }

    fn all_items_are_valid(&self, items: &[Value]) -> bool {
        items.iter().all(|item| {
            AstHelper::is_function_definition(item) || AstHelper::is_function_call(item)
        })
    }

    fn process_mixed_items(&mut self, items: &[Value]) -> Result<(), String> {
        for item in items {
            if AstHelper::is_function_definition(item) {
                self.compile_function(item)?;
            } else if AstHelper::is_function_call(item) {
                self.top_level_expressions.push(item.clone());
            }
        }
        Ok(())
    }

    fn compile_function(&mut self, item: &Value) -> Result<(), String> {
        let func_items = AstHelper::extract_list_items(item)
            .ok_or("Invalid function definition structure")?;
        self.compile_function_from_items(&func_items)
    }

    fn compile_function_from_items(&mut self, items: &[Value]) -> Result<(), String> {
        FunctionCompiler::compile_function_def(
            items,
            &mut self.functions,
            &self.module,
            self.context,
            &self.builder,
        )?;
        Ok(())
    }

    fn create_main_function(&self) -> Result<(), String> {
        let main_function = self.define_main_function();
        let entry_block = self.context.append_basic_block(main_function, "entry");
        
        self.builder.position_at_end(entry_block);

        let codegen = CodeGenerator::new(self.context, &self.builder, &self.functions);
        let empty_vars = HashMap::new();
        let mut return_value = self.context.i64_type().const_int(0, false);
        
        for expr in &self.top_level_expressions {
            return_value = codegen.compile_expression(expr, &empty_vars, main_function)?;
        }
        
        self.builder
            .build_return(Some(&return_value))
            .map_err(|e| format!("Failed to build return: {}", e))?;

        Ok(())
    }

    fn define_main_function(&self) -> FunctionValue<'ctx> {
        let i64_type = self.context.i64_type();
        let main_fn_type = i64_type.fn_type(&[], false);
        self.module.add_function("main", main_fn_type, None)
    }

    fn write_ir(&self, output_path: &str) -> Result<(), String> {
        let ir_path = format!("{}.ll", output_path);
        self.module
            .print_to_file(&ir_path)
            .map_err(|e| format!("Failed to write LLVM IR: {}", e))
    }

    fn compile_to_object(&self, output_path: &str) -> Result<(), String> {
        let ir_path = format!("{}.ll", output_path);
        let obj_path = format!("{}.o", output_path);

        self.run_command("llc-18", &["-filetype=obj", &ir_path, "-o", &obj_path])
    }

    fn link_executable(&self, output_path: &str) -> Result<(), String> {
        let obj_path = format!("{}.o", output_path);

        self.run_command("gcc", &[&obj_path, "-o", output_path, "-no-pie"])
    }

    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<(), String> {
        let output = Command::new(cmd)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to run {}: {}", cmd, e))?;

        if !output.status.success() {
            return Err(format!(
                "{} failed: {}",
                cmd,
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }
}
