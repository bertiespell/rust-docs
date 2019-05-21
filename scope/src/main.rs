fn main() {
    let _s = "hello"; // this is a string literal (stack?) - it is immutable
    let mut s1 = String::from("hello"); // this is allocated on the HEAP - stores an amount of text unknown at compile time
    s1.push_str(" world");
    println!("{}", s1);

    let s2 = s1;
    // println!("{}", s1); // this line wouldn't work. If s1 and s2 both go out of scope there is a double-free error
    // to avoid this, Rust considers s1 to no longer be valid here

    // this doesn't just copy the pointer, length and capacity (which is a shallow copy) - it also invalidates the first variable and for this reason it is called a MOVE!
    // s1 was moved into s2

    // automatic copying always occurs in this way - i.e. it is never "deep"

    let s3 = s2.clone();
    println!("Cloned!: {}, {}", s3, s2);

    let myString = String::from("Hello again"); // comes into scope

    takes_ownership(myString); // this takes ownership and so other things can't use it

    let x = 5;

    makes_copy(x); // can still use x after this point because it is a i32

    test_ownership();
}

// when _s goes out of scope, Rust calls a special function called 'drop' for us, which frees the memory

// If a type has the Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait. 

// Data on the heap will be dropped after it goes out of scope, unless it's moved to another function or piece of code that takes ownership

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(number: i32) {
    println!("{}", number);
}

fn test_ownership() {
    let s1 = gives_ownership(); // gives_ownership moves its return... the value is copied into s1

    let s2 = String::from("Another little example");

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let returning_string = String::from("Yello");
    returning_string
}

fn takes_and_gives_back(example_string: String) -> String {
    example_string
}

// returns two values

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// it is tedious to always pass ownership around like this though. So it's possible to use a pointer.