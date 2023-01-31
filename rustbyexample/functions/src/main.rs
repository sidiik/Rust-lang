#![deny(clippy::all)]

fn main() {
    // let _say_hello = |name: &str| format!("Hello, {}", name);
    fn _say_hello(name: &mut str) -> String {
        format!("Hello, {}", name)
    }

    let mut name = String::from("John");

    println!("{} ", _say_hello(&mut name))
}
