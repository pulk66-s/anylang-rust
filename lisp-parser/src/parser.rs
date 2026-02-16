use crate::{ast::TaggedAst, cst::CstParser, LispCst};
use serde::{Deserialize, Serialize};

/// Generic Lisp AST that represents the lisp.g4 grammar structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LispAst {
    /// Atomic symbol (identifier)
    Atom(String),
    /// Dotted pair: (car . cdr)
    DottedPair(Box<(LispAst, LispAst)>),
    /// List: (expr1 expr2 ...)
    List(Vec<LispAst>),
}

impl LispAst {
    /// Pretty print the ast
    pub fn pretty_print(&self) -> String {
        match self {
            LispAst::Atom(s) => s.clone(),
            LispAst::DottedPair(pair) => {
                format!("({} . {})", pair.0.pretty_print(), pair.1.pretty_print())
            }
            LispAst::List(exprs) => {
                let exprs_str = exprs.iter()
                    .map(|e| e.pretty_print())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("({})", exprs_str)
            }
        }
    }
}

/// Parser for Lisp according to lisp.g4 grammar
pub struct LispParser;

impl LispParser {
    pub fn new() -> Self {
        LispParser
    }

    /// Parse Lisp input and return a generic AST
    pub fn parse_generic(&self, input: &str) -> Result<Vec<LispAst>, String> {
        let mut cst_parser = CstParser::new(input)?;
        let mut results = Vec::new();
        
        while !cst_parser.is_at_end() {
            let cst = cst_parser.parse_s_expression()?;
            results.push(Self::cst_to_ast(cst)?);
        }
        
        if results.is_empty() {
            Err("No expressions to parse".to_string())
        } else {
            Ok(results)
        }
    }

    /// Parse Lisp and return all s-expressions as AST nodes
    pub fn parse_all(&self, input: &str) -> Result<Vec<LispAst>, String> {
        self.parse_generic(input)
    }

    /// Parse a single s-expression
    pub fn parse_single(&self, input: &str) -> Result<LispAst, String> {
        let mut cst_parser = CstParser::new(input)?;
        let cst = cst_parser.parse_s_expression()?;
        Self::cst_to_ast(cst)
    }

    /// Convert CST to generic AST
    fn cst_to_ast(cst: LispCst) -> Result<LispAst, String> {
        match cst {
            LispCst::Atom(a) => Ok(LispAst::Atom(a.value)),
            LispCst::Number(n) => Ok(LispAst::Atom(n.value.to_string())),
            LispCst::List(l) => {
                let asts: Result<Vec<_>, _> = l.value.iter()
                    .map(|cst| Self::cst_to_ast(cst.clone()))
                    .collect();
                Ok(LispAst::List(asts?))
            }
            LispCst::DottedPair(pair) => {
                let car = Self::cst_to_ast(pair.value.0)?;
                let cdr = Self::cst_to_ast(pair.value.1)?;
                Ok(LispAst::DottedPair(Box::new((car, cdr))))
            }
            LispCst::Nil => Ok(LispAst::List(vec![])),
        }
    }

    /// Alternative parse method that returns the old TaggedAst for compatibility
    pub fn parse(&self, input: &str) -> Result<Vec<TaggedAst>, String> {
        let mut ast_parser = crate::ast::TaggedAstParser::new(input)?;
        ast_parser.parse()
    }
}

