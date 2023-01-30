#![deny(clippy::all)]

fn main() {
    let name = String::from("Sidiiq");
    let name2 = &name;
    greet(&name);
    greet(&name2);
}

fn greet(name: &String) {
    println!("Hello, {name}");
}
