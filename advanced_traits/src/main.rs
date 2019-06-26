fn main() {
    use_new_add();
    use_default_implementation();
    fully_qualified_syntax();
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

    fn add(self, other: Meters) -> Millimeters { // example here we don't just use default, we override
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// ~~~~~ Fully Qualified Syntax for Disambiguation: Calling methods with the same name ~~~~~

/**

Nothing in Rust prevents a trait from having a method with the same name as another trait’s method, nor does Rust prevent you from implementing both traits on one type. It’s also possible to implement a method directly on the type with the same name as methods from traits.

When calling methods with the same name, you’ll need to tell Rust which one you want to use. 
 */


trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// When we call fly on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type,

fn use_default_implementation() {
    let person = Human;
    person.fly();

    // To call the fly methods from either the Pilot trait or the Wizard trait, we need to use more explicit syntax to specify which fly method we mean
    Pilot::fly(&person);
    Wizard::fly(&person);
}

// However, associated functions that are part of traits don’t have a self parameter. When two types in the same scope implement that trait, Rust can’t figure out which type you mean unless you use fully qualified syntax. For example, the Animal trait in Listing 19-27 has the associated function baby_name, the implementation of Animal for the struct Dog, and the associated function baby_name defined on Dog directly.

// A trait with an associated function and a type with an associated function of the same name that also implements the trait:
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn fully_qualified_syntax() {
    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name()); ERROR - this won't compile => type annotations required: cannot resolve `_: Animal`
    // Because Animal::baby_name is an associated function rather than a method, and thus doesn’t have a self parameter, Rust can’t figure out which implementation of Animal::baby_name we want. 

    // To disambiguate and tell Rust that we want to use the implementation of Animal for Dog, we need to use fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
}

// ~~~~~~~~~~ Super Traits! ~~~~~~~~~

/**

Sometimes, you might need one trait to use another trait’s functionality. In this case, you need to rely on the dependent trait’s also being implemented. The trait you rely on is a supertrait of the trait you’re implementing.

For example, let’s say we want to make an OutlinePrint trait with an outline_print method that will print a value framed in asterisks. That is, given a Point struct that implements Display to result in (x, y), when we call outline_print on a Point instance that has 1 for x and 3 for y, it should print the following:

**********
*        *
* (1, 3) *
*        *
**********

In the implementation of outline_print, we want to use the Display trait’s functionality. Therefore, we need to specify that the OutlinePrint trait will work only for types that also implement Display and provide the functionality that OutlinePrint needs. We can do that in the trait definition by specifying OutlinePrint: Display. This technique is similar to adding a trait bound to the trait. 

 */

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}