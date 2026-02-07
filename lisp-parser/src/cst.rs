use crate::span::Spanned;
use crate::lexer::{Lexer, LexerParser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LispCst {
    Atom(Spanned<String>),
    Number(Spanned<f64>),
    List(Spanned<Vec<LispCst>>),
    Nil,
}

pub struct CstParser {
    tokens: Vec<Lexer>,
    index: usize,
}

impl CstParser {
    pub fn new(str: &str) -> Result<Self, String> {
        Ok(CstParser {
            tokens: LexerParser::parse(str)?,
            index: 0,
        })
    }

    pub fn parse(&mut self) -> Result<LispCst, String> {
        self.list()
    }

    fn list(&mut self) -> Result<LispCst, String> {
        let mut elements = Vec::new();

        self.lpar()?;
        while let Some(tok) = self.tokens.get(self.index) {
            if matches!(tok, Lexer::RPar(_)) {
                break;
            }
            self.list()
                .or_else(|_| self.atom())
                .or_else(|_| self.number())
                .map(|elem| elements.push(elem))?;
        }
        self.rpar()?;
        Ok(LispCst::List(Spanned::new(elements)))
    }

    fn atom(&mut self) -> Result<LispCst, String> {
        match self.tokens.get(self.index) {
            Some(Lexer::Atom(s)) => {
                self.index += 1;
                Ok(LispCst::Atom(s.clone()))
            }
            Some(tok) => Err(format!("Expected atom, found {:?}", tok)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn number(&mut self) -> Result<LispCst, String> {
        match self.tokens.get(self.index) {
            Some(Lexer::Number(n)) => {
                self.index += 1;
                Ok(LispCst::Number(n.clone()))
            }
            Some(tok) => Err(format!("Expected number, found {:?}", tok)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn lpar(&mut self) -> Result<(), String> {
        match self.tokens.get(self.index) {
            Some(Lexer::LPar(_)) => {
                self.index += 1;
                Ok(())
            }
            Some(tok) => Err(format!("Expected '(', found {:?}", tok)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn rpar(&mut self) -> Result<(), String> {
        match self.tokens.get(self.index) {
            Some(Lexer::RPar(_)) => {
                self.index += 1;
                Ok(())
            }
            Some(tok) => Err(format!("Expected ')', found {:?}", tok)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}
