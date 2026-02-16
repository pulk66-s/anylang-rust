// Example demonstrating the Lisp parser for lisp.g4 grammar
// Run with: cargo run --example parse_lisp

use lisp_parser::LispParser;

fn main() {
    let parser = LispParser::new();
    
    println!("=== Lisp Parser Examples ===\n");
    
    // Example 1: Simple list
    let input1 = "(a b c)";
    println!("Input: {}", input1);
    match parser.parse_single(input1) {
        Ok(ast) => println!("Parsed: {}\n", ast.pretty_print()),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 2: Dotted pair
    let input2 = "(cons . (a b))";
    println!("Input: {}", input2);
    match parser.parse_single(input2) {
        Ok(ast) => println!("Parsed: {}\n", ast.pretty_print()),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 3: Nested lists
    let input3 = "(defun factorial (n) (if (<= n 1) 1 (* n (factorial (- n 1)))))";
    println!("Input: {}", input3);
    match parser.parse_single(input3) {
        Ok(ast) => println!("Parsed: {}\n", ast.pretty_print()),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 4: Multiple expressions
    let input4 = "(define x 10) (+ x 5)";
    println!("Input: {}", input4);
    match parser.parse_all(input4) {
        Ok(asts) => {
            for (i, ast) in asts.iter().enumerate() {
                println!("  Expression {}: {}", i + 1, ast.pretty_print());
            }
            println!();
        }
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 5: Complex nested expression
    let input5 = "((lambda (x) (+ x 1)) 5)";
    println!("Input: {}", input5);
    match parser.parse_single(input5) {
        Ok(ast) => println!("Parsed: {}\n", ast.pretty_print()),
        Err(e) => println!("Error: {}\n", e),
    }
}
