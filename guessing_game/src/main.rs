//Libs
use std::io;
use std::cmp::Ordering;
use rand::Rng;
//Funcs
fn main()
    {
        println!("Hello, world!\n");
        let secret_number = rand::thread_rng().gen_range(1..=10);
        println!("\t--> Guessing Game <--");
        println!("Secret Number = {secret_number}");
        loop
            {
                println!("Give a guess ↓ ");
                let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
                let guess: u32 = match
                    guess.trim()
                    .parse()
                        {
                            Ok(num) => num,
                            Err(_) => continue,
                        };
                println!("You Guessed = {guess}");
                match guess.cmp(&secret_number)
                    {
                        Ordering::Less => println!("Too small"),
                        Ordering::Greater => println!("Too big"),
                        Ordering::Equal =>
                            {
                                println!("You win");
                                break;
                            }
                    }
            }
    }
