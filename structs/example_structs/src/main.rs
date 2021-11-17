//-------
// we have to opt in to make the Debug functionality available for our struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// making a new project that will take the width and height of a rectangle specified in pixels
// and calculate the area of the rectangle
fn main() {
    //let width1 = 30;
    //let height1 = 50;
    //let rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // doing an immutable borrow of a struct Rectangle instance, so that main retains its
        // ownership and can continue using rect1
        area(&rect1)
        );

    // Structs dont have a provided implementation of Display
    //println!("rect1 is {}", rect1);
    // Output format: Debug
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        // dbg! returns ownership of the expression's value, so result still gets assigned to width
        width: dbg!(30 * scale),
        height: 50,
    };

    // we dont want dbg! to take ownership of rect2, so we use a reference to dbg! instead
    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
//// better than using two parameters for function input is to use a struct as the dimensions
//// in this case are related (both belong to the SAME rectangle)
//fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//}
//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}
