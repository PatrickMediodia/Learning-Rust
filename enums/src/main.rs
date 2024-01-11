use std::fmt;

/*
// creation of custom types where you can enumerate all the possible variants
enum Shapes {
    Rectangle,
    Circle,
    Triangle,
}
*/

enum IpAddrKind {
    V4,
    V6,
}

/* 
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/

// you can put data directly into an enum
// automatically get this constructor function defined as a result of defining the enum
enum IpAddrV1 {
    V4(String),
    V6(String),
}

// each variant can have different types and amounts of associated data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    /*
    // variants of an enum are namespaced under its identifier
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1");
    }
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1");
    }
    */

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let x: i8 = 5;
    let y: Option<i8> = None;

    // if y is not none, use that value
    // if y is none, use the default value -> unwrap_or(default)
    let sum = x + y.unwrap_or(0);
}

// implement the fmt::Display method for IpAddrKind enum
impl fmt::Display for IpAddrKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddrKind::V4 => write!(f, "V4"),
            IpAddrKind::V6 => write!(f, "V6"),
        }
    }
}

fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    println!("The IP address kind is {}", ip_kind);
    ip_kind
}

// enums can have a wide variety of types
enum Message {
    Quit,   // no data associateda at all
    Move { x: i32, y: i32 }, // has named fields like a struct
    Write(String), // includes a single string
    ChangeColor(i32, i32, i32), // includes 3 i32 values
}

/* 
// if we were to use structs to create the enum above, it would be harder to handle as it is different types
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

/* 
Rust does not have nulls
if a value is not Optional<T>, therefore it is sure to have a valid value
if a value is Optional<T>, you have to handle both cases
    - Some -> there is a value
    - None -> there is no value
Rust makes sure that there is a value if it is non optional, and if it is optional, you have to handle all the cases.
*/