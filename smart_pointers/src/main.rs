/**
 * =========== Boxes ===========
 * 
 * Allow you to store data on the heap (the box is a pointer)
 * 
 * Used when: 
 * 
 * 1) You have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size
 * 
 * 2) When you have a large amount of data and you want to transfer ownership, but want to ensure data won't be copied when you do
 * 
 * 3) When you want to own a value and you only care that it's a type that implements a specific trait (rather than being of a specific type) (trait object)
 */

use List::{Cons, Nil};
fn main() {
    let b = Box::new(5); // Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated.  The deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap).

    // One type whose size can’t be known at compile time is a recursive type, where a value can have as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.


    println!("b = {}", b);

    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));

    // Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types. They also don’t have any performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need.

    // ==== DEREF trait and Smart Pointers ===

    // The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.
}

enum List {
    Cons(i32, Box<List>),
    Nil
}