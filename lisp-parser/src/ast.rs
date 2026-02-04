use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LispAst {
    Atom(String),
    Number(f64),
    List(Vec<LispAst>),
    Nil,
}
