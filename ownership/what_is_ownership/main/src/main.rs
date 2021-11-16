fn main() {
    // the variable s (owner of "hello") is VALID from this point (where it's declared)
    // until the end of the current scope
    //let s = "hello";

    // This kind of String can be mutated; memory requested on the heap by memory allocator
    // at runtime and automatically returned once variable is out of scope
    let mut s = String::from("hello");
    s.push_str(", world!");//push_str() appends a literal to a String

    println!("{}", s);//This will print 'hello, world!'

    //// Memory safety detail in Rust (almost like a shallow copy, but Rust also invalidates the
    //// first variable)
    //let s1 = String::from("hello");
    //let s2 = s1;
    //// Rust will never automatically create "deep" copies of your data; any automatic copying
    //// will be assumed to be inexpensive in terms of runtime

    //println!("{}, world!", s1);

    // Doing a deep copy (need to use a common method called clone)
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //Stack-Only Data: Copy
    // Note: This is allowed, because nothing here is stored on the heap (no move happens)
    // out of scope)
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Can return multiple values using a tuple
    let s3 = String::from("hello");
    
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

// Problem with this is you have to return the String to the calling function so that you can
// still use it after the call to calculate_length
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();// len() returns the length of a String
    (s, length)
}
