fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a Let statement
    let condition = true;
    // the number variable will be bound to a value based on the outcome of the if expression
    // the type of number variable must be the same (i.e. cannot use a string type in else)
    let number = if condition { 5 } else { 6 };

    println!("The value of number is:{}", number);

    // use break keyword to exit
    // the continue keyword within a loop tells program to skip over any remaining code in this
    // iteration of the loop and go to the next iteration
    //loop {
    //    println!("again!");
    //}

    let mut count = 0;
    // outer loop has label 'counting_up
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // in loops, add the value you want to return after the break expression
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // While loops
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");


    // Looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // OK
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // Better
    for element in a {
        println!("the value is: {}", element);
    }

    // Most Rustaceans would use a forr loop for while conditions (safety + conciseness)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");

//    if number != 0 {
//        // blocks of code associated with the conditions in if expressions are sometimes called
//        // arms (just like the arms in match expressions)
//        println!("number was something other than zero");
//    } //else {
////        println!("condition was false")
////    }
}
