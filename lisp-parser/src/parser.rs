use crate::{TaggedAstParser, ast::TaggedAst};

pub struct LispParser;

impl LispParser {
    pub fn new() -> Self {
        LispParser
    }

    pub fn parse(&self, input: &str) -> Result<Vec<TaggedAst>, String> {
        let mut ast_parser = TaggedAstParser::new(input)?;

        ast_parser.parse()
    }
}
