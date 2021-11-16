fn main() {
    let word = String::from("Hello world!");
    let first = first_word(&word);
    println!("First word is: {}", first);
}

fn first_word(s: &String) -> usize {
    // need to go through String element by element and check whether a value is a space
    // so we convert our string to an array of bytes
    let bytes = s.as_bytes();

    // Create an iterator over the array of bytes using the iter method
    // iter is a method that returns each element in a collection
    // enumerate wraps the result of iter and returns each element as part of a tuple instead
    // first element of the tuple: index, second element: a reference to the element
    for (i, &item) in bytes.iter().enumerate() {
        // byte literal syntax
        if item == b' ' {
            // if we find a space, we return the position
            return i;
        }
    }
    // otherwise, we return the length of the string
    s.len()
}
