fn main() {
    // a string slice is a reference to part of a String
    // internally the slice data structure stores the starting position and the length of the slice
    let mut s = String::from("hello, world");

    // Rather than a reference to the entire String, it's a reference to a portion of the String
    let hello = &s[0..5];// Note: This is equivalent to &s[..5]
    let world = &s[6..11];

    // making use of the immutable reference
    let word = first_word(&s);
    //// making use of the mutable reference
    //s.clear();// error!
    //// immutable reference still active; if we have an immutable reference to something, we
    //// cannot also take a mutable reference
    //// Note that a reference's scope starts from where it is introduced and continue through
    //// to the last time that reference is used.
    //println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // returning a slice
            return &s[0..i];
        }
    }

    &s[..]
}
