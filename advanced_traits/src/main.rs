fn main() {
    println!("Hello, world!");
}

/**

One example of a trait with an associated type is the Iterator trait that the standard library provides. The associated type is named Item and stands in for the type of the values the type implementing the Iterator trait is iterating over.

 */

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>; // The type Item is a placeholder type, and the next method’s definition shows that it will return values of type Option<Self::Item>. Implementors of the Iterator trait will specify the concrete type for Item, and the next method will return an Option containing a value of that concrete type.
}

/**

Associated types might seem like a similar concept to generics, in that the latter allow us to define a function without specifying what types it can handle. So why use associated types?

Let’s examine the difference between the two concepts

 */