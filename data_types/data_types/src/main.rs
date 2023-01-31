fn main() {
    // let tup = ("Full stack coding bootcamp", 19);

    // let (course, students) = tup;

    // println!("{course} has {students}");

    // let langs = ["JS", "PY"];

    // println!("{} and {}", langs[0], langs[9])

    let sum = sum(3, 5);
    println!("The sum is {sum}")
}

fn sum(x: i32, y: i32) -> i32 {
    println!("The value of x is {x}");
    println!("The value of y is {y}");

    return x + y;
}
