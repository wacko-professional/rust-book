fn plus_one(d: i32) -> i32 {
    d + 1
}

// Most functions in Rust return the last expression implicitly, but you can also return early
// from a function by using the return keyword and specifying a value
fn five() -> i32 {
    5
}

fn main() {
    let x = {
        // Below is an expression; Expressions evaluate to a value
        // Note: Expressions do not include ending semicolons; else you turn it into a statement
        let z = 3;
        z + 1
    };
    // Below is a statement (statements dont return values); Note however that the 6 is an
    // expression that evaluates to the value of 5, which y then binds to.
    let y = 6;
    let q = five();
    let d = plus_one(5);
    println!("Hello, world!");
    println!("The value of q is: {}", q);
    println!("The value of d is: {}", d);

    another_function(5);
    print_labeled_measurement(5, 'h');
}

// need to declare type of each parameter in function signatures
fn another_function(x: i32) {
    println!("The value of x  is: {}", x);
}
//fn another_function() {
//    println!("Another function");
//}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
