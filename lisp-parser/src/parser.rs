use crate::ast::LispAst;

pub struct LispParser;

impl LispParser {
    pub fn new() -> Self {
        LispParser
    }

    fn skip_whitespace(input: &str, pos: usize) -> usize {
        let chars: Vec<char> = input.chars().collect();
        let mut i = pos;
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        i
    }

    fn parse_atom(input: &str, pos: usize) -> Result<(LispAst, usize), String> {
        let chars: Vec<char> = input.chars().collect();
        let mut i = pos;
        let mut atom = String::new();

        while i < chars.len() && !chars[i].is_whitespace() && chars[i] != '(' && chars[i] != ')' {
            atom.push(chars[i]);
            i += 1;
        }
        if atom.is_empty() {
            return Err("Expected atom".to_string());
        }
        if let Ok(num) = atom.parse::<f64>() {
            Ok((LispAst::Number(num), i))
        } else if atom == "nil" {
            Ok((LispAst::Nil, i))
        } else {
            Ok((LispAst::Atom(atom), i))
        }
    }

    fn parse_list(input: &str, pos: usize) -> Result<(LispAst, usize), String> {
        let chars: Vec<char> = input.chars().collect();
        let mut i = pos;

        if i >= chars.len() || chars[i] != '(' {
            return Err("Expected '('".to_string());
        }
        i += 1; // skip '('

        let mut elements = Vec::new();

        loop {
            i = Self::skip_whitespace(input, i);

            if i >= chars.len() {
                return Err("Unexpected end of input, expected ')'".to_string());
            }

            if chars[i] == ')' {
                i += 1; // skip ')'
                break;
            }

            let (expr, new_pos) = Self::parse_expr(input, i)?;
            elements.push(expr);
            i = new_pos;
        }
        Ok((LispAst::List(elements), i))
    }

    fn parse_expr(input: &str, pos: usize) -> Result<(LispAst, usize), String> {
        let chars: Vec<char> = input.chars().collect();
        let i = Self::skip_whitespace(input, pos);

        if i >= chars.len() {
            return Err("Unexpected end of input".to_string());
        }

        if chars[i] == '(' {
            Self::parse_list(input, i)
        } else {
            Self::parse_atom(input, i)
        }
    }

    pub fn parse(&self, input: &str) -> Result<LispAst, String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(LispAst::Nil);
        }

        let mut expressions = Vec::new();
        let mut pos = 0;

        loop {
            pos = Self::skip_whitespace(trimmed, pos);
            
            if pos >= trimmed.len() {
                break;
            }

            let (ast, new_pos) = Self::parse_expr(trimmed, pos)?;
            expressions.push(ast);
            pos = new_pos;
        }

        if expressions.is_empty() {
            Ok(LispAst::Nil)
        } else if expressions.len() == 1 {
            Ok(expressions.into_iter().next().unwrap())
        } else {
            // Multiple expressions - wrap them in a list
            Ok(LispAst::List(expressions))
        }
    }
}
