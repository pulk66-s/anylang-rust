use lisp_logic::Logic;

fn main() {
    let logic = Logic::new();
    
    // Test: Parse Lisp code by converting to JSON first
    let lisp_code = "(defun simple () (+ 1 2))";
    println!("Testing with Lisp code: {}", lisp_code);
    
    // Convert Lisp to JSON manually (simulating what FFI does)
    let mut parser = lisp_parser::ast::TaggedAstParser::new(lisp_code).unwrap();
    let ast = parser.parse().unwrap();
    let json_str = serde_json::to_string_pretty(&ast).unwrap();
    
    println!("\nGenerated JSON:");
    println!("{}", json_str);
    
    // Parse the JSON with Logic
    match logic.parse(&json_str) {
        Ok(result) => println!("\nLogic processing result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
