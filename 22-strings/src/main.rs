fn main() {
    println!("Hello, world!");

    // This is how we can create a string.
    let empty_string = String::new();
    println!("{}", empty_string);

    // This is how we can create string with initial value
    let mut how_are_you = String::from("Hi");
    // This appends string to the string
    how_are_you.push_str(", How are you ");
    // This appends a char to the string
    how_are_you.push('?');
    println!("{}", how_are_you);

    // to_string is also equivalent for String::from
    let drink = "Drink".to_string();
    let water = "Water".to_string();
    let drink_water = drink + &water;

    println!("{}", drink_water);

    // This won't work since '+' operation takes ownership of first
    // println!("{}", drink);
    // But this works
    println!("{}", water);

    let stay = "stay".to_string();
    let safe = "safe".to_string();

    // This all going to work because format doesn't take ownerships
    let stay_safe = format!("{}{}", stay, safe);
    println!("{}", stay_safe);
    println!("{}", stay);
    println!("{}", safe);

    // This doesn't work because string indexes can't be reached like this.
    // Because non ascii characters may take more than one slot.
    // Rust prevents potential errors in the first place
    // println!("{}", stay_safe[0]);

    for char in stay_safe.chars() {
        println!("{}", char);
    }

    for byte in stay_safe.bytes() {
        println!("{}", byte);
    }
}
