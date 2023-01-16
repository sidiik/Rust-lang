fn main() {
    println!("Hello world");
    printing::announce_time();
}

mod printing {
    pub mod time_stuff;

    pub use time_stuff::print_date_time as announce_time;
}
