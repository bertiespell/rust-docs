// Finally, we’ll explore some advanced features related to functions and closures, which include function pointers and returning closures.

/**
 * Function Pointers
 * 
 * We’ve talked about how to pass closures to functions; you can also pass regular functions to functions! This technique is useful when you want to pass a function you’ve already defined rather than defining a new closure. Doing this with function pointers will allow you to use functions as arguments to other functions. Functions coerce to the type fn (with a lowercase f), not to be confused with the Fn closure trait. The fn type is called a function pointer. 
 * 
 * Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), so you can always pass a function pointer as an argument for a function that expects a closure. It’s best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures.
 */

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { // We specify that the parameter f in do_twice is an fn that takes one parameter of type i32 and returns an i32. 
    // Unlike closures, fn is a type rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

// An example of where you would want to only accept fn and not closures is when interfacing with external code that doesn’t have closures: C functions can accept functions as arguments, but C doesn’t have closures.

// As an example of where you could use either a closure defined inline or a named function, let’s look at a use of map. To use the map function to turn a vector of numbers into a vector of strings, we could use a closure, like this:

// Note that we must use the fully qualified syntax that we talked about earlier in the “Advanced Traits” section because there are multiple functions available named to_string. Here, we’re using the to_string function defined in the ToString trait, which the standard library has implemented for any type that implements Display.

// Some people prefer this style, and some people prefer to use closures. They end up compiling to the same code, so use whichever style is clearer to you.

fn closure_or_named_fn() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    // Or we could name a function as the argument to map instead of the closure, like this:
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
}

// Returning Closures 

// Closures are represented by traits, which means you can’t return closures directly. In most cases where you might want to return a trait, you can instead use the concrete type that implements the trait as the return value of the function. But you can’t do that with closures because they don’t have a concrete type that is returnable; you’re not allowed to use the function pointer fn as a return type, for example.

// The following code tries to return a closure directly, but it won’t compile:

// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

// ERROR => The error references the Sized trait again! Rust doesn’t know how much space it will need to store the closure. We saw a solution to this problem earlier. We can use a trait object:

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}