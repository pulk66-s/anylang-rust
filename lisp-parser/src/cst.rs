use crate::span::Spanned;
use crate::lexer::{Lexer, LexerParser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LispCst {
    Atom(Spanned<String>),
    Number(Spanned<f64>),
    List(Spanned<Vec<LispCst>>),
    DottedPair(Box<Spanned<(LispCst, LispCst)>>),
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

    /// Parse the next s-expression
    pub fn parse_s_expression(&mut self) -> Result<LispCst, String> {
        self.skip_past_end()?;
        self.s_expression()
    }

    /// Parse all s-expressions in the input (implements lisp_ rule)
    pub fn parse(&mut self) -> Result<LispCst, String> {
        let mut elements = Vec::new();
        
        while !self.is_at_end() {
            self.skip_past_end()?;
            if self.is_at_end() {
                break;
            }
            elements.push(self.s_expression()?);
        }
        
        if elements.is_empty() {
            Ok(LispCst::Nil)
        } else if elements.len() == 1 {
            Ok(elements.into_iter().next().unwrap())
        } else {
            // Wrap multiple expressions in a list
            Ok(LispCst::List(Spanned::new(elements)))
        }
    }

    /// s_expression: ATOMIC_SYMBOL | '(' s_expression '.' s_expression ')' | list
    fn s_expression(&mut self) -> Result<LispCst, String> {
        self.skip_past_end()?;
        
        // Try dotted pair: '(' s_expression '.' s_expression ')'
        if self.peek_token().map(|t| matches!(t, Lexer::LPar(_))).unwrap_or(false) {
            let saved_index = self.index;
            if let Ok(pair) = self.try_dotted_pair() {
                return Ok(pair);
            }
            // Reset if not a dotted pair
            self.index = saved_index;
        }

        // Try list: '(' s_expression+ ')'
        if self.peek_token().map(|t| matches!(t, Lexer::LPar(_))).unwrap_or(false) {
            return self.list();
        }

        // Try atomic symbol or number
        self.atom_or_number()
    }

    /// Try to parse a dotted pair: '(' s_expression '.' s_expression ')'
    fn try_dotted_pair(&mut self) -> Result<LispCst, String> {
        let _start_index = self.index;
        self.lpar()?;
        
        let car = self.s_expression()?;
        self.skip_past_end()?;
        
        // Check for dot
        if !self.peek_token().map(|t| matches!(t, Lexer::Dot(_))).unwrap_or(false) {
            return Err("Not a dotted pair".to_string());
        }
        self.index += 1; // consume dot
        self.skip_past_end()?;
        
        let cdr = self.s_expression()?;
        self.skip_past_end()?;
        
        self.rpar()?;
        
        Ok(LispCst::DottedPair(Box::new(Spanned::new((car, cdr)))))
    }

    /// list: '(' s_expression+ ')'
    fn list(&mut self) -> Result<LispCst, String> {
        let mut elements = Vec::new();
        
        self.lpar()?;
        self.skip_past_end()?;
        
        while self.peek_token().map(|t| !matches!(t, Lexer::RPar(_))).unwrap_or(false) {
            elements.push(self.s_expression()?);
            self.skip_past_end()?;
        }
        
        if elements.is_empty() {
            return Err("List must have at least one element".to_string());
        }
        
        self.rpar()?;
        Ok(LispCst::List(Spanned::new(elements)))
    }

    fn atom_or_number(&mut self) -> Result<LispCst, String> {
        self.skip_past_end()?;
        match self.tokens.get(self.index).cloned() {
            Some(Lexer::Atom(s)) => {
                self.index += 1;
                Ok(LispCst::Atom(s))
            }
            Some(Lexer::Number(n)) => {
                self.index += 1;
                Ok(LispCst::Number(n))
            }
            _ => Err("Expected atom or number".to_string()),
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

    fn skip_past_end(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn peek_token(&self) -> Option<&Lexer> {
        self.tokens.get(self.index)
    }
}
