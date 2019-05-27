// every reference in Rust has a lifetime

// most of the time this is implicit and inferred

// Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

// Allows us to prevent dangling references
fn main() {
    // let r;
    {
        let x = 5;
        // r = &x // this won't compile - the value r is referring to has gone out of scope by the time we come to use it
    }
    // println!("r: {}", r); // if this worked, r would be referencing memory that was deallocated when x went out of scope

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

    // When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. 

    // i.e. Rust can now infer this, we don't need to make this concrete here, because it just takes the smallest of the two
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the if case or the else case will execute. We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes

// The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value. To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

// Basically, we need to add more information for the borrow checker

// lifetime annotation syntax
// functions can accept references with any lifetime by specifying a generic lifetime parameter.

// Lifetimes have a very particular type of syntax

// The names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types
// Most people use the name 'a

// Here we can place lifetime parameter annotations after the & of a reference, using a space to separate the reference from the annotations type

// as with generic types, generic lifetime specifiers need to be in angled brackets

// The constraint we want to express in this signature is that all the references in the parameters and the return value must have the same lifetime.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. 
    // That's what we want Rust to enforce
    if x.len() > y.len() {
        return x
    } else {
        return y
    }
}

fn example_lifetimes_1() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result); // the shorter lifetime is used, but this is valid in here so it's fine
    }
}

fn example_lifetimes_2() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // this won't compile - "string2 doesn't live long enough"
    }
        println!("The longest string is {}", result);
}

// instead, if you knew the first string was always going to be longer (which would make this whole exercise pointless - you could instead do this)

fn longest_again<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer to a value created within this function, which would be a dangling reference because the value will go out of scope at the end of the function.

// Even though we’ve specified a lifetime parameter 'a for the return type, this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all

fn longest_3<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str() // result does not live long enough
}