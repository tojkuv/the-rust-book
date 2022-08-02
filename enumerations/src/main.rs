fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // using a structure type here is more verbose that implementing an enum type with embedded data
    let home = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // enum types are more succinct than structure types
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    // let m be binded to an enumeration variant `Write` from the enumeration type `Message`, which contains the
    // associated data of a String type, which is initialized by the associated function `from` that contains a string
    // literal arguement `"hello"` 
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // compile-time error: x and y are of different types
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {

}

struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    // the name of each enum variant becomes a function that returns an instance of the enum variant 
    // (an IpAddr variant in this case)
    V4(String),
    V6(String),
}

enum IpAddr3 {
    // this would not be possible using a structure since types associated with key values can only be of one type.
    // enumerations can be of the same enumation type but have different associated data with each enumeration.
    // associated data can be string literals, String types, numeric types, structures, enumerations, etc.
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(self: &Self) {
        // the body of the method would use `self` to get the value that the method was called on. the `.` access 
        // operator can be used to access associated data of an enum variant.
    }
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct

enum Option<T> {
    // this enumeration from the standard library is imported automatically in the prelude. it is a proper 
    // implementation of the concept `null`, which was poorly implemented in ALGO and passed down to subsequent 
    // languages. 
    None,   // anytime `Some(T)` is used, the `None` variant has to accounted for in the code (the compiller will provide
            // warning when such cases are not handled by something like a `match` expression)
    Some(T),
}

// note: the standard library types are often not more complex than simple type implementations
// note: there are no null pointers, so any type pointer that does not point ot an `Option<T>` value, has associated
// data by design.
// question: are namespaces statements or expressions? 