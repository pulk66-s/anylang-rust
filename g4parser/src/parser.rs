use parse_tools::{CharTools, Iter, StringTools};
use crate::ast::{G4Ast, G4Grammar, G4Rule, G4RuleType, G4Quantifier};

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    /// Remove comments from input
    fn remove_comments(input: &str) -> String {
        let mut result = String::new();
        let mut in_line_comment = false;
        let mut in_block_comment = false;
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if in_line_comment {
                if chars[i] == '\n' {
                    result.push('\n');
                    in_line_comment = false;
                }
                i += 1;
            } else if in_block_comment {
                if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '/' {
                    in_block_comment = false;
                    i += 2;
                } else {
                    i += 1;
                }
            } else if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
                in_line_comment = true;
                i += 2;
            } else if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '*' {
                in_block_comment = true;
                i += 2;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
        result
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

    /// Parse a terminal string: 'text' or "text"
    fn parse_terminal_string(&self, txt: &mut Iter) -> Result<String, String> {
        CharTools::parse_spaces(txt)?;
        let quote = if txt.starts_with("'") {
            '\''
        } else if txt.starts_with("\"") {
            '"'
        } else {
            return Err("Expected quoted string".to_string());
        };

        txt.next(); // consume opening quote
        let mut terminal = String::new();
        
        loop {
            match txt.next() {
                Some('\\') => {
                    // Handle escape sequences
                    match txt.next() {
                        Some('n') => terminal.push('\n'),
                        Some('t') => terminal.push('\t'),
                        Some('r') => terminal.push('\r'),
                        Some(c) => terminal.push(c),
                        None => return Err("Unexpected end in string".to_string()),
                    }
                }
                Some(c) if c == quote => break,
                Some(c) => terminal.push(c),
                None => return Err("Unterminated string".to_string()),
            }
        }
        Ok(terminal)
    }

    /// Parse character class: [a-z], [0-9], etc
    fn parse_char_class(&self, txt: &mut Iter) -> Result<String, String> {
        CharTools::parse_spaces(txt)?;
        if !txt.starts_with("[") {
            return Err("Expected [".to_string());
        }
        txt.next(); // consume [

        let mut chars = String::from("[");
        loop {
            match txt.next() {
                Some(']') => {
                    chars.push(']');
                    break;
                }
                Some(c) => chars.push(c),
                None => return Err("Unterminated character class".to_string()),
            }
        }
        Ok(chars)
    }

    /// Parse a primary expression: terminal, reference, charclass, grouped expr, or EOF
    fn parse_primary(&self, txt: &mut Iter) -> Result<G4Ast, String> {
        CharTools::parse_spaces(txt)?;

        if txt.starts_with("EOF") {
            StringTools::starts(txt, "EOF")?;
            return Ok(G4Ast::Eof);
        }

        if txt.starts_with("'") || txt.starts_with("\"") {
            let terminal = self.parse_terminal_string(txt)?;
            return Ok(G4Ast::Terminal(terminal));
        }

        if txt.starts_with("[") {
            let charclass = self.parse_char_class(txt)?;
            return Ok(G4Ast::CharClass(charclass));
        }

        if txt.starts_with("(") {
            txt.next(); // consume (
            CharTools::parse_spaces(txt)?;
            let expr = self.parse_alternative(txt)?;
            CharTools::parse_spaces(txt)?;
            if !txt.starts_with(")") {
                return Err("Expected )".to_string());
            }
            txt.next(); // consume )
            return Ok(G4Ast::Group(Box::new(expr)));
        }

        // Parse identifier (rule name or reference)
        let ident = StringTools::identifier(txt)?;
        Ok(G4Ast::Reference(ident))
    }

    /// Parse quantified expression: expr, expr+, expr*, expr?
    fn parse_quantified(&self, txt: &mut Iter) -> Result<G4Ast, String> {
        let mut expr = self.parse_primary(txt)?;
        CharTools::parse_spaces(txt)?;

        loop {
            let quantifier = if txt.starts_with("+") {
                txt.next();
                Some(G4Quantifier::OneOrMore)
            } else if txt.starts_with("*") {
                txt.next();
                Some(G4Quantifier::ZeroOrMore)
            } else if txt.starts_with("?") {
                txt.next();
                Some(G4Quantifier::Optional)
            } else {
                None
            };

            if let Some(q) = quantifier {
                expr = G4Ast::Quantified {
                    expr: Box::new(expr),
                    quantifier: q,
                };
                CharTools::parse_spaces(txt)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parse sequence: expr1 expr2 expr3 (without | or ;)
    fn parse_sequence(&self, txt: &mut Iter) -> Result<G4Ast, String> {
        CharTools::parse_spaces(txt)?;
        let mut exprs = Vec::new();

        loop {
            // Stop if we see |, ;, ), or end of input
            if txt.starts_with("|") || txt.starts_with(";") || txt.starts_with(")") || 
               txt.is_empty() || txt.starts_with("->") {
                break;
            }

            match self.parse_quantified(txt) {
                Ok(expr) => {
                    exprs.push(expr);
                    CharTools::parse_spaces(txt)?;
                }
                Err(_) => break,
            }
        }

        if exprs.is_empty() {
            Err("Expected expression".to_string())
        } else if exprs.len() == 1 {
            Ok(exprs.into_iter().next().unwrap())
        } else {
            Ok(G4Ast::Sequence(exprs))
        }
    }

    /// Parse alternative: expr1 | expr2 | expr3
    fn parse_alternative(&self, txt: &mut Iter) -> Result<G4Ast, String> {
        CharTools::parse_spaces(txt)?;
        let mut exprs = vec![self.parse_sequence(txt)?];

        loop {
            CharTools::parse_spaces(txt)?;
            if !txt.starts_with("|") {
                break;
            }
            txt.next(); // consume |
            CharTools::parse_spaces(txt)?;

            exprs.push(self.parse_sequence(txt)?);
        }

        if exprs.len() == 1 {
            Ok(exprs.into_iter().next().unwrap())
        } else {
            Ok(G4Ast::Alternative(exprs))
        }
    }

    /// Parse action: -> skip, -> channel(HIDDEN), etc
    fn parse_action(&self, txt: &mut Iter) -> Result<Option<G4Ast>, String> {
        CharTools::parse_spaces(txt)?;
        if !txt.starts_with("->") {
            return Ok(None);
        }
        txt.next();
        txt.next(); // consume ->
        CharTools::parse_spaces(txt)?;

        let action = StringTools::identifier(txt)?;
        Ok(Some(G4Ast::Action(action)))
    }

    /// Parse a complete rule: [fragment] RULE_NAME : expr ;
    fn parse_rule(&self, txt: &mut Iter) -> Result<G4Rule, String> {
        CharTools::parse_spaces(txt)?;

        // Check for "fragment" keyword
        let rule_type = if txt.starts_with("fragment") {
            StringTools::starts(txt, "fragment")?;
            CharTools::parse_spaces(txt)?;
            G4RuleType::Fragment
        } else {
            G4RuleType::Normal
        };

        let rule_name = StringTools::identifier(txt)?;
        CharTools::parse_spaces(txt)?;
        CharTools::parse_double_dot(txt)?;
        CharTools::parse_spaces(txt)?;

        let expr = self.parse_alternative(txt)?;
        CharTools::parse_spaces(txt)?;

        // Check for action
        if let Ok(Some(_action)) = self.parse_action(txt) {
            CharTools::parse_spaces(txt)?;
            // TODO: attach action to expr
        }

        CharTools::parse_semi_colon(txt)?;

        Ok(G4Rule {
            name: rule_name,
            rule_type,
            expr,
        })
    }

    /// Parse all rules in the grammar
    fn parse_rules(&self, txt: &mut Iter) -> Result<Vec<G4Rule>, String> {
        let mut rules = Vec::new();

        loop {
            CharTools::parse_spaces(txt)?;
            if txt.is_empty() {
                break;
            }

            match self.parse_rule(txt) {
                Ok(rule) => rules.push(rule),
                Err(e) => {
                    // Try to skip to next rule on error
                    while !txt.is_empty() && !txt.starts_with(";") {
                        txt.next();
                    }
                    if !txt.is_empty() {
                        txt.next(); // skip the ;
                    }
                }
            }
        }

        Ok(rules)
    }

    /// Parse a complete ANTLR4 grammar file
    pub fn parse(&self, input: String) -> Result<G4Grammar, String> {
        let cleaned = Self::remove_comments(&input);
        let mut iter = Iter::from_string(&cleaned);

        let grammar_name = self.parse_grammar_name(&mut iter)?;
        let rules = self.parse_rules(&mut iter)?;

        Ok(G4Grammar::new(grammar_name, rules))
    }
}
