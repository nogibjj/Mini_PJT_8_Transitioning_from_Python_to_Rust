use clap::Parser; // Import the Parser trait from the clap crate to parse command-line arguments

// Define a struct to hold the command-line arguments
#[derive(Parser, Debug)] // Derive the Parser trait for Args to enable parsing and Debug for printing
#[clap(version = "1.0", about = "calculator")] // Metadata: version and description of the command-line tool
struct Args {
    #[clap(subcommand)]
    // This specifies that Args contains a subcommand (like add, multiply, etc.)
    command: Commands, // Store the subcommand in the command field
}

// Define an enum to list the supported subcommands
#[derive(Parser, Debug)] // Derive the Parser and Debug traits for Commands
enum Commands {
    Add { a: i64, b: i64 },
    Subtract { a: i64, b: i64 },
    Multiply { a: i64, b: i64 },
    Divide { a: i64, b: i64 },
}

fn main() {
    let args = Args::parse(); // Parse the command-line arguments into the Args struct

    // Match the parsed subcommand and execute the appropriate code for each case
    match args.command {
        Commands::Add { a, b } => println!("Sum: {}", a + b),
        Commands::Subtract { a, b } => println!("Subtraction: {}", a - b),
        Commands::Multiply { a, b } => println!("Product: {}", a * b),
        Commands::Divide { a, b } => {
            if b == 0 {
                println!("wrong");
            } else {
                println!("Division: {}", a / b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = Commands::Add { a: 2, b: 3 };
        match result {
            Commands::Add { a, b } => assert_eq!(a + b, 5), // 2 + 3 = 5
            _ => panic!("Error!"),
        }
    }

    #[test]
    fn test_subtract() {
        let result = Commands::Subtract { a: 5, b: 3 };
        match result {
            Commands::Subtract { a, b } => assert_eq!(a - b, 2), // 5 - 3 = 2
            _ => panic!("Error!"),
        }
    }

    #[test]
    fn test_multiply() {
        let result = Commands::Multiply { a: 4, b: 3 };
        match result {
            Commands::Multiply { a, b } => assert_eq!(a * b, 12), // 4 * 3 = 12
            _ => panic!("Error!"),
        }
    }

    #[test]
    fn test_divide() {
        let result = Commands::Divide { a: 6, b: 3 };
        match result {
            Commands::Divide { a, b } => assert_eq!(a / b, 2), // 6 / 3 = 2
            _ => panic!("Error!"),
        }
    }
}
