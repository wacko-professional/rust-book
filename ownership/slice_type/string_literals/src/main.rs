fn main() {
    // String literals are immutable references; hence they are immutable
    // they are slices pointing to that specific point of the binary
    let s: &str = "Hello world";
    let t = "Hello, world";
    println!("These are equivalent: s: {}, t:{}", s, t);

    let my_string = String::from("hello world");

    //first_word  works on slices of String's, whether partial or while
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // first word also works on references to Strings, which are equivalent to whole slices
    // of Strings
    let word = first_word(&my_string);

    // Note; this is a string literal (type &str)
    let my_string_literal = "hello world";

    // first_word works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already, this works too, without the
    // slice syntax!
    let word = first_word(my_string_literal);

    // String slices are specific to strings, but there's a more general slice type too
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];//the slice  has type &[i32] and works same way string slices do
    assert_eq!(slice, &[2, 3]);
}

// the signature shown allows us to use the same function on both &String values and &str values
// More experienced Rustacean would write this instead of s: &String
fn first_word(s: &str) -> &str {
    println!("First Word is : {}", s);
    s
}
