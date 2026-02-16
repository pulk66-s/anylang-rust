pub struct Parser;

use parse_tools::{CharTools, Iter, LogicTools, StringTools};

use crate::ast::{G4Ast, G4Grammar};

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    fn parse_grammar_name(&self, txt: &mut Iter) -> Result<String, String> {
        CharTools::parse_spaces(txt)?;
        StringTools::starts(txt, "grammar")?;
        CharTools::parse_spaces(txt)?;
        let identifier = StringTools::identifier(txt)?;
        CharTools::parse_spaces(txt)?;
        CharTools::parse_semi_colon(txt)?;
        Ok(identifier)
    }

    fn parse_and(&self, txt: &mut Iter) -> Result<G4Ast, String> {
        let mut res = Vec::new();

        loop {
            match self.parse_expr(txt) {
                Ok(r) => res.push(r),
                _ => break,
            };
        }
        Ok(G4Ast::And(res))
    }

    fn parse_or(&self, txt: &mut Iter) -> Result<G4Ast, String> {
        CharTools::parse_spaces(txt)?;
        let mut res = Vec::new();

        loop {
            match self.parse_expr(txt) {
                Ok(r) => res.push(r),
                _ => break,
            };
            CharTools::parse_spaces(txt)?;
            if let Err(_) = CharTools::parse_pipe(txt) {
                break;
            }
            CharTools::parse_spaces(txt)?;
        }
        Ok(G4Ast::Or(res))
    }

    fn parse_char(&self, txt: &mut Iter) -> Result<G4Ast, String> {
        CharTools::parse_spaces(txt)?;
        let c = CharTools::parse_single_quote(txt)?;
        CharTools::parse_spaces(txt)?;
        Ok(G4Ast::Terminal(c.to_string()))
    }

    fn parse_terminal(&self, txt: &mut Iter) -> Result<G4Ast, String> {
        CharTools::parse_spaces(txt)?;
        let identifier = StringTools::identifier(txt).map(|s| G4Ast::Terminal(s))
            .or_else(|_| self.parse_char(txt))?;
        Ok(identifier)
    }

    fn parse_expr(&self, txt: &mut Iter) -> Result<G4Ast, String> {
        CharTools::parse_spaces(txt)?;
        let res = self.parse_or(txt)
            .or_else(|_| self.parse_and(txt))
            .or_else(|_| self.parse_terminal(txt))?;
        CharTools::parse_spaces(txt)?;
        Ok(res)
    }

    fn parse_expr_def(&self, txt: &mut Iter) -> Result<G4Ast, String> {
        println!("1 {}", txt.to_string());
        CharTools::parse_spaces(txt)?;
        println!("2 {}", txt.to_string());
        let identifier = StringTools::identifier(txt)?;
        println!("3 {}", identifier);
        CharTools::parse_spaces(txt)?;
        println!("4 {}", txt.to_string());
        CharTools::parse_double_dot(txt)?;
        println!("5");
        CharTools::parse_spaces(txt)?;
        println!("'{}'", txt.to_string());
        let expr = self.parse_expr(txt)?;
        CharTools::parse_spaces(txt)?;
        CharTools::parse_semi_colon(txt)?;
        Ok(G4Ast::Expr(identifier, Box::new(expr)))
    }

    fn parse_many_expr(&self, txt: &mut Iter) -> Result<Vec<G4Ast>, String> {
        let mut res = Vec::new();

        loop {
            match self.parse_expr_def(txt) {
                Ok(r) => res.push(r),
                _ => break,
            };
        }
        Ok(res)
    }

    pub fn parse(&self, txt: String) -> Result<G4Grammar, String> {
        let mut iter = Iter::from_string(&txt);
        let grammar_name = self.parse_grammar_name(&mut iter)?;

        let exprs = self.parse_many_expr(&mut iter)?;
        Ok(G4Grammar::new(grammar_name, exprs))
    }
}
