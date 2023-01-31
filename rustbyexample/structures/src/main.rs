#![deny(clippy::all)]

struct Person {
    name: String,
    age: u8,
}

fn create_user(name: String, age: u8) -> Person {
    Person { name, age }
}

fn main() {
    println!("Hello, world!");
    let person = create_user("sidiq".to_string(), 15);

    println!(
        "My name is {} and i am {} years old",
        person.name, person.age
    )
}
