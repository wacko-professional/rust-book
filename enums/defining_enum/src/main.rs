//enum IpAddrKind {
//    V4,
//    V6,
//}

// We can present the same concept in a more concise way using just an enum
// rather than an enum inside a struct by putting data directly into each enum variant
enum IpAddr {
    // This says that both V4 and V6 variants will have associated values
    V4(u8, u8, u8, u8),
    V6(String),
    // Note: Can even embed enum variants in the form of strucs
    // V6(Ipv6Addr)
}

enum Message {
    Quit,
    // has named fields like a struct does
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// just as we are able to define methods on structs using impl, we're also able to define
// methods on enums
impl Message {
    // method named call that we could define on our Message enum
    fn call(&self) {
        // method body would be defined here
        // self will be a variable m that  has the value Message::Write(String::from("hello"))
    }
}

let m = Message:Write(String::from("hello"));
m.call();

// The Option<T> enum can encode the concept of a value being present or absent
// The <T> syntax is a generic type parameter. <T> means the Some variant of the Option
// enum can hold one piece of data of any type, and each type makes the overall Option<T>
// type a different type
let some_number = Some(5);// is of type Option<i32>
let some_string = Some("a string"); // is of type Option<&str>

// Is better than having null, because the compiler wont let us use an Option<T> value as if
// it were definitely a valid value
let absent_number: Option<i32> = None;// is of type  Option<i32>

let x: i8 = 5;
let y: Option<i8> = Some(5);
// Will get an error message because Rust doesnt understand how to add an i8 and an Option<i8>
// In other words, you have to convert an Option<T> to a T before you can perform T operations
// with it (helps to catch most common issue with null: assuming that something isnt null when
// it actually is)
let sum = x + y;

//// using a Struct to bundle the kind and address values together
//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

//let home = IpAddr {
//    kind: IpAddrKind::V4,
//    address: String::from("127.0.0.1"),
//};

// Attaching data to each enum variant directly
let home = IpAddr::V4(127, 0, 0, 1);

//let loopback = IpAddr {
//    kind: IpAddrKind::V6,
//    address: String::from("::1"),
//};

let loopback = IpAddr::V6(String::from("::1"));

//// Create instances of each of the variants in our enum
//// Now both values are of the same type (namespaced by IpAddrKind)
//let four = IpAddrKind::V4;
//let six = IpAddrKind::V6;
//
//// we can call this function with either variant
//fn route(ip_kind: IpAddrKind) {}
//
//route(IpAddrKind::V4);
//route(IpAddrKind::V6);
