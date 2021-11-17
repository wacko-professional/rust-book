struct User {
    active: bool,
    // we want instances of this struct to own all of its data
    // and for that data to be valid for as long as the entire struct is valid
    username: String,
    email: String,
    sign_in_count: u64,
}

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");

// Struct update syntax: Create a new instance of a struct that uses most of an old instance's
// values but changes some
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
}

// better, more concise way
let user3 = User {
    email: String::from("another@example.com"),
    // syntax .. specified remaining fields not explicitly set should have the same values
    // as the fields in the given instance
    // MUST COME LAST in this syntax
    ..user1
}

// Tuple structs (useful when you want to give the whole tuple a name and make the tuple be a
// different type from other tuples, where naming each field would be verbose or redundant)
// Note; each struct you define is its own type (i.e. Color and Point are their own types)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// Unit-like structs without any fields
struct AlwaysEqual;
let subject = AlwaysEqual;

fn main() {
    println!("Reusing Structs")
}
