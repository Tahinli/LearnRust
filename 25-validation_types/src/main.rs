use validation_types::TahinliWord;

fn main() {
    println!("Hello, world!");

    // This will create new TahinliWord type
    let mut tahinli_word = TahinliWord::new("Tahinli".to_string());
    // This will print like normal string
    println!("{}", tahinli_word.get());

    // Borrow checker not allow to manipulate it
    // It's read only value
    // let mut try_to_manipulate = tahinli_word.get();
    // try_to_manipulate.clear();

    // This will try to bypass Tahinli Word rules and panic
    tahinli_word.set("This is not a Tahinli Word".to_string());
    println!("Code won't run this line :/");
}
