let config_max = Some(3u8);

match config_max {
    // if we want to bind the value to the variable max in the pattern
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}

// less verbose way because we dont need to add _ => after processing just one variant
// in here, the max bind to the value inside the Some of var config_max
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}

let mut count = 0;

// This:
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

// is equivalent to this (except less verbose):
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
