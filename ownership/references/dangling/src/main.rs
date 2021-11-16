fn main() {
    //let reference_to_nothing = dangle();
    let no_reference = no_dangle();
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    // Rust does not allow dangling references
//    &s// we return a reference to the String, s
//}// Here, s goes out of scope, and is dropped. Its memory goes away.
// //Danger!
 
// This works without any problems; Ownership is moved out and nothing is deallocated
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
