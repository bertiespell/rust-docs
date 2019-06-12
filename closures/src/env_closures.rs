// In the workout generator example, we only used closures as inline anonymous functions. However, closures have an additional capability that functions don’t have: they can capture their environment and access variables from the scope in which they’re defined.

// This is an important security feature

fn main() {
    let x = 4;

    let equal_to_x = |z| z == x; // uses something in scope

    let y = 4;

    assert!(equal_to_x(y));
}

/** TYPES of closure

FnOnce => consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.

FnMut => can change the environment because it mutably borrows values.

Fn => borrows values from the environment immutably.

 */