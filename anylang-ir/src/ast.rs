use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommonAstTag {
    FunDef,
    FunCall,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Le,
    Ge,
    Lte,
    Gte,
    Number,
    Var,
    Cond
}

impl fmt::Display for CommonAstTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CommonAstTag::FunDef => "FunDef",
            CommonAstTag::FunCall => "FunCall",
            CommonAstTag::Add => "Add",
            CommonAstTag::Sub => "Sub",
            CommonAstTag::Mul => "Mul",
            CommonAstTag::Div => "Div",
            CommonAstTag::Eq => "Eq",
            CommonAstTag::Le => "Le",
            CommonAstTag::Ge => "Ge",
            CommonAstTag::Lte => "Lte",
            CommonAstTag::Gte => "Gte",
            CommonAstTag::Number => "Number",
            CommonAstTag::Var => "Var",
            CommonAstTag::Cond => "Cond"
        };
        write!(f, "{}", s)
    }
}
