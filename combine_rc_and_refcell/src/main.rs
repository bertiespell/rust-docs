
use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    // ------- EXAMPLE -------
    // Now we use RC together with RefCell - to get multiple owners of something that has interior mutation!
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); // nWe need to clone value so both a and value have ownership of the inner 5 value rather than transferring ownership from value to a or having a borrow from value.... We wrap the list a in an Rc<T> so when we create lists b and c, they can both refer to a

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10; // After we’ve created the lists in a, b, and c, we add 10 to the value in value. We do this by calling borrow_mut on value, which uses the automatic dereferencing feature to dereference the Rc<T> to the inner RefCell<T> value. The borrow_mut method returns a RefMut<T> smart pointer, and we use the dereference operator on it and change the inner value.

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}