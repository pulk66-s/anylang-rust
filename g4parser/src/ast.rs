#[derive(Debug)]
pub enum G4Ast {
    Expr(String, Box<G4Ast>),
    Or(Vec<G4Ast>),
    And(Vec<G4Ast>),
    Many(Box<G4Ast>),
    Terminal(String)
}

#[derive(Debug)]
pub struct G4Grammar {
    name: String,
    exprs: Vec<G4Ast>
}

impl G4Grammar {
    pub fn new(name: String, exprs: Vec<G4Ast>) -> Self {
        G4Grammar {
            name,
            exprs
        }
    }
}