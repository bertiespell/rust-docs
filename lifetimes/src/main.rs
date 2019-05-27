// every reference in Rust has a lifetime

// most of the time this is implicit and inferred

// Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

// Allows us to prevent dangling references
fn main() {
    let r;
    {
        let x = 5;
        r = &x // this won't compile - the value r is referring to has gone out of scope by the time we come to use it
    }
    println!("r: {}", r); // if this worked, r would be referencing memory that was deallocated when x went out of scope

    // RUST KNOWS!!   .... because of the Borrow Checker ;)
}

// Borrow Checker
// * Compares scopes to determine whether all borrows are valid
// * Lifetimes are shorter and so program is rejected

// generic lifetimes in functions

// Let’s write a function that returns the longer of two string slices. This function will take two string slices and return a string slice.

fn print_longest() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        return x
    } else {
        return y
    }
}

