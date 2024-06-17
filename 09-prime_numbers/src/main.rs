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
fn prime(limit: u32) -> i8 {
    if limit < 2 {
        println!("None");
        return -1;
    }
    println!("\t2");
    if limit == 2 {
        return 0;
    }
    let mut b;

    for i in (3..=limit).step_by(2) {
        b = true;
        if i % 2 != 0 {
            for j in (3..=(i as f64).sqrt() as u32).step_by(2) {
                if i % j == 0 {
                    //println!("{} is even", i);
                    b = false;
                    break;
                }
            }
            if b == true {
                println!("\t{}", i);
            }
        }
    }
    return 0;
}
fn main() {
    println!("Hello, world!");

    println!("Limit for Prime = ↓");
    prime(get_input());
}
