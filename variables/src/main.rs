fn main() {
    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    //let mut x = 5;
    //
    // Shadowing: Note - variable is still immutable after the loop transformations have been
    // completed
    let x = 5;
    let x = x + 1;
    let spaces = "   ";
    let spaces = spaces.len();
    {
        let x = x *2;
        println!("The value of x in the inner scope is: {}", x);
    }
    //x = 6;
    println!("The value of x is: {}", x);
}
