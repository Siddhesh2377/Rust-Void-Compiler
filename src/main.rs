use regex::Regex;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Read the file contents into a string
    let contents = fs::read_to_string("Main.vd")?;

    // Regular expression to match function definitions
    let function_regex = Regex::new(r"fn\s+(\w+)\s*\([^)]*\)\s*\{([^}]*)\}").unwrap();

    // Create a vector to hold all functions
    let mut functions: Vec<Function> = Vec::new();

    // Iterate through all matches in the file contents
    for capture in function_regex.captures_iter(&contents) {
        let function_name = &capture[1];
        let function_body = &capture[2];
        functions.push(Function::new(function_name.to_string(), function_body.to_string()));
    }

    // Regular expression to match method/function calls
    let method_call_regex = Regex::new(r"(\w+)\s*\(([^)]*)\)").unwrap();

    // Create a vector to hold method calls
    let mut method_calls: Vec<MethodCall> = Vec::new();

    // Iterate over all functions to find method calls
    for function in &functions {
        for capture in method_call_regex.captures_iter(&function.body) {
            let method_name = &capture[1];
            let method_args = &capture[2];
            let args: Vec<Argument> = parse_arguments(method_args);
            method_calls.push(MethodCall::new(function.name.clone(), method_name.to_string(), args));
        }
    }

    // Print the extracted method calls
    for method in &method_calls {
        if method.parent_method == "main" {
            println!("Called Method: {}", method.name);
            for arg in &method.args {
                println!("Argument: {}, Type: {}", arg.value, arg.arg_type);
            }
            println!();
        }
    }

    Ok(())
}

// Define the Function struct
struct Function {
    name: String,
    body: String,
}

// Implement a constructor for Function
impl Function {
    fn new(name: String, body: String) -> Function {
        Function { name, body }
    }
}

// Define the MethodCall struct
struct MethodCall {
    parent_method: String,
    name: String,
    args: Vec<Argument>,
}

// Implement a constructor for MethodCall
impl MethodCall {
    fn new(parent_method: String, name: String, args: Vec<Argument>) -> MethodCall {
        MethodCall { parent_method, name, args }
    }
}

// Define the Argument struct
struct Argument {
    value: String,
    arg_type: String,
}

// Implement a constructor for Argument
impl Argument {
    fn new(value: String, arg_type: String) -> Argument {
        Argument { value, arg_type }
    }
}

// Function to parse arguments and determine their types
fn parse_arguments(arguments: &str) -> Vec<Argument> {
    let args: Vec<&str> = arguments.split(',').map(|s| s.trim()).collect();
    let mut parsed_args: Vec<Argument> = Vec::new();

    for arg in args {
        let arg_type = if arg.starts_with("\"") && arg.ends_with("\"") {
            "String"
        } else if arg.parse::<i64>().is_ok() {
            "Integer"
        } else if arg.parse::<f64>().is_ok() {
            "Float"
        } else if arg == "true" || arg == "false" {
            "Boolean"
        } else {
            "Unknown"
        };
        parsed_args.push(Argument::new(arg.to_string(), arg_type.to_string()));
    }

    parsed_args
}
