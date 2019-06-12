// In the workout generator example, we only used closures as inline anonymous functions. However, closures have an additional capability that functions don’t have: they can capture their environment and access variables from the scope in which they’re defined.

// This is an important security feature

fn main() {
    let x = 4;

    let equal_to_x = |z| z == x; // uses something in scope

    let y = 4;

    assert!(equal_to_x(y));
}

fn example_of_capturing_closures() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x); We can't use x here!

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

/** TYPES of closure

FnOnce => consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.

FnMut => can change the environment because it mutably borrows values.

Fn => borrows values from the environment immutably.

The compiler can infer these :)
 */

// If you want to force the closure to take ownership of the values it uses in the environment, you can use the move keyword before the parameter list. This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread.