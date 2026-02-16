// Example: Parsing the program.lisp file according to lisp.g4 grammar
// This demonstrates parsing a real Lisp file

use lisp_parser::LispParser;
use std::fs;

fn main() {
    println!("=== Lisp File Parser ===\n");
    
    let parser = LispParser::new();
    
    // Parse the program.lisp file
    match fs::read_to_string("program.lisp") {
        Ok(content) => {
            println!("File content:\n{}\n", content);
            println!("Parsing...\n");
            
            match parser.parse_all(&content) {
                Ok(asts) => {
                    println!("Successfully parsed {} s-expression(s):\n", asts.len());
                    
                    for (i, ast) in asts.iter().enumerate() {
                        println!("[{}] {}", i + 1, ast.pretty_print());
                    }
                    
                    println!("\n✓ File parsed successfully!");
                }
                Err(e) => {
                    println!("✗ Parse error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Could not read program.lisp: {}", e);
            println!("\nCreating example file for demonstration...");
            
            let example = r#"(define factorial
  (lambda (n)
    (if (= n 0)
        1
        (* n (factorial (- n 1))))))

(factorial 5)"#;
            
            let parser = LispParser::new();
            println!("\nExample Lisp code:\n{}\n", example);
            
            match parser.parse_all(example) {
                Ok(asts) => {
                    println!("Parsed {} s-expression(s):\n", asts.len());
                    for (i, ast) in asts.iter().enumerate() {
                        println!("[{}] {}", i + 1, ast.pretty_print());
                    }
                }
                Err(e) => {
                    println!("Parse error: {}", e);
                }
            }
        }
    }
}
