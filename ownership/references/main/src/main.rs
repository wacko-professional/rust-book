fn main() {
    let mut s1 = String::from("hello");
    // circumventing ownership by passing by reference
    let len = calculate_length(&s1);

    //// Mutable references have one big restriction; You can only have one mutable reference
    //// to a particular piece of data at a time.
    //// Here we can synchronise the references using scopes
    {
        let r1 = &mut s1;
    }
    let r2 = &mut s1;
    // Note: We also cannot have a mutable reference while we have an immutable one
    //println!("{}, {}", r1, r2);

    change(&mut s1);
    println!("The length of '{} is {}.", s1, len);
}

// Function that has a reference to an object as a parameter instead of
// taking ownership of the value.
// The ampersands are references that allow you to refer to some value without taking ownership
// of it
fn calculate_length(s: &String) -> usize {
    // here s refers to the value of s1 but does not own it.
    // Because it does not own it, the value it points to will not be dropped when the reference
    // stops being used.
    s.len()
}

fn change(some_string: &mut String) {
    // Note: References are immutable by default
    some_string.push_str(", world");
}
