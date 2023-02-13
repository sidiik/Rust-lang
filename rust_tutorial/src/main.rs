// fn main() {
//     let city_name = "Hargeisa";
//     println!("The city of {}", city_name);

//     print_population(1_324_578, 114_293, 108_0972)
// }

// fn print_population(adults: u64, kids: u32, buildings: u32) {
//     let population = adults + (kids as u64);

//     let buildings_per_person = (buildings as f64) / (population as f64);

//     println!("Population {}", population);
//     println!("Kids {}", kids);
//     println!("Adults {}", adults);
//     println!("Buildings {}", buildings);
//     println!("Buildings per person {}", buildings_per_person);

//     if buildings_per_person >= 1.0 {
//         println!("Everyone can have their own building");
//     } else {
//         println!("Building must be shared");
//     }
// }

// #[derive(Debug)]
// struct Point {
//     x: i64,
//     y: i64,
// }
// fn main() {
//     let values = new_points(&45, &23);

//     let Point { y, .. } = values;
//     println!("{}", y)
// }

// fn new_points(x: &i64, y: &i64) -> Point {
//     Point { x: *x, y: *y }
// }

use std::alloc::{dealloc, GlobalAlloc, Layout};

#[derive(Debug)]
struct City {
    residents: i64,
    description: String,
    is_coastal: bool,
}

enum Colors {
    Red,
    Green,
    Custom(u8, u8, u8),
}

enum CitySize {
    Town,
    City,
    Metrapolis,
    Area { residents: i64 },
}

impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> City {
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 10_000;
                (
                    format!("A town of approximately {} residents", residents),
                    residents,
                )
            }

            CitySize::City => {
                let residents = 1_000_000;
                (
                    format!("A city of approximately {} residents", residents),
                    residents,
                )
            }

            CitySize::Metrapolis => {
                let residents = 1_000;
                (
                    format!("A metrapolis of approximately {} residents", residents),
                    residents,
                )
            }

            CitySize::Area { residents } => (
                format!("A area of approximately {} residents", residents),
                residents,
            ),
        };

        City {
            description,
            is_coastal,
            residents,
        }
    }
}

fn main() {
    // let rustville = new_city(123, false);

    // let current_clr = Colors::Red;

    // let color_str = match current_clr {
    //     Colors::Red => println!("it was a red"),
    //     Colors::Green => println!("it was green"),
    //     _ => println!("it was something else"),
    // };

    // let mut email_str = "sidiikpro@gmail.com".to_string();

    // let last_char = &email_str.pop();

    // let hargeisa = City::new(CitySize::Area { residents: 5 }, false);

    // println!("{:?}", hargeisa)

    // let mut city_names = vec!["Hargeisa", "burao", "Erigavo", "Rustville"];

    // let last_city = match city_names.pop() {
    //     Some(inner_value) => inner_value,
    //     None => "",
    // };

    // if last_city.starts_with("R") {
    //     println!("{} starts with R", last_city)
    // } else {
    //     println!("{} does not start with R", last_city)
    // }

    // city_names.push(last_city);

    // for city in city_names.iter() {
    //     println!("* {city}")
    // }

    // let orders = vec![1, 2, 3, 4];

    let nums = vec![10, 23, 4, 5, 3, 4, 6, 2, 4, 43, 8];

    let sum_of_nums = sum(nums.clone());
    let product_of_nums = product(nums.clone());
    let average = average(nums);

    //

    println!("Sum of numbers is {sum_of_nums}");
    println!("Product of numbers is {product_of_nums}");
    println!("Average of numbers is {average}");
}

fn new_city(residents: i64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("This is coastal city with {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        City {
            description: format!("This is non-coastal city with {} residents", residents),
            residents,
            is_coastal,
        }
    }
}

fn product(nums: Vec<i32>) -> i32 {
    let mut total = 1;

    for num in nums.iter() {
        total *= num;
    }

    total
}

fn sum(nums: Vec<i32>) -> i32 {
    let mut total = 1;

    for num in nums.iter() {
        total += num;
    }

    total
}

fn average(nums: Vec<i32>) -> i32 {
    let length = nums.len() as i32;

    sum(nums) / length
}
