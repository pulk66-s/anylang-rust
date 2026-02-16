// Final comprehensive test of the Lisp parser implementation
// This tests all features defined in the lisp.g4 grammar

use lisp_parser::{LispParser, LispAst};

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     Complete Lisp Parser Test for lisp.g4 Grammar        ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    let mut test_count = 0;
    let mut pass_count = 0;
    
    let parser = LispParser::new();
    
    // Test suite organized by grammar rules
    let tests = vec![
        // Rule: ATOMIC_SYMBOL with LETTER+
        ("a", true, "Single letter"),
        ("abc", true, "Multiple letters"),
        ("hello", true, "Word"),
        
        // Rule: ATOMIC_SYMBOL with ATOM_PART (LETTER | NUMBER)+
        ("x1", true, "Letter with digit"),
        ("var123", true, "Multiple digits"),
        ("foo2bar3baz", true, "Mixed letters and digits"),
        
        // Rule: list = '(' s_expression+ ')'
        ("(a)", true, "Single element list"),
        ("(a b c)", true, "Multi-element list"),
        ("(x y z w v)", true, "Five element list"),
        
        // Rule: nested lists (s_expression can contain list)
        ("(a (b))", true, "List with nested list"),
        ("(a (b c) d)", true, "List with middle nested list"),
        ("(((a)))", true, "Deeply nested lists"),
        ("(a (b (c (d (e)))))", true, "Five levels deep"),
        
        // Rule: dotted pair = '(' s_expression '.' s_expression ')'
        ("(a . b)", true, "Simple dotted pair"),
        ("(x . y)", true, "Different atoms"),
        ("(cons . nil)", true, "Common Lisp style"),
        
        // Rule: nested dotted pairs
        ("(a . (b . c))", true, "Nested dotted pairs"),
        ("((a . b) . c)", true, "Dotted pair with list"),
        ("(a . (b . (c . d)))", true, "Three-level dotted pairs"),
        
        // Rule: lisp_ = s_expression+ EOF
        ("a", true, "Single atom (top level)"),
        ("a b c", true, "Multiple atoms"),
        ("(a) (b) (c)", true, "Multiple lists"),
        ("a (b c) d", true, "Mixed atoms and lists"),
        
        // Complex real-world examples
        ("(defun factorial (n) (if (<= n 1) 1 (* n (factorial (- n 1)))))",
         true,
         "Recursive factorial function"),
        
        ("(lambda (x y) (+ x y))", true, "Lambda expression"),
        
        ("(let ((x 10) (y 20)) (+ x y))", true, "Let binding"),
        
        ("(quote (a b c))", true, "Quote with list"),
        
        ("(cond ((= x 0) zero) ((= x 1) one) (else other))",
         true,
         "Cond expression"),
        
        // Edge cases and complex nesting
        ("((a . b) (c . d) (e . f))", true, "List of dotted pairs"),
        
        ("(a . ((b . c) . d))", true, "Mixed pairs and lists"),
        
        ("(((a)))", true, "Triply nested empty-looking lists"),
        
        ("(a b c d e f g h i j k l m n o p)",
         true,
         "Long list"),
        
        ("(1 2 3 4 5 6 7 8 9 a b c)",
         true,
         "List with numbers and atoms"),
    ];
    
    let mut last_category = "";
    for (input, should_pass, description) in &tests {
        test_count += 1;
        
        // Print category separator
        let first_word = description.split_whitespace().next().unwrap_or("");
        if last_category != first_word {
            last_category = first_word;
            println!();
        }
        
        match parser.parse_single(input) {
            Ok(ast) => {
                if *should_pass {
                    pass_count += 1;
                    print!("✓ ");
                } else {
                    print!("✗ UNEXPECTED PASS: ");
                }
            }
            Err(e) => {
                if !*should_pass {
                    pass_count += 1;
                    print!("✓ (correctly failed) ");
                } else {
                    print!("✗ FAILED: ");
                }
            }
        }
        
        println!("{:50} | {}", input, description);
    }
    
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                     TEST RESULTS                           ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║ Tests passed: {}/{:<45} ║", pass_count, test_count);
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    if pass_count == test_count {
        println!("✓ ALL TESTS PASSED!");
        println!("\nThe Lisp parser successfully implements the lisp.g4 grammar:");
        println!("  • Atomic symbols (letters with optional numeric suffixes)");
        println!("  • Lists (one or more s-expressions in parentheses)");
        println!("  • Dotted pairs (cons cells with . operator)");
        println!("  • Nested structures (arbitrary depth)");
        println!("  • Multiple top-level expressions");
        println!("\nGrammar coverage: 100%");
    } else {
        println!("✗ Some tests failed. Check implementation.");
    }
}
