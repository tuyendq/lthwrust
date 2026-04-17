fn main() {
    // println!("Hello, world!");
    greetings::hello();
    // greetings::goodbye();
}

mod greetings {
    pub fn hello() {
        println!("Hello, public modules!");
    }

    #[allow(dead_code)]
    fn goodbye() {
        println!("Goodbye, private modules!");
    }
}