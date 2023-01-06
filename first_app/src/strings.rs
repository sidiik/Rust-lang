pub fn run() {

    let mut hello = String::from("Hello ");
    let mut name = String::from("sidiq");

    // Get the length of string

    println!("Length is: {}", hello.len());

    // Push char

    hello.push('W');

    // Push more string

    hello.push_str(" has been addded to the variable");


    // String capacity

    println!("Capacity is: {}", hello.capacity());

    // is empty

    println!("Is empty {}", hello.is_empty());

    // Replace

    
    println!("{}", name.replace("sidiq", "sidiiq"));

    

    println!("{}", hello);

    // loop through whitespace

    for word in hello.split_whitespace(){
        println!("{}",word)
    }



    println!("{}", hello)

}