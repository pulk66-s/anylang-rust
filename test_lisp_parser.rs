// Test program to demonstrate the Lisp parser implementation
// This tests the parser according to the lisp.g4 grammar

use lisp_parser::{LispParser, LispAst};

fn main() {
    println!("=== Lisp Parser Test Suite ===\n");
    
    // Test 1: Simple atoms
    test_parse("a", "Simple atom");
    test_parse("hello", "Identifier");
    test_parse("x1", "Atom with number");
    
    // Test 2: Lists
    test_parse("(a b c)", "Simple list");
    test_parse("()", "Empty list");
    test_parse("(a)", "Single element list");
    
    // Test 3: Nested lists
    test_parse("(a (b c) d)", "Nested list");
    test_parse("(a (b (c d)))", "Deeply nested list");
    
    // Test 4: Dotted pairs
    test_parse("(a . b)", "Simple dotted pair");
    test_parse("(a . (b . c))", "Nested dotted pair");
    test_parse("((a . b) . (c . d))", "List of dotted pairs");
    
    // Test 5: Complex expressions
    test_parse("(defun factorial (n) (if (<= n 1) 1 (* n (factorial (- n 1)))))", 
               "Recursive function definition");
    
    test_parse("(a 1 b 2 c 3)", "Atoms and numbers");
    
    // Test 6: Multiple expressions
    test_parse_multiple("(a b) (c d) (e f)", "Multiple expressions");
    
    println!("\n=== All tests completed ===");
}

fn test_parse(input: &str, description: &str) {
    println!("Test: {}", description);
    println!("Input:  {}", input);
    
    let parser = LispParser::new();
    match parser.parse_single(input) {
        Ok(ast) => {
            println!("Output: {}", ast.pretty_print());
            println!("AST:    {:?}", ast);
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    }
    println!();
}

fn test_parse_multiple(input: &str, description: &str) {
    println!("Test: {}", description);
    println!("Input:  {}", input);
    
    let parser = LispParser::new();
    match parser.parse_all(input) {
        Ok(asts) => {
            for (i, ast) in asts.iter().enumerate() {
                println!("  Expression {}: {}", i + 1, ast.pretty_print());
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    }
    println!();
}

/* Expected output when running this test:

=== Lisp Parser Test Suite ===

Test: Simple atom
Input:  a
Output: a
AST:    Atom("a")

Test: Identifier
Input:  hello
Output: hello
AST:    Atom("hello")

Test: Atom with number
Input:  x1
Output: x1
AST:    Atom("x1")

Test: Simple list
Input:  (a b c)
Output: (a b c)
AST:    List([Atom("a"), Atom("b"), Atom("c")])

... and so on ...

*/
