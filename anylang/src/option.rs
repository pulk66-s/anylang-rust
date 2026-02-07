pub struct Options {
    pub output: Option<String>,
    pub parser: Option<String>,
    pub logic: Option<String>,
    pub backend: Option<String>,
    pub input: Option<String>,
}

impl Options {
    pub fn new() -> Self {
        Self { 
            output: None,
            parser: None,
            logic: None,
            backend: None,
            input: None,
        }
    }
}

pub fn contains_help_flag(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "-h" || arg == "--help")
}

pub fn parse_args(args: &[String]) -> Result<Options, String> {
    let mut options = Options::new();
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    options.output = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Expected output file after -o".to_string());
                }
            }
            "-p" | "--parser" => {
                if i + 1 < args.len() {
                    options.parser = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Expected parser library path after -p".to_string());
                }
            }
            "-l" | "--logic" => {
                if i + 1 < args.len() {
                    options.logic = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Expected logic library path after -l".to_string());
                }
            }
            "-b" | "--backend" => {
                if i + 1 < args.len() {
                    options.backend = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Expected backend library path after -b".to_string());
                }
            }
            arg => {
                if !arg.starts_with('-') && options.input.is_none() {
                    options.input = Some(arg.to_string());
                }
                i += 1;
            }
        }
    }
    Ok(options)
}
