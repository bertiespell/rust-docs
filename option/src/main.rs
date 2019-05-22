fn main() {
    test_options();
}

// Option - another enum defined by the standard library
// Encodes the idea that something could be SOMETHING or NOTHING
// Expressing this in the type system checks that we are handling errors properly
// Prevents bugs

// Rust does not have NULL (a value which means there is no value)

// However the concept still needs to be conveyed:
// The value either doesn't exist, or is absent or invalid for some reason

// INSTEAD

// Rust has an enum (Option<T>) which encodes that a value can be PRESENT or ABSENT

// enum Option<T> { // Doesn't need to be brought into scope to use (included in Prelude)
//     Some(T), // SO are its variants (some and none) - which don't need to use :: syntax
//     None, 
// }

// If we use None rather than Some - we need to tell Rust what type of Option we have - the compiler cannot infer the type

fn test_options() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let absent_number: Option<i32> = None;

    // because Option<T> is a different type to T
    // we cannot
    // let sum = x + y; Basically here you need to convert y to T before you can use it (which means handling an error).

    // So - how to get the value of T out?
    // Handle when T is there - handle when it's not
    // Sounds like a case for PATTERN MATCHING
    //...... i.e. match
    value_in_cents(Coin::Quarter(UsState::Alabama));
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin { // unlike if - the expression used here doesn't have to be a boolean value
        Coin::Penny => 1, // match arms
        Coin::Nickel => 5, // pattern => then some code
        Coin::Dime => 10, // don't need {} if the arm is short
        Coin::Quarter(State) => {
            println!("{:?}", State);
            25
        },
    }
}

// match arms can bind to the parts of the value that match a pattern 


// Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value. If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.

fn plus_one(number: Option<i32>) -> Option<i32> {
    match number {
        Option::Some(value) => Some(value + 1),
        None => None, // don't need that namespacing
    }
}

fn use_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}


// a placeholder for THE REST

fn placeholder() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}