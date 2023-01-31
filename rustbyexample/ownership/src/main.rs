#![deny(clippy::all)]

fn main() {
    let name = String::from("Sidiiq");
    let name2 = &name;
    greet(&name);
    greet(&name2);

    let mut foo = String::from("foo");

    empty_string(&mut foo);
}

fn greet(name: &String) {
    println!("Hello, {name}");
}

fn empty_string(value: &mut String) {
    value.clear();
    println!("foo {}", value)
}
