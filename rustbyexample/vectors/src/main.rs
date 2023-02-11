use std::vec;

#[derive(Debug)]
struct Person {
    username: String,
    id: i32,
}

fn main() {
    // let mut years: Vec<i32> = vec![1995, 2000, 2005];
    // years.push(2010);
    // println!("{:?}", years);

    let mut people: [Person; 2] = [
        Person {
            username: "farah".to_string(),
            id: 1,
        },
        Person {
            username: "sidiiq".to_string(),
            id: 2,
        },
    ];

    let mut users: Vec<Person> = vec![
        Person {
            username: "farah".to_string(),
            id: 1,
        },
        Person {
            username: "sidiiq".to_string(),
            id: 2,
        },
    ];

    for user in users {
        print!("{} is {} ", user.id, user.username);
    }
}
