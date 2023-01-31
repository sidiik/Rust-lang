#![deny(clippy::all)]

struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]

struct Point(f64, f64, f64);

impl Point {
    fn describe(&self) {
        println!("This point contains {}, {}, {}", self.0, self.1, self.2)
    }

    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }

    fn zeros() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}

fn create_user(name: String, age: u8) -> Person {
    Person { name, age }
}

fn main() {
    println!("Hello, world!");
    let person = create_user("sidiq".to_string(), 15);
    let _person2 = Person {
        name: "Ahmed".to_string(),
        age: 34,
    };

    println!(
        "My name is {} and i am {} years old",
        person.name, person.age
    );

    let p = Point(1.0, 2.0, 3.3);

    p.describe();
    println!("The square of points are {:?}", p.twice());

    let p1 = Point::zeros();

    println!("{:?}", p1);
}
