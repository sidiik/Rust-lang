#![deny(clippy::all)]

fn main() {
    // let _say_hello = |name: &str| format!("Hello, {}", name);
    // fn _say_hello(name: &mut str) -> String {
    //     format!("Hello, {}", name)
    // }

    // let mut name = String::from("John");

    // println!("{} ", _say_hello(&mut name))

    let multiply_by_2 = |x: f32| x * 2.0;
    let ptr = multiply_by_2;

    let y = 8f32;

    println!("4 * 2 = {}", ptr(y));
}
