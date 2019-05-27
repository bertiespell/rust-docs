// You can use trait bounds to conditionally implement methods

// This struct is generically typed
// If the UNDERLYING structure - T - implements both Display and PartialOrd THEN we can call cmp_display

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> { // this has to declare a type
    fn new(x: T, y: T) -> Self { // Can return Self here instead of Pair<T>
        Self {
            x,
            y,
        }
    }
}

use std::fmt::Display;

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// we can also conditionally implement a trait for any type that implements another trait

// implements of a train on a type that satisfied the trait bounds are called "blanket implementations" (used extensively in the Rust standard library)

// For example, the standard library implements the ToString trait on any type that implements the Display trait.

impl<T: Display> ToString for T {
    unimplemented!();
}

// Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like this because integers implement Display:

fn main() {
    let s = 3.to_string();
    println!("I turned into a letter: {}", s);
}

// Blanket implementations appear in the documentation for the trait in the “Implementors” section.


