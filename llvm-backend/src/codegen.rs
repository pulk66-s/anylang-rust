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
}
