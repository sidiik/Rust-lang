#![deny(clippy::all)]

enum AnimalType {
    Dog,
    Cat,
    Rabbit,
}

#[warn(dead_code)]

struct Size {
    width: f32,
    height: f32,
}

enum Shapes {
    Regtangle { width: f64, height: f64 },
    Square(f32, f32, Size),
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

    let square = Shapes::Square(
        3.2,
        4.5,
        Size {
            width: 4.5,
            height: 32.2,
        },
    );

    match square {
        Shapes::Square(x, y, Size { width, height }) => {
            println!("x {}, y {}, width {}, height {}", x, y, width, height)
        }

        _ => println!("This is not square"),
    }
}
