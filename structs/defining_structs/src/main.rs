// A struct's name should describe the significance of the pieces of data being grouped together
struct User {
    // pieces of data within a struct are called fields
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Can return a struct instance from a function
fn build_user(email: String, username: String) -> User {
    User {
        // we can just set email, we dont need to do email: email
        //email: email,
        email,
        //username: username,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    build_user(String::from("jordan.singgih@gmail.com"), String::from("jordansas"));
}

//// using a struct after we've defined it
//// create an instance by stating name of the struct then add curly brackets containing key:value
//// pairs. We dont have to specify the fields in the same order in which we declared them in the
//// struct
//// Note: If we mark it an instance as mut, the entire instance must be mutable
//let mut user1 = User {
//    email: String::from("someone@example.com"),
//    username: String::from("someusername123"),
//    active: true,
//    sign_in_count:1,
//};
//
//user1.email = String::from("anotheremail@example.com");
//
////fn main() {
////    println!("Hello, world!");
////}
