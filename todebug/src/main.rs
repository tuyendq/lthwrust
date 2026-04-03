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

    let s1 = String::from("learn Rust");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
    
}