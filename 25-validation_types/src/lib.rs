// This is our type to enforce some rules
// Since struct itself is public and word inside of it private
// we're able to create TahinliWord everywhere but
// can't modify it's value (word) outside of this module
// This will allow us to apply some rules
pub struct TahinliWord {
    word: String,
}

impl TahinliWord {
    // This's basically a validator that enforces TahinliWord rules
    // Since this is private I'm only using it in this module for clean code
    fn validator(word: &String) -> bool {
        match word.as_str() {
            "Tahinli" | "tahinli" | "TAHINLI" => true,
            _ => false,
        }
    }

    // This is one and only function that allow user to create new TahinliWord
    // Thanks to this situation we're able to check whatever we want
    pub fn new(word: String) -> Self {
        TahinliWord::validator(&word);
        TahinliWord { word }
    }

    // This is a getter method that only exposes it's value
    pub fn get(&self) -> &String {
        &self.word
    }

    // This is a setter method that's the only way to edit existing TahinliWord
    // This gives us power to do whatever check we want
    pub fn set(&mut self, word: String) {
        if TahinliWord::validator(&word) {
            self.word = word;
        } else {
            panic!("\"{}\" is not a Tahinli Word", word);
        }
    }
}
