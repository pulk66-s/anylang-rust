use serde::{Deserialize, Serialize};

use crate::{LispCst, cst::CstParser, span::Spanned};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Le,
    Ge,
    Lte,
    Gte,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Tag {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tagged<T> {
    tag: Tag,
    value: T,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuncDef {
    name: String,
    params: Vec<String>,
    body: Vec<TaggedAst>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpCall {
    op: Op,
    args: Vec<TaggedAst>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunCall {
    name: String,
    args: Vec<TaggedAst>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cond {
    cond: Box<TaggedAst>,
    then_branch: Vec<TaggedAst>,
    else_branch: Vec<TaggedAst>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaggedAst {
    FunDef(Tagged<FuncDef>),
    FunCall(Tagged<FunCall>),
    Cond(Tagged<Cond>),
    Number(Tagged<f64>),
    Add(Tagged<OpCall>),
    Sub(Tagged<OpCall>),
    Mul(Tagged<OpCall>),
    Div(Tagged<OpCall>),
    Eq(Tagged<OpCall>),
    Le(Tagged<OpCall>),
    Ge(Tagged<OpCall>),
    Lte(Tagged<OpCall>),
    Gte(Tagged<OpCall>),
    Var(Tagged<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaggedAstParser {
    cst: LispCst,
}

impl TaggedAstParser {
    pub fn new(str: &str) -> Result<Self, String> {
        Ok(TaggedAstParser {
            cst: CstParser::new(str)?.parse()?,
        })
    }

    fn ensure_names(&self, atoms: &Vec<LispCst>) -> Result<Vec<String>, String> {
        atoms.into_iter().map(|a| match a {
            LispCst::Atom(s) => Ok(s.value.clone()),
            _ => Err("Expected atom".to_string()),
        }).collect()
    }

    fn parse_many_body(&self, list: &Spanned<Vec<LispCst>>) -> Result<Vec<TaggedAst>, String> {
        list.value.iter().map(|e| self.parse_elem(e)).collect()
    }

    fn parse_defun(&self, list: &Spanned<Vec<LispCst>>) -> Result<TaggedAst, String> {
        let values = &list.value;
        let name_atom = values.get(1).ok_or("Expected function name")?;
        let param_atoms = values.get(2).ok_or("Expected parameter list")?;
        let body_elements = &values[3..];

        if body_elements.is_empty() {
            return Err("Expected function body".to_string());
        }
        match (name_atom, param_atoms) {
            (
                LispCst::Atom(name), 
                LispCst::List(params)
            ) => {
                let body = body_elements.iter().map(|e| self.parse_elem(e)).collect::<Result<Vec<_>, _>>()?;

                Ok(TaggedAst::FunDef(Tagged {
                    tag: Tag::FunDef,
                    value: FuncDef { 
                        name: name.value.clone(),
                        params: self.ensure_names(&params.value)?, 
                        body
                    }
                }))
            },
            _ => Err(self.format_spanned_error("Invalid defun syntax", list)),
        }
    }

    fn parse_op(&self, sign: &str, args: &Vec<LispCst>) -> Result<TaggedAst, String> {
        let ast_args = self.parse_all(args)?;

        match sign {
            "+" => Ok(TaggedAst::Add(Tagged { tag: Tag::Add, value: OpCall { op: Op::Add, args: ast_args } })),
            "-" => Ok(TaggedAst::Sub(Tagged { tag: Tag::Sub, value: OpCall { op: Op::Sub, args: ast_args } })),
            "*" => Ok(TaggedAst::Mul(Tagged { tag: Tag::Mul, value: OpCall { op: Op::Mul, args: ast_args } })),
            "/" => Ok(TaggedAst::Div(Tagged { tag: Tag::Div, value: OpCall { op: Op::Div, args: ast_args } })),
            "==" => Ok(TaggedAst::Eq(Tagged { tag: Tag::Eq, value: OpCall { op: Op::Eq, args: ast_args } })),
            "<" => Ok(TaggedAst::Le(Tagged { tag: Tag::Le, value: OpCall { op: Op::Le, args: ast_args } })),
            ">" => Ok(TaggedAst::Ge(Tagged { tag: Tag::Ge, value: OpCall { op: Op::Ge, args: ast_args } })),
            "<=" => Ok(TaggedAst::Lte(Tagged { tag: Tag::Lte, value: OpCall { op: Op::Lte, args: ast_args } })),
            ">=" => Ok(TaggedAst::Gte(Tagged { tag: Tag::Gte, value: OpCall { op: Op::Gte, args: ast_args } })),
            e => Err(format!("Unknown operator: {}", e)),
        }
    }

    fn parse_cond(&self, list: &Spanned<Vec<LispCst>>) -> Result<TaggedAst, String> {
        println!("Parsing cond with list: {:?}", list);
        let values = &list.value;
        let cond = self.parse_elem(values.get(1).ok_or("Expected condition")?)?;
        let then_branch = self.parse_elem(values.get(2).ok_or("Expected then branch")?)?;
        let else_branch = self.parse_elem(values.get(3).ok_or("Expected else branch")?)?;

        Ok(TaggedAst::Cond(Tagged {
            tag: Tag::Cond,
            value: Cond { cond: Box::new(cond), then_branch: vec![then_branch], else_branch: vec![else_branch] },
        }))
    }

    fn parse_known_function(&self, list: &Spanned<Vec<LispCst>>) -> Result<TaggedAst, String> {
        let first = list.value.first().ok_or("Expected function name")?;

        match first {
            LispCst::Atom(s) if s.value == "defun"
                => self.parse_defun(list),
            LispCst::Atom(s) if s.value == "if"
                => self.parse_cond(list),
            LispCst::Atom(s) if matches!(s.value.as_str(), "+" | "-" | "*" | "/" | "==" | "<" | ">" | "<=" | ">=") 
                => self.parse_op(s.value.as_str(), &list.value[1..].to_vec()),
            _ => Err(self.format_spanned_error("Unknown function", list))
        }
    }

    fn format_spanned_error(&self, message: &str, span: &Spanned<impl std::fmt::Debug>) -> String {
        format!("ERROR: {} at span {:?}", message, span)
    }

    fn parse_all(&self, list: &Vec<LispCst>) -> Result<Vec<TaggedAst>, String> {
        list.iter().map(|e| self.parse_elem(e)).collect()
    }

    fn parse_func_call(&self, args: &Vec<LispCst>) -> Result<TaggedAst, String> {
        let name = match args.first() {
            Some(LispCst::Atom(s)) => s.value.clone(),
            _ => return Err("Expected function name in function call".to_string()),
        };
        let ast_args = self.parse_all(&args[1..].to_vec())?;

        Ok(TaggedAst::FunCall(Tagged {
            tag: Tag::FunCall,
            value: FunCall { name, args: ast_args },
        }))
    }

    fn parse_elem(&self, elem: &LispCst) -> Result<TaggedAst, String> {
        println!("Parsing element: {:?}", elem);
        match elem {
            LispCst::List(l) => self.parse_known_function(l).or(self.parse_func_call(&l.value)),
            LispCst::Atom(s) => Ok(TaggedAst::Var(Tagged { tag: Tag::Var, value: s.value.clone() })),
            LispCst::Number(n) => Ok(TaggedAst::Number(Tagged { tag: Tag::Number, value: n.value })),
            _ => Err(format!("Unexpected CST node: {:?}", elem)),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<TaggedAst>, String> {
        println!("CST: {:#?}", self.cst);
        match &self.cst {
            LispCst::List(l) => self.parse_known_function(&l).map(|ast| vec![ast]),
            _ => Err("Expected top-level list".to_string()),
        }
    }
}
