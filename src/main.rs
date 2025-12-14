use std::io::{self, Write};
use std::process;

fn main() {
    println!("=== IA Coding - Code Generator ===");
    println!("Generate Rust code from natural language descriptions");
    println!("Type 'quit' or 'exit' to leave\n");

    loop {
        print!("Enter your description (e.g., 'Create a for loop in Rust iterating from 0 to 10'): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                // Check for exit commands
                if input.is_empty() {
                    continue;
                }
                
                if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
                    println!("Goodbye!");
                    break;
                }

                // Process the input
                match generate_code(input) {
                    Ok(code) => {
                        println!("\n--- Generated Code ---");
                        println!("{}", code);
                        println!("--- End of Generated Code ---\n");
                    }
                    Err(e) => {
                        eprintln!("Error: {}\n", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read input: {}", e);
                process::exit(1);
            }
        }
    }
}

/// Generate code based on natural language description
/// 
/// # Arguments
/// * `description` - Natural language description of the code to generate
/// 
/// # Returns
/// * `Result<String, String>` - Generated code or error message
fn generate_code(description: &str) -> Result<String, String> {
    // Validate input
    if description.is_empty() {
        return Err("Description cannot be empty".to_string());
    }

    if description.len() < 5 {
        return Err("Description is too short. Please provide a more detailed description.".to_string());
    }

    // TODO: Integration with llama_cpp will be implemented here
    // For now, we'll provide a placeholder implementation that demonstrates
    // the structure and error handling
    
    // Since we need a model file to use llama_cpp, and it's not provided in the requirements,
    // we'll create a placeholder that explains how to integrate it
    Err(format!(
        "Model integration pending. To use llama_cpp:\n\
         1. Download a GGUF model file (e.g., from Hugging Face)\n\
         2. Place it in the project directory\n\
         3. Update the code to load the model\n\
         \n\
         Your description was: '{}'",
        description
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_code_empty_input() {
        let result = generate_code("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Description cannot be empty");
    }

    #[test]
    fn test_generate_code_short_input() {
        let result = generate_code("test");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too short"));
    }

    #[test]
    fn test_generate_code_valid_input() {
        let result = generate_code("Create a for loop in Rust");
        // Currently returns error as model is not loaded
        assert!(result.is_err());
    }
}
