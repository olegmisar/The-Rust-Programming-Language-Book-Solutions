extern crate rand;

use rand::Rng;

use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Starting a new game...");

        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("Guess a number:");

        loop {
            let guess: i32 = loop {
                let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");

                match guess.trim().parse() {
                    Ok(n) => break n,
                    Err(_) => println!("Please, type a whole number"),
                }
            };

            match guess.cmp(&secret_number) {
                Ordering::Greater => println!("Too big"),
                Ordering::Less => println!("Too small"),
                Ordering::Equal => {
                    println!("You win!");
                    println!();
                    break;
                }
            }
        }
    }
}
