use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        println!("The secret number is {secret_number}");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Equal => {
                println!("{}", "You win {}".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big".blue()),
        }
    }
}
