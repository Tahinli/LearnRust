enum Types {
    Number(i32),
    String(String),
    Float(f32),
}
fn main() {
    println!("Hello, world!");
    // Actually no need to specify type but I did
    let mut numbers: Vec<i32> = Vec::new();

    // This adds element to the end of vector
    numbers.push(0);
    numbers.push(1);

    for _ in 0..3 {
        // This tries to remove last element from vector
        match numbers.pop() {
            Some(popped) => println!("{}", popped),
            None => eprintln!("Error: Vector is Empty"),
        }
    }

    // This is macro for vector
    let texts = vec!["Hello", "My", "Friend"];

    // Iteration over vector
    for text in texts {
        println!("{}", text);
    }

    // This won't work because of borrow checker
    // it prevents us to modify vector in iteration.

    // let mut texts = vec!["Hello", "My", "Friend"];
    // for text in texts {
    //     texts.push("Let me try");
    // }

    // This may seem like keeping different types
    // actually they are belongs to same enum

    let mut different_types = vec![];
    different_types.push(Types::String(String::from("Hi!")));
    different_types.push(Types::Number(1));
    different_types.push(Types::Float(1.1));

    for element in &different_types {
        match element {
            Types::Number(number) => println!("{}", number),
            Types::String(string) => println!("{}", string),
            Types::Float(float) => println!("{}", float),
        }
    }
}
