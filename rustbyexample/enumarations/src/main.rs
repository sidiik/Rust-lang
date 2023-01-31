#![deny(clippy::all)]
#![warn(dead_code)]

enum AnimalType {
    Dog,
    Cat,
    Rabbit,
}

fn main() {
    let kitty = AnimalType::Cat;

    match kitty {
        AnimalType::Cat => println!("Meow"),
        AnimalType::Dog => println!("Woof"),
        AnimalType::Rabbit => println!("Hoot"),
    }
}
