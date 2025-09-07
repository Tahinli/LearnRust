use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // this is how we define hash map
    let mut hash_map = HashMap::new();
    // we can add key, value pairs like this
    hash_map.insert("Ahmet", 0);
    hash_map.insert("Kaan", 1);
    hash_map.insert("Gümüş", 2);

    // we can iterate pairs
    for hash in &hash_map {
        println!("{:#?}", hash);
    }

    // we're able to collect all keys with it
    println!("{:#?}", hash_map.keys());

    // this will return option(value)
    println!("{:#?}", hash_map.get("Ahmet"));

    // this will return option(key, value) pair
    println!("{:#?}", hash_map.get_key_value("Ahmet"));

    // inserting same key changes original value if exists
    hash_map.insert("Ahmet", 10);
    println!("{:#?}", hash_map.get_key_value("Ahmet"));

    // this allows us to insert a value if it haven't existed
    // If exists there will be no change
    println!("{}", hash_map.entry("Ahmet").or_insert(13));
    println!("{}", hash_map.entry("WhoIsThis").or_insert(13));
    println!("{}", hash_map.entry("WhoIsThisAgain").or_insert(13));

    // this tries to remove key and returns option(value)
    println!("{:#?}", hash_map.remove("WhoIsThis"));

    //this also removes but returns not only value but with key, option(key, value)
    println!("{:#?}", hash_map.remove_entry("WhoIsThisAgain"));
}
