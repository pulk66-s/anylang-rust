#[derive(Debug, Clone, PartialEq)]
pub enum G4Quantifier {
    ZeroOrMore,  // *
    OneOrMore,   // +
    Optional,    // ?
    None,        // default
}

#[derive(Debug, Clone, PartialEq)]
pub enum G4RuleType {
    Normal,
    Fragment,
    Lexer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum G4Ast {
    /// Named rule: name : expr ;
    Rule {
        name: String,
        rule_type: G4RuleType,
        expr: Box<G4Ast>,
    },
    /// Sequence of expressions: expr1 expr2
    Sequence(Vec<G4Ast>),
    /// Alternatives: expr1 | expr2 | expr3
    Alternative(Vec<G4Ast>),
    /// Terminal string: 'text' or "text"
    Terminal(String),
    /// Non-terminal identifier: RULE_NAME or rule_name
    Reference(String),
    /// Character class: [a-z], [0-9], etc
    CharClass(String),
    /// Grouped expression: (expr)
    Group(Box<G4Ast>),
    /// Quantified expression: expr+ or expr* or expr?
    Quantified {
        expr: Box<G4Ast>,
        quantifier: G4Quantifier,
    },
    /// Action: -> skip, -> channel(HIDDEN), etc
    Action(String),
    /// EOF marker
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G4Rule {
    pub name: String,
    pub rule_type: G4RuleType,
    pub expr: G4Ast,
}

#[derive(Debug)]
pub struct G4Grammar {
    pub name: String,
    pub rules: Vec<G4Rule>,
}

impl G4Grammar {
    pub fn new(name: String, rules: Vec<G4Rule>) -> Self {
        G4Grammar { name, rules }
    }

    pub fn add_rule(&mut self, rule: G4Rule) {
        self.rules.push(rule);
    }
}