fn main() {
    use_new_add();
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

The difference is that when using generics, as in Listing 19-21, we must annotate the types in each implementation; because we can also implement Iterator<String> for Counter or any other type, we could have multiple implementations of Iterator for Counter. In other words, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time. When we use the next method on Counter, we would have to provide type annotations to indicate which implementation of Iterator we want to use.

With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times. In Listing 19-20 with the definition that uses associated types, we can only choose what the type of Item will be once, because there can only be one impl Iterator for Counter. We don’t have to specify that we want an iterator of u32 values everywhere that we call next on Counter.
 */

pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

// ~~~~~~~~~ Default Generic Type Parameters and Operator Overloading ~~~~~~~~

/**

When we use generic type parameters, we can specify a default concrete type for the generic type. This eliminates the need for implementors of the trait to specify a concrete type if the default type works. The syntax for specifying a default type for a generic type is <PlaceholderType=ConcreteType> when declaring the generic type.

A great example of a situation where this technique is useful is with operator overloading. Operator overloading is customizing the behavior of an operator (such as +) in particular situations.

Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator.
 */

// Here we overload the + operator to add two Point instances together. We do this by implementing the Add trait on a Point struct:

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn use_new_add() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}

// The default generic type in this code is within the Add trait. Here is its definition:

trait AddExample<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters { // example here we don't just use default
        Millimeters(self.0 + (other.0 * 1000))
    }
}