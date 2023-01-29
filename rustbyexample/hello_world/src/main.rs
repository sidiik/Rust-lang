fn main() {
    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string

    println!("{0} this is {1}, {1} this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "The quick brown fox",
        verb = "jumbs over"
    );

    println!("Base 10 {:b}", 120);
    println!("Base 10 {:o}", 120);
    println!("Base 10 {:x}", 120);
    println!("{number:>5}", number = 1);
    println!("{number:*>5}", number = 1);

    #[derive(Debug)]
    struct Structure(i32);
    println!("Now {:?} is printable", Structure(3));

    #[derive(Debug)]
    struct Deep(Structure);

    println!("Now Deep is printable {:?}", Deep(Structure(7)));
}
