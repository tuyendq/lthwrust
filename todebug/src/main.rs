fn main() {
    // const PI: f64 = 3.14159;
    // let mut radius = 10;
    // let mut area = PI * (radius as f64) * (radius as f64);
    // println!("Bán kính: {}, diện tích: {}", radius, area);
    
    // // change radius
    // radius = 20;
    
    // area = PI * (radius as f64) * (radius as f64);
    // println!("Bán kính: {}, diện tích: {}", radius, area);

    // let x = 7;
    // let y = x;
    // println!("x: {}, y: {}", x, y);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // // println!("s1: {}", s1);  // Error: value borrowed here after move
    // println!("s2: {}", s2);

    // let s1 = String::from("learn Rust");
    // let s2 = s1.clone();
    // println!("s1: {}, s2: {}", s1, s2);

    // let s1 = String::from("Learn Rust is fun!");
    // print_string(&s1);
    // println!("s1 is: {}", s1);

    // ====
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    // ===

    // ===
    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    //     r1.push_str(" world");
    //     println!("{}", r1);
    // }
    // let r2 = &mut s;
    // r2.push_str("!");
    // println!("{}", r2);
    // ===

    // ===
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);
    // ===

    // ===
    // let mut s = String::from("hello");
    // let r1 = &s;  // borrow to read (immutable borrow)
    // let r2 = &s;  // borrow to read (immutable borrow)
    // let r3 = &mut s;
    // println!("{}, {}, {}", r1, r2, r3);  // Error: cannot borrow s as mutable because it is also borrowed as immutable
    // ===

    let s= String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

}

// fn print_string(s: &String) {
//     println!("Reading: {}", s);
// }