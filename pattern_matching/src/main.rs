/**
 * Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple. Using patterns in conjunction with match expressions and other constructs gives you more control over a program’s control flow. A pattern consists of some combination of the following:

Literals
Destructured arrays, enums, structs, or tuples
Variables
Wildcards
Placeholders
These components describe the shape of the data we’re working with, which we then match against values to determine whether our program has the correct data to continue running a particular piece of code.

To use a pattern, we compare it to some value. If the pattern matches the value, we use the value parts in our code


e.g.

match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

match must be exhaustive...

A particular pattern _ will match anything, but it never binds to a variable, so it’s often used in the last match arm. The _ pattern can be useful when you want to ignore any value not specified, for example. 

if let expressions are a shorter way to write the equivalent of a match that only matches one case. Optionally, if let can have a corresponding else containing code to run if the pattern in the if let doesn’t match.


 */
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 { // You can see that if let can also introduce shadowed variables in the same way that match arms can: the line if let Ok(age) = age introduces a new shadowed age variable that contains the value inside the Ok variant. This means we need to place the if age > 30 condition within that block: we can’t combine these two conditions into if let Ok(age) = age && age > 30. The shadowed age we want to compare to 30 isn’t valid until the new scope starts with the curly bracket.
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // the while let conditional loop allows a while loop to run for as long as a pattern continues to match.
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loops - In a for loop, the pattern is the value that directly follows the keyword for, so in for x in y the x is the pattern.
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // even a simple let expression
    let x = 5;
    // let PATTERN = EXPRESSION;

}
