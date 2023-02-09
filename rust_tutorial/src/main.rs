fn main() {
    let city_name = "Hargeisa";
    println!("The city of {}", city_name);

    print_population(1_324_578, 114_293, 108_097)
}

fn print_population(adults: u64, kids: u32, buildings: u32) {
    let population = adults + (kids as u64);

    let buildings_per_person = (buildings as f64) / (population as f64);

    println!("Population {}", population);
    println!("Kids {}", kids);
    println!("Adults {}", adults);
    println!("Buildings {}", buildings);
    println!("Buildings per person {}", buildings_per_person);
}
