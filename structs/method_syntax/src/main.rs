// Methods are similar to functions, but they're defined within the context of a struct
// Their first parameter is always self (represents the INSTANCE of the struct the method
// is called on)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl (implementation) block for Rectangle
// need this to define the function within the context of Rectangle
// main benefit of using methods over functions is for organization (put all things we can do
// with an instance of a type in one impl block)
impl Rectangle {
    // making an area method defined on the Rectangle struct
    // in an impl block, the type self is an alias for the type that the impl block is for
    // methods must  have a param named self of type self for the first param
    // here, the method is borrowing self immutably (else would do &mut self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // can choose to give a method the same name as one of the struct's fields
    // often (but not always) methods with the same name as a field will be defined to only
    // return the value in the field and do nothing else (getters)
    fn width(&self) -> bool {
        self.width > 0
    }

    // using an immutable borrow of rect2 (an instance of Rectangle)
    // methods can take multiple parameters that we add to the signature after the self param
    // these params work just like params in functions
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // we can have associated functions that dont have self as their first parameter
    // these are not methods because they dont need an instance of the type to work with
    // associated functions that arent methods are often used for constructors
    // Note: Just like String::from, we need to use :: syntax to call this function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // should return True
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // should return False
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
        );

    // with parentheses, Rust knows we mean the method width
    if rect1.width() {
        // without parentheses, Rust knows we mean the field width
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
