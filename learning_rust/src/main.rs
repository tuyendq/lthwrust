// Import the environment module from the standard library
use std::env;

fn main() {
    println!("System programming using Rust!");

    // Get computer name from environment variables
    let computer_name = env::var("COMPUTERNAME")
        .unwrap_or_else(|_| "Undefined".to_string());

    // Get current working dir
    let current_dir = env::current_dir().expect("Cannot access dir.");

    println!("Your computer name: {}", computer_name);
    println!("Your current working dir: {:?}", current_dir);
}
