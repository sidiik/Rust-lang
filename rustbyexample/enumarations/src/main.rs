#![deny(clippy::all)]

enum AnimalType {
    Dog,
    Cat,
    Rabbit,
}

#[warn(dead_code)]

enum Shapes {
    Regtangle { width: f64, height: f64 },
}

fn main() {
    let kitty = AnimalType::Cat;

    match kitty {
        AnimalType::Cat => println!("Meow"),
        AnimalType::Dog => println!("Woof"),
        AnimalType::Rabbit => println!("Hoot"),
    }

    let rectangle = Shapes::Regtangle {
        width: 3.0,
        height: 3.4,
    };

    match rectangle {
        Shapes::Regtangle { width, height } => {
            println!("width  {} * height {} = {}", width, height, width * height)
        }
        _ => println!("Not a rectange"),
    }
}
