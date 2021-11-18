// Declaring the front of house module, whose body will be in src/front_of_house
// Using a semicolon after mod front_of_house tells Rust to load the contents of the module
// FROM ANOTHER FILE with the SAME NAME as the module
// 
// The mod keyword declares modules and Rust looks in a file with the same name as the module
// for code that goes into that module
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
