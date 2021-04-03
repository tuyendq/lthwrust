fn main() {
    println!("Hello, world!");
    let mut array = ['a', 'b', 'c'];
    array[0] = 'x';
    println!("First element is: {}", array[0]);

    let numbers: [i32; 5];
    numbers = [0; 5];
    println!("Last element {}", numbers[4]);

    let index: usize = numbers.len();
    println!("Last element is {}", numbers[index]);

}
