// There's one big difference between the MyBox<T> type we're about to build and the real Box<T>: our version will not store its data on the heap. We are focusing this example on Deref, and so where the data is actually stored is less important than the pointer-like behavior.

use std::ops::Deref;

fn main() {
    // A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else. In Listing 15-6, we create a reference to an i32 value and then use the dereference operator to follow the reference to the data:

    let x = 5;
    let y = &x;
    println!("{}", x);
    println!("{}", *y);

    // rewritten using boxes

    let x = 5;
    let y = Box::new(x); // The only difference is that here we set y to be an instance of a box pointing to the value in x rather than a reference pointing to the value of x

    println!("{}", x);
    println!("{}", *y); // Using a box - the dereference operator still works

    let x = 5;
    let y = MyBox::new(x);
    println!("{}", x);
    println!("{}", *y);

    // Deref coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition.

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Rust calls deref again to turn the &String into &str, to match hello function definition

    // if we didn't have derefencing

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

fn hello(name: &str) { // &Mybox<String> becomes &String => DEREFERENCING
    println!("Hello, {}!", name); // // dereferencing happening here
}

// Let's build our own smart pointer

struct MyBox<T>(T); // tuple struct with one element

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // The type Target = T; syntax defines an associated type for the Deref trait to use. Associated types are a slightly different way of declaring a generic parameter

    fn deref(&self) -> &T {
        &self.0 // We fill in the body of the deref method with &self.0 so deref returns a reference to the value we want to access with the * operator.
    }
}
