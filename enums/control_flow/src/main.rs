//enum Coin {
//    Penny,
//    Nickel,
//    Dime,
//    Quarter
//}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// input is of type Enum; can take value of Penny, Nickel, Dime or Quarter
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // if you want to run multiple lines of code in a match arm, can use curly brackets
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // will bind the state field of the Quarter to the state attribute
        Coin::Quarter(state) => {
            // we can then use that binding in the prinln! expression
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // if there's a value, adds 1 to that value
    // if there isnt a value inside, function should return the None value
    match x {
        None => None,
        // Returns Some option holding value i + 1
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
// in this case, i will take the value of 5 in the plus one matching
let six = plus_one(five);
// matches with None block
let none = plus_one(None);

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    // use only if you want to bind all other
    //other => move_player(other),
    // if you dont want to bind, then can just use _
    //_ => reroll(),
    // if you want nothing else to happen, then can use:
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    println!("Hello, world!");
}
