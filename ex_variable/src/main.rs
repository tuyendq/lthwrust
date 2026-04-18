fn main() {
    let x: i32 = 5;  // immutable variable
    println!("Value of x is: {}", x);
    // x = 10; // This will cause a compile-time error because x is immutable
    let x = 10; // shadowing the previous variable x
    println!("Value of x is: {}", x);
}
