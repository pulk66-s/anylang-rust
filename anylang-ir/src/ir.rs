use serde::{Deserialize, Serialize};

use crate::CommonOpTag;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommonIRTag {
    FunDef(CommonIRFunDef),
    FunCall(CommonIRFunCall),
    Cond(CommonCondIR),
    Op(CommonOpIR),
    Number(i64),
    Var(String)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonOpIR {
    op: CommonOpTag,
    left: Box<CommonIRTag>,
    right: Box<CommonIRTag>,
}

impl CommonOpIR {
    pub fn new(op: CommonOpTag, left: CommonIRTag, right: CommonIRTag) -> Self {
        CommonOpIR {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonCondIR {
    cond: Box<CommonIRTag>,
    then_branch: Vec<CommonIRTag>,
    else_branch: Vec<CommonIRTag>,
}

impl CommonCondIR {
    pub fn new(cond: CommonIRTag, then_branch: Vec<CommonIRTag>, else_branch: Vec<CommonIRTag>) -> Self {
        CommonCondIR {
            cond: Box::new(cond),
            then_branch,
            else_branch,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VarDecl {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonIRFunCall {
    name: String,
    args: Vec<CommonIRTag>,
}

impl CommonIRFunCall {
    pub fn new(name: String, args: Vec<CommonIRTag>) -> Self {
        CommonIRFunCall { name, args }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonIRFunDef {
    name: String,
    params: Vec<String>,
    body: Vec<CommonIRTag>,
    return_type: Option<String>,
}

impl CommonIRFunDef {
    pub fn new(
        name: String,
        params: Vec<String>,
        body: Vec<CommonIRTag>,
        return_type: Option<String>,
    ) -> Self {
        CommonIRFunDef {
            name,
            params,
            body,
            return_type,
        }
    }
}
