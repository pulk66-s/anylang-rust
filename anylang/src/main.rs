mod backend_loader;
mod cli;
mod file_io;
mod logic_loader;
mod option;
mod output;
mod parser_loader;

use backend_loader::{Backend, DynamicBackend};
use cli::print_usage;
use file_io::read_file;
use logic_loader::{DynamicLogic, Logic};
use output::write_output;
use parser_loader::{DynamicParser, Parser};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || option::contains_help_flag(&args[1..]) {
        print_usage(&args[0]);
        return;
    }

    let options = match option::parse_args(&args[1..]) {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            print_usage(&args[0]);
            return;
        }
    };
    let parser_path = match options.parser {
        Some(ref path) => path,
        None => {
            eprintln!("Error: Parser library path is required (-p option)");
            print_usage(&args[0]);
            return;
        }
    };
    let input_file = match options.input {
        Some(ref path) => path,
        None => {
            eprintln!("Error: Input file is required");
            print_usage(&args[0]);
            return;
        }
    };
    let parser = match DynamicParser::load(parser_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to load parser: {}", e);
            return;
        }
    };

    match parser.name() {
        Ok(name) => {
            println!("Loaded parser: {}", name);
        }
        _ => {
            eprintln!("Warning: Could not retrieve parser information");
        }
    }

    let content = match read_file(input_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    println!("\nParsing file: {}", input_file);
    let ast_json = match parser.parse(&content) {
        Ok(ast) => {
            println!("Successfully parsed!");
            println!("\nAST (JSON):");
            println!("{}", ast);
            ast
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return;
        }
    };

    let ir_json = if let Some(logic_path) = &options.logic {
        println!("\n--- Logic Processing Phase ---");
        let logic = match DynamicLogic::load(logic_path) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to load logic library: {}", e);
                return;
            }
        };

        println!("Processing AST through logic layer...");
        match logic.parse(&ast_json) {
            Ok(ir) => {
                println!("Successfully processed!");
                println!("\nIR (JSON):");
                println!("{}", ir);
                ir
            }
            Err(e) => {
                eprintln!("Logic processing error: {}", e);
                return;
            }
        }
    } else {
        ast_json.clone()
    };

    if let Some(backend_path) = &options.backend {
        println!("\n--- Compilation Phase ---");
        let backend = match DynamicBackend::load(backend_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Failed to load backend: {}", e);
                return;
            }
        };

        match (backend.name(), backend.architectures()) {
            (Ok(name), Ok(archs)) => {
                println!("Loaded backend: {} (architectures: {:?})", name, archs);
            }
            _ => {
                eprintln!("Warning: Could not retrieve backend information");
            }
        }

        let output_path = options.output.as_deref().unwrap_or("a.out");

        println!("Compiling to: {}", output_path);
        match backend.compile(&ir_json, output_path) {
            Ok(()) => {
                println!("\n✓ Compilation successful!");
                println!("Run with: ./{}", output_path);
            }
            Err(e) => {
                eprintln!("Compilation error: {}", e);
            }
        }
    } else {
        if let Some(output_path) = &options.output {
            let output_data = if options.logic.is_some() { &ir_json } else { &ast_json };
            match write_output(output_path, output_data) {
                Ok(_) => {
                    let label = if options.logic.is_some() { "IR" } else { "AST" };
                    println!("\n{} written to: {}", label, output_path);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }
}
