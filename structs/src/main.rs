fn main() {
    println!("Hello, world!");
    let email = String::from("Bertie");
    let username = String::from("example@example.com");
    create_user(email, username);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn create_instance() {
    let user = User {
        username: String::from("Bertie"),
        email: String::from("example@example.cpm"),
        active: true,
        sign_in_count: 10,
    };

    // use dot notation to get specific values out user.email
    // has to be mutuable to assign a particular field
    let mut mutable_user = User {
        username: String::from("Bertie"),
        email: String::from("example@example.cpm"),
        active: true,
        sign_in_count: 10,
    };

    mutable_user.email = String::from("New name");
}

fn create_user(email: String, username: String) -> User {
    // as with any expression
    // we can create a new instance of a struct as the last expression in a function body
    // which explicitly returns that new instance

    User {
        email: email, // this can be shorthand though - see below
        username,
        active: true,
        sign_in_count: 1
    }
}

// sometimes useful to create a new struct, using data from an old instance
// use Struct Update Syntax

fn create_from_other() {
    let user1 = User {
        username: String::from("Bertie"),
        email: String::from("example@example.cpm"),
        active: true,
        sign_in_count: 10,
    };

    let user2 = User {
        username: String::from("Bertie"),
        email: String::from("example@example.cpm"),
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };

    // we can do this quicker with struct update syntax

    let user3 = User {
        username: String::from("Bertie"),
        email: String::from("example@example.cpm"),
        ..user1
    };
}

fn tuple_structs() {
    // structs that look like tuples
    struct Color(i32, i32, i32); // can be destructured to access individual pieces
    struct Point(i32, i32, i32); // can access via dot notation (followed by index)

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn unit_like_structs() {
    // these are like ()
    // they don't have any fields

    struct Unit_Like_Struct();

    // good in situations in which you want to define a trait on some type
    // but don't have any *data* to store in the type itself
}

// In the user example before - we used the String type and NOT a slice &str
// This was deliberate
// We want the struct to own all of its data AND for that data to be valid for as long as the entire struct is valid

// It is possible for Structs to store references to data owned by something else - but this requires using lifetimes

// Lifetimes ensure that the data in the struct is valid for the entire lifetime of the struct