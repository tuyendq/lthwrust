use std::env;

fn main() {
    println!("Hello, arguments!");

    if env::args().len() < 2 {
        println!("Usage: \nProgram requires at least 2 arguments!\n");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }
}
