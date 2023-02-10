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

struct City {
    residents: i64,
    description: String,
    is_coastal: bool,
}
fn main() {
    let rustville = new_city(123, false);

    println!("{}", rustville.description)
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
