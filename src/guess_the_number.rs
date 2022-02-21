use rand::Rng;
use std::{cmp::Ordering, io};

pub fn guess_the_number() {
    loop {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1, 10);

        // println!("The secret number is: {}", secret_number);

        println!("Please input your guess. (Type \"quit\" to break)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "quit" {
                    break;
                }
                eprintln!("Please enter your guess as a number [1-10].");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }
}
