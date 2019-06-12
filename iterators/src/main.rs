fn main() {
    println!("Hello, world!");
}

// Iterators are lazy (no effect until you call methods that consume them)

// All iterators implement a trait named Iterator that is defined in the standard library

trait Iterator {
    type Item; // defining an associated type with this trait

    fn next(&mut self) -> Option<Self::Item>; //  this code says implementing the Iterator trait requires that you also define an Item type, and this Item type is used in the return type of the next method. In other words, the Item type will be the type returned from the iterator.

    // The Iterator trait only requires implementors to define one method: the next method, which returns one item of the iterator at a time wrapped in Some and, when iteration is over, returns None.

    // methods with default implementations elided

    // iter() into_iter() iter_mut()
    // The iter method produces an iterator over immutable references.
    // If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead
    // Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.

    // Methods that call next are called consuming adaptors, because calling them uses up the iterator. 

    // Other methods defined on the Iterator trait, known as iterator adaptors, allow you to change iterators into different kinds of iterators.
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Let's implement our own Iterator trait over Counter!
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}