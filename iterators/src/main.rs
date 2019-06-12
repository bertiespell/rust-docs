fn main() {
    println!("Hello, world!");
}

// Iterators are lazy (no effect until you call methods that consume them)

// All iterators implement a trait named Iterator that is defined in the standard library

trait Iterator {
    type Item; // defining an associated type with this trait

    fn next(&mut self) -> Option<Self::Item>; //  this code says implementing the Iterator trait requires that you also define an Item type, and this Item type is used in the return type of the next method. In other words, the Item type will be the type returned from the iterator.

    // methods with default implementations elided
}