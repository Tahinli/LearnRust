use std::io;
fn get_input() -> i32 {
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
fn main() {
    println!("Hello, world!");

    println!("1) Fahrenheit to Celsius\n2) Celsius to Fahrenheit");
    let mut selection;
    let mut b = true;
    while b {
        selection = get_input();
        match selection {
            1 => {
                println!("Fahrenheit Value = ↓");
                selection = get_input();
                println!("Celsius Value = {}", (selection - 32) as f32 * 5.0 / 9.0);
                b = false;
            }
            2 => {
                println!("Celsius Value = ↓");
                selection = get_input();
                println!(
                    "Fahrenheit Value = {}",
                    (selection as f32 * 9.0 / 5.0) + 32.0
                );
                b = false;
            }
            _ => {
                println!("Expected Valid Value");
            }
        }
    }
}
