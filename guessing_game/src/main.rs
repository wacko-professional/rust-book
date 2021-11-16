use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    
    // Takes a range expression as an argument and generates a random number in the range
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Comment out: No fun telling what the number is
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // In Rust, values are immutable by default
        // String::new is a function that returns a new instance of a String
        // new is an associated function of the String type
        let mut guess = String::new();

        // calling the stdin function from the io  module
        // The stdin function returns an instance of std::io::Stdin, which is a type that represents a
        // handle to the standard iput for your terminal
        io::stdin()
            // Calling read_line method on the standard input handle to get input from the user
            // read_line needs to append whatever the user types into a string; the string argument
            // needs to be mutable so the method can change the string's content by adding the user
            // input
            // References are immutable by default
            .read_line(&mut guess)
            // read_line returns an io::Result, whose types are enumerations (Ok or Err)
            // An instance of io::Result has an expect method that you can call
            // If is an Err value, expect will make the program crash and display the arg in expect
            .expect("Failed to read line");

        // shadowing the previous value of guess
        // u32 is a good default choice for a small positive number
        // the parse method returns a Result type (as much as the read_line method does)
        let guess: u32 = match guess.trim().parse() {
            // Ok value will contain the resulting parsed number; will end up assigned as u32 guess
            Ok(num) => num,
            // _ is a catchall value; continue means to go to the next iteration of the loop
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // cmp method compares two values and can be called on anything that can be compared
        // it takes a referrence to whatever you want to compare with
        // returns a variant of the Ordering enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // break to exit loop
                break;
            }
        }
    }
}
