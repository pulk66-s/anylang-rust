use crate::span::{Span, Spanned};

#[derive(Debug, Clone)]
pub enum Lexer {
    Atom(Spanned<String>),
    Number(Spanned<f64>),
    LPar(Span),
    RPar(Span),
    Dot(Span),
}

pub struct LexerParser<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
    index: usize,
}

impl<'a> LexerParser<'a> {
    pub fn new(input: &'a str) -> Self {
        LexerParser { input: input.chars().peekable(), index: 0 }
    }

    pub fn parse(input: &'a str) -> Result<Vec<Lexer>, String> {
        let mut parser = Self::new(input);
        parser.run()
    }

    fn run(&mut self) -> Result<Vec<Lexer>, String> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();
            match self.parse_token()? {
                Some(token) => tokens.push(token),
                None => break,
            }
        }
        Ok(tokens)
    }

    fn new_span(&mut self, size: usize) -> Span {
        let start = self.index;
        let end = self.index + size;

        self.index += size;
        Span { start, end }
    }

    fn parse_token(&mut self) -> Result<Option<Lexer>, String> {
        match self.input.peek() {
            None => Ok(None),
            Some(&'(') => Ok(Some(self.parse_lpar()?)),
            Some(&')') => Ok(Some(self.parse_rpar()?)),
            Some(&'.') => {
                // Check if this is a dot token (followed by whitespace/paren) or part of a number
                let mut temp_input = self.input.clone();
                temp_input.next(); // skip the dot
                match temp_input.peek() {
                    Some(&c) if c.is_whitespace() || c == '(' || c == ')' => {
                        Ok(Some(self.parse_dot()?))
                    }
                    _ => Ok(Some(self.parse_atom()?))
                }
            }
            Some(&c) if c.is_digit(10) => {
                Ok(Some(self.parse_number()?))
            }
            Some(_) => Ok(Some(self.parse_atom()?)),
        }
    }

    fn parse_lpar(&mut self) -> Result<Lexer, String> {
        match self.input.next_if(|&c| c == '(') {
            Some(_) => Ok(Lexer::LPar(self.new_span(1))),
            None => Err("Expected '('".to_string()),
        }
    }

    fn parse_dot(&mut self) -> Result<Lexer, String> {
        match self.input.next_if(|&c| c == '.') {
            Some(_) => Ok(Lexer::Dot(self.new_span(1))),
            None => Err("Expected '.'".to_string()),
        }
    }

    fn parse_rpar(&mut self) -> Result<Lexer, String> {
        match self.input.next_if(|&c| c == ')') {
            Some(_) => Ok(Lexer::RPar(self.new_span(1))),
            None => Err("Expected ')'".to_string()),
        }
    }

    fn parse_number(&mut self) -> Result<Lexer, String> {
        let start = self.index;
        let mut str = String::new();

        if let Some(&'-') = self.input.peek() {
            str.push('-');
            self.input.next();
            self.index += 1;
        }
        while let Some(&c) = self.input.peek() {
            if c.is_digit(10) || c == '.' {
                str.push(c);
                self.input.next();
                self.index += 1;
            } else {
                break;
            }
        }
        if str.is_empty() {
            Err("Expected number".to_string())
        } else {
            match str.parse::<f64>() {
                Ok(num) => Ok(Lexer::Number(Spanned { 
                    value: num, 
                    span: Span { start, end: self.index } 
                })),
                Err(_) => Err(format!("Invalid number: {}", str)),
            }
        }
    }

    fn parse_atom(&mut self) -> Result<Lexer, String> {
        let start = self.index;
        let mut str = String::new();

        while let Some(&c) = self.input.peek() {
            if !c.is_whitespace() && c != '(' && c != ')' {
                str.push(c);
                self.input.next();
                self.index += 1;
            } else {
                break;
            }
        }
        
        if str.is_empty() {
            Err("Expected atom".to_string())
        } else {
            Ok(Lexer::Atom(Spanned { 
                value: str, 
                span: Span { start, end: self.index } 
            }))
        }
    }

    fn skip_whitespace(&mut self) {
        while self.input.next_if(|c| c.is_whitespace()).is_some() {
            self.index += 1;
        }
    }
}
