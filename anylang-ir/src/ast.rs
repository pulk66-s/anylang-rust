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
    Neq,
    Le,
    Ge,
    Lte,
    Gte,
    Number,
    Var,
    Cond
}

impl CommonAstTag {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "FunDef" => Ok(CommonAstTag::FunDef),
            "FunCall" => Ok(CommonAstTag::FunCall),
            "Add" => Ok(CommonAstTag::Add),
            "Sub" => Ok(CommonAstTag::Sub),
            "Mul" => Ok(CommonAstTag::Mul),
            "Div" => Ok(CommonAstTag::Div),
            "Eq" => Ok(CommonAstTag::Eq),
            "Neq" => Ok(CommonAstTag::Neq),
            "Le" => Ok(CommonAstTag::Le),
            "Ge" => Ok(CommonAstTag::Ge),
            "Lte" => Ok(CommonAstTag::Lte),
            "Gte" => Ok(CommonAstTag::Gte),
            "Number" => Ok(CommonAstTag::Number),
            "Var" => Ok(CommonAstTag::Var),
            "Cond" => Ok(CommonAstTag::Cond),
            _ => Err(format!("Unknown AST tag: {}", s)),
        }
    }

    fn to_string(&self) -> String {
        match self {
            CommonAstTag::FunDef => "FunDef".to_string(),
            CommonAstTag::FunCall => "FunCall".to_string(),
            CommonAstTag::Add => "Add".to_string(),
            CommonAstTag::Sub => "Sub".to_string(),
            CommonAstTag::Mul => "Mul".to_string(),
            CommonAstTag::Div => "Div".to_string(),
            CommonAstTag::Eq => "Eq".to_string(),
            CommonAstTag::Neq => "Neq".to_string(),
            CommonAstTag::Le => "Le".to_string(),
            CommonAstTag::Ge => "Ge".to_string(),
            CommonAstTag::Lte => "Lte".to_string(),
            CommonAstTag::Gte => "Gte".to_string(),
            CommonAstTag::Number => "Number".to_string(),
            CommonAstTag::Var => "Var".to_string(),
            CommonAstTag::Cond => "Cond".to_string()
        }
    }
}

impl fmt::Display for CommonAstTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommonOpTag {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl CommonOpTag {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Add" => Ok(CommonOpTag::Add),
            "Sub" => Ok(CommonOpTag::Sub),
            "Mul" => Ok(CommonOpTag::Mul),
            "Div" => Ok(CommonOpTag::Div),
            "Eq" => Ok(CommonOpTag::Eq),
            "Neq" => Ok(CommonOpTag::Neq),
            "Le" => Ok(CommonOpTag::Lt),
            "Ge" => Ok(CommonOpTag::Gt),
            "Lte" => Ok(CommonOpTag::Lte),
            "Gte" => Ok(CommonOpTag::Gte),
            _ => Err(format!("Unknown Op tag: {}", s)),
        }
    }

    fn to_string(&self) -> String {
        match self {
            CommonOpTag::Add => "Add".to_string(),
            CommonOpTag::Sub => "Sub".to_string(),
            CommonOpTag::Mul => "Mul".to_string(),
            CommonOpTag::Div => "Div".to_string(),
            CommonOpTag::Eq => "Eq".to_string(),
            CommonOpTag::Lt => "Lt".to_string(),
            CommonOpTag::Gt => "Gt".to_string(),
            CommonOpTag::Lte => "Lte".to_string(),
            CommonOpTag::Gte => "Gte".to_string(),
            CommonOpTag::Neq => "Neq".to_string(),
        }
    }
}

impl fmt::Display for CommonOpTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
