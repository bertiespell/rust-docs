// The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope. You can provide an implementation for the Drop trait on any type, and the code you specify can be used to release resources like files or network connections. We’re introducing Drop in the context of smart pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer. For example, Box<T> customizes Drop to deallocate the space on the heap that the box points to.

// Specify the code to run when a value goes out of scope by implementing the Drop trait. The Drop trait requires you to implement one method named drop that takes a mutable reference to self. 
fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created."); // When main finishes - these will go out of scope and be dropped - printing out the line below declared in the trait... Variables are dropped in the reverse order of their creation, so d was dropped before c. 

    // Dropping early - included in the prelude

    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping: {}", self.data);
    }
}

// ====== Calling Drop Early ======

// Occasionally, however, you might want to clean up a value early. One example is when using smart pointers that manage locks: you might want to force the drop method that releases the lock to run so other code in the same scope can acquire the lock.

// Rust doesn’t let you call the Drop trait’s drop method manually; instead you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.

// Rust doesn’t let us call drop explicitly because Rust would still automatically call drop on the value at the end of main. This would be a double free error because Rust would be trying to clean up the same value twice.

// ==== Graph Structures
// Ownership is less clear and may require reference counting

/*
In the majority of cases, ownership is clear: you know exactly which variable owns a given value. However, there are cases when a single value might have multiple owners. For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it.

To enable multiple ownership, Rust has a type called Rc<T>, which is an abbreviation for reference counting.

The Rc<T> type keeps track of the number of references to a value which determines whether or not a value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.
 */

// We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

// Rc<T> is only for use in single-threaded scenarios

// Let's create two lists that both share ownership of a third list

// We’ll create list a that contains 5 and then 10. Then we’ll make two more lists: b that starts with 3 and c that starts with 4. Both b and c lists will then continue on to the first a list containing 5 and 10. In other words, both lists will share the first list containing 5 and 10.
use List::{Cons, Nil};

fn reference_counting() {
     let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // This isn't allowed - because of borrowing rules - only one thing allowed to take ownership - a already moved into b - so it can't be moved into c
}

enum List {
    Cons(i32, Box<List>),
    Nil
}

// we’ll change our definition of List to use Rc<T> in place of Box<T>

// Each Cons variant will now hold a value and an Rc<T> pointing to a List. When we create b, instead of taking ownership of a, we’ll clone the Rc<List> that a is holding, thereby increasing the number of references from one to two and letting a and b share ownership of the data in that Rc<List>

// We’ll also clone a when creating c, increasing the number of references from two to three. Every time we call Rc::clone, the reference count to the data within the Rc<List> will increase, and the data won’t be cleaned up unless there are zero references to it.
use std::rc::Rc;

enum ReferenceCountList {
    Cons(i32, Rc<ReferenceCountList>),
    Nil
}

use ReferenceCountList::{Cons as RcCons, Nil as RcNil};