pub fn print_usage(program: &str) {
    eprintln!("Usage: {} [OPTIONS] <input-file>", program);
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -p, --parser <path>    Path to the parser library (.so file)");
    eprintln!("  -l, --logic <path>     Path to the logic library (.so file)");
    eprintln!("  -b, --backend <path>   Path to the backend library (.so file)");
    eprintln!("  -o, --output <path>    Output file path");
    eprintln!("  -h, --help             Show this help message");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  Parse only:");
    eprintln!("    {} -p target/debug/liblisp_parser.so input.lisp", program);
    eprintln!("  Parse with logic processing:");
    eprintln!("    {} -p target/debug/liblisp_parser.so -l target/debug/liblisp_logic.so input.lisp", program);
    eprintln!("  Parse, logic, and compile:");
    eprintln!(
        "    {} -p target/debug/liblisp_parser.so -l target/debug/liblisp_logic.so -b target/debug/libsimple_backend.so input.lisp -o program",
        program
    );
}
