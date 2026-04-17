fn main() {
    // println!("Hello, world!");
    greetings::hello();
    // greetings::goodbye();
}

mod greetings {
    pub fn hello() {
        println!("Hello, public modules!");
        good_morning();
        _good_night();
    }

    #[allow(dead_code)]
    fn goodbye() {
        println!("Goodbye, private modules!");
    }

    fn good_morning() {
        println!("Good morning, private modules!");
    }

    fn _good_night() {
        println!("Good night, private modules!");
    }
}