use rand::Rng;
use std::io;

fn main() {
    println!("Please guess a number");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=2);

    println!("The secret number is {secret_number}");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read your input.");

    println!("You guessed {guess}")
}
