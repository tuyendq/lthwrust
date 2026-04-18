fn main() {
    // println!("Hello, world!");
    let s = String::from("Hello, world!");
    for c in s.chars() {
        print!("{} ", c);
    }

    println!();
    
    for b in s.bytes() {
        print!("{} ", b);
    }
}
