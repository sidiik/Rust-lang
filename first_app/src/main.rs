fn main() {
    // println!("Hello world");
    // println!("U+273F");
    // printing::announce_time();

    // struct secrets {
    //     x: i32,
    //     y: i32,
    // }

    // let x_and_y = secrets { x: 1, y: 2 };

    // println!("x is {} and y is {}", x_and_y.x, x_and_y.y)

    // let x_and_y = [12, -89, 45];

    // println!("{:?}", x_and_y)
    #[derive(Debug)]
    #[allow(dead_code)]

    enum Fruit {
        Apple,
        Banana,
    }

    type Food = Fruit;
    let fruit = Food::Banana;

    println!("{:?}", fruit);
}

mod printing {
    pub mod time_stuff;

    pub use time_stuff::print_date_time as announce_time;
}
