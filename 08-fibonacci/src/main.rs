use std::io;
fn get_input() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Expected Valid Number");
            }
        };
    }
}
fn fibonacci(val: u32) -> u128 {
    let mut a = 0;
    let mut b = 1;
    let mut c;
    for _element in 2..=val {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}
fn main() {
    println!("Hello, world!");
    println!("Give a Number for Fibonacci Calculation ↓");
    println!("Result = {}", fibonacci(get_input()));
}
