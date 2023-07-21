#![feature(core_intrinsics)]
use rand::Rng;
use std::{cmp::Ordering, io};

#[warn(unsafe_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
    println!("Guess the number!");
    println!("Please enter a number.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        print_type_of(&guess); // <<--------为什么这里变成guess String类型

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(a) => {
                println!("{a}");
                continue;
            }
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
        print_type_of(&guess); // <<--------这里guess u32类型
    }
}
