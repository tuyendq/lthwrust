fn main() {
    println!("Hello, world!");

    // loop
    loop {
        println!("Loop forerver");
        break;
    }

    // while loop
    println!("while loop");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number -1;
    }


    // For loop
    println!("for loop");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
