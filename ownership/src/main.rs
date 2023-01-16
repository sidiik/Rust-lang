fn main() {
    let mut s = String::from("Hello world");
    let s1 = a(&mut s);

    println!("The length of {s} is {s1}");
    // let _s1 = &s;

    // println!("This is {s} {_s1}");

    // let x = 5;
    // let y = x;

    // println!("{y}")
}
fn a(some_str: &mut String) -> usize {
    some_str.push_str(" Again!!");
    let length = some_str.len();
    return length;
}
