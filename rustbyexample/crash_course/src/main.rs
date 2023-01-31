#![deny(clippy::all)]

const MY_AGE: u8 = 22;

fn main() {
    let firstName = "Sidiq Cumar";
    let user = (22, "John");

    let (age, name) = user;

    println!("{0} is {1} years old", name, age);
}
