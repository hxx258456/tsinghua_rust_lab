use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    println!("Please enter a number.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
