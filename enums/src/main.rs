enum IpAddrKind {
    V4,
    V6, // these are known as the VARIANTS of the enum
}

fn main() {
    // We can create INSTANCES of these variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6; // these are namespaced under its identifier

    // this is useful, because values four and six -> ARE NOW OF THE SAME TYPE
    route(four);
    route(six);
    route(IpAddrKind::V4);
}

fn route(ip_type: IpAddrKind) {
    
}

// What if we want to store data along with a type
// We might be tempted to do the following

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn test_enums_and_structs() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // We used a struct to bundle a kind and a value together
    // So now the variant is associated with a value

    // BUT:
    // We can represent this much more concisely using just an enum (this is a common patter)

    let home2 = RefactoredIpAddr::V4(String::from("127.0.0.1"));
    let loopback2 = RefactoredIpAddr::V6(String::from("::1"));

    let home3 = ReRefactoredIpAddr::V4(127, 0, 0, 1);
    let loopback3 = ReRefactoredIpAddr::V6(String::from("::1"));
}

enum RefactoredIpAddr {
    V4(String), // We attach data to each variant of the enum directly
    V6(String), // So there is no need for an extra struct
}

// This has another advantage!
// Each variant can accept different data types AND amounts of data associated

enum ReRefactoredIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Storing and encoding IP addresses is so common that it is part of the standard library!

// You can put any type of data inside an enum

// Even though there is a standard library definition we can use ours without conflict. Because we haven't brought the standard library one into scope

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // this contains an anonymous struct inside it
    Write(String),
    ChangeColour(i32, i32, i32),
}

// If you wanted to, you could write these as their own Structs

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

pub struct MessageStruct {
    Quit: QuitMessage,
    Move: MoveMessage,
    Write: WriteMessage,
    ChangeColour: ChangeColorMessage,
}

// We can also make impls on enums!

impl Message {
    fn call(&self) {
        // method body defined here
    }
}

fn use_impl() {
    let m = Message::Write(String::from("Write a little message"));
    m.call();
}