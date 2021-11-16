fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // bool
    let t = true;
    let f: bool = false;

    // char
    let c = 'z';
    let z = 'Z'
    
    // tuple (w. optional type annotations); have fixed length
    let x: (i32, f64, u8) = (500, 6.4, 1);

    // getting individual values out of a tuple
    let (a, b, c) = tup;

    println!("The value of y is: {}", c);

    // can access a tuple element by using a period (.) followed by the index of the value
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // arrays (Have a fixed length; every element must have same type), allocated on the stack
    let a = [1, 2, 3, 4, 5];
    // denoting array of length 5 with elements of type i32
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // creating an array that contains the same value for each element
    let a = [3; 5];// (same as writing let a = [3, 3, 3, 3, 3])

    let first = a[0];
    let second = a[1];
}
