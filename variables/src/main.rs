fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("X = {x}");
    x = 6;
    println!("X = {x}");

    let shadowed = 5;
    let shadowed = shadowed + 2; //overshadow

    {
        let shadowed = shadowed + 13; //overshadow in scope
        println!("Shadowed in this scope is = {shadowed}");
    }
    println!("Shadowed in there is = {shadowed}");

    let str = "Hello There";
    println!("Str = {}", str);

    let str = str.len(); //overshadow with different type
    println!("Str = {}", str);
}
