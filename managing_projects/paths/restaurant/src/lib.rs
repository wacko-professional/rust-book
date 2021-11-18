mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist()  {}
    }
}

use crate::front_of_house::hosting;
// Note, we can also bring an item into scope using a relative path:
// use self::front_of_house::hosting;
// Note: If we want external code to refer to it, we need to prepend with pub
// pub use crate::front_of_house::hosting;
// Using nested paths to bring multiple items into scope in one line
// use std::{cmp::Ordering, io};// is equivalent to std::cmp::Ordering and std::io
// We can also do it this way:
// use std::io::{self, Write} if we want to use std::io and std::io::Write
// We can bring all public items define in a path into scope using the * operator
// use std::collections::*; // this brings all public items defined in std::collections into the
// current scope (but makes it harder to tell where a name used in your program is defined)


// bringing hosting module into the scope of the eat at restaurant function
pub fn eat_at_restaurant() {
    // we only need to specify this way now that we've brought hosting module into the scope
    hosting::add_to_waitlist();
    // this is the idiomatic way; bringing the FUNCTION's parent module into scope means we
    // have to specify the parent module when calling the function; makes it clear that the
    // function isnt locally defined while still minimising repetition of the full path
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::collections::HashMap;

fn main() {
    // Different to structs, enums and other items
    // idiomatic way to bring the standard library's HashMap struct into the scope of a binary
    // crate
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// if bringing two items with the same name into scope with use, you cannot refer the full
// path because Rust doesnt allow that (having 2 types of the same name in the same scope)
use std::fmt;
use std::io;
// Note: We can use as when using use to specify a new local name/alias
// use std::io::Result as IoResult;

fn function1() -> fmt::Result {

}

fn function2() -> io::Result<()> {

}
