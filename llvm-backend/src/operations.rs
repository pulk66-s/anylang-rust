use anylang_ir::CommonOpTag;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::IntValue;
use inkwell::IntPredicate;

pub struct OperationCompiler<'ctx> {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
}

impl<'ctx> OperationCompiler<'ctx> {
    pub fn new(context: &'ctx Context, builder: &'ctx Builder<'ctx>) -> Self {
        Self { context, builder }
    }

    pub fn compile_binary_op(
        &self,
        op: &CommonOpTag,
        left: IntValue<'ctx>,
        right: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, String> {
        match op {
            CommonOpTag::Add => self.compile_add(left, right),
            CommonOpTag::Sub => self.compile_sub(left, right),
            CommonOpTag::Mul => self.compile_mul(left, right),
            CommonOpTag::Div => self.compile_div(left, right),
            CommonOpTag::Eq => self.compile_eq(left, right),
            CommonOpTag::Neq => self.compile_neq(left, right),
            CommonOpTag::Lt => self.compile_lt(left, right),
            CommonOpTag::Gt => self.compile_gt(left, right),
            CommonOpTag::Lte => self.compile_lte(left, right),
            CommonOpTag::Gte => self.compile_gte(left, right),
        }
    }

    fn compile_add(&self, left: IntValue<'ctx>, right: IntValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.builder.build_int_add(left, right, "add")
            .map_err(|e| format!("Failed to build add: {}", e))
    }

    fn compile_sub(&self, left: IntValue<'ctx>, right: IntValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.builder.build_int_sub(left, right, "sub")
            .map_err(|e| format!("Failed to build sub: {}", e))
    }

    fn compile_mul(&self, left: IntValue<'ctx>, right: IntValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.builder.build_int_mul(left, right, "mul")
            .map_err(|e| format!("Failed to build mul: {}", e))
    }

    fn compile_div(&self, left: IntValue<'ctx>, right: IntValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.builder.build_int_signed_div(left, right, "div")
            .map_err(|e| format!("Failed to build div: {}", e))
    }

    fn compile_comparison(
        &self,
        pred: IntPredicate,
        left: IntValue<'ctx>,
        right: IntValue<'ctx>,
        name: &str,
    ) -> Result<IntValue<'ctx>, String> {
        let cmp = self.builder.build_int_compare(pred, left, right, name)
            .map_err(|e| format!("Failed to build {}: {}", name, e))?;
        self.builder.build_int_z_extend(cmp, self.context.i64_type(), &format!("{}_ext", name))
            .map_err(|e| format!("Failed to extend {}: {}", name, e))
    }

    fn compile_eq(&self, left: IntValue<'ctx>, right: IntValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.compile_comparison(IntPredicate::EQ, left, right, "eq")
    }

    fn compile_neq(&self, left: IntValue<'ctx>, right: IntValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.compile_comparison(IntPredicate::NE, left, right, "neq")
    }

    fn compile_lt(&self, left: IntValue<'ctx>, right: IntValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.compile_comparison(IntPredicate::SLT, left, right, "lt")
    }

    fn compile_gt(&self, left: IntValue<'ctx>, right: IntValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.compile_comparison(IntPredicate::SGT, left, right, "gt")
    }

    fn compile_lte(&self, left: IntValue<'ctx>, right: IntValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.compile_comparison(IntPredicate::SLE, left, right, "lte")
    }

    fn compile_gte(&self, left: IntValue<'ctx>, right: IntValue<'ctx>) -> Result<IntValue<'ctx>, String> {
        self.compile_comparison(IntPredicate::SGE, left, right, "gte")
    }
}
