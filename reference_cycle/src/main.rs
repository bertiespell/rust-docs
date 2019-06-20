use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

// This code creates a list in a and a list in b that points to the list in a. Then it modifies the list in a to point to b, creating a reference cycle. There are println! statements along the way to show what the reference counts are at various points in this process.
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), // A cons list definition that holds a RefCell<T> so we can modify what a Cons variant is referring to... we want to modify which List value a Cons variant is pointing to
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> { // make it convenient for us to access the second item if we have a Cons variant.
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}