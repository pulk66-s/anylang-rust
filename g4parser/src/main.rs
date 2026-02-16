mod parser;
mod ast;

use parser::Parser;
use std::fs;

fn main() {
    let p = Parser::new();

    let contents = fs::read_to_string("../tests/lisp.g4")
        .expect("Failed to read tests/lisp.g4");

    println!("=== ANTLR4 Grammar Parser ===\n");
    println!("Parsing: tests/lisp.g4\n");

    match p.parse(contents) {
        Ok(grammar) => {
            println!("✓ Parsed successfully!\n");
            println!("Grammar Name: {}\n", grammar.name);
            println!("Rules ({}):", grammar.rules.len());
            println!("{}", "─".repeat(60));

            for (i, rule) in grammar.rules.iter().enumerate() {
                let rule_type = match rule.rule_type {
                    ast::G4RuleType::Fragment => "fragment ",
                    ast::G4RuleType::Lexer => "lexer ",
                    ast::G4RuleType::Normal => "",
                };
                println!("\n[{}] {}{}", i + 1, rule_type, rule.name);
                println!("  Expression: {:#?}", rule.expr);
            }
            println!("\n{}", "─".repeat(60));
            println!("\n✓ Grammar parsing completed successfully!");
        }
        Err(e) => {
            println!("✗ Error parsing: {}", e);
        }
    }
}
