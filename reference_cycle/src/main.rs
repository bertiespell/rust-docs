use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

fn main() {
    println!("Hello, world!");
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