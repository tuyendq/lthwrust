use std::fs::File;

fn main() {
    // let f: u32 = File::open("hello.txt");
    // let f: u32 = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(_) => panic!("Couldn't open file."),
    }
}
