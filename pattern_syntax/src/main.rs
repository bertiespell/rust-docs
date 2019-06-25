fn main() {
    match_names_variables();
    multiple_patterns();
    match_range();
    destructuring();
}

fn match_against_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

//  there is a complication when you use named variables in match expressions. Because match starts a new scope, variables declared as part of a pattern inside the match expression will shadow those with the same name outside the match construct, as is the case with all variables.
fn match_names_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // The pattern in the second match arm introduces a new variable named y that will match any value inside a Some value. Because we’re in a new scope inside the match expression, this is a new y variable, not the y we declared at the beginning with the value 10. This new y binding will match any value inside a Some, which is what we have in x. Therefore, this new y binds to the inner value of the Some in x. That value is 5, so the expression for that arm executes and prints Matched, y = 5.
        _ => println!("Default case, x = {:?}", x),
    } // .... To create a match expression that compares the values of the outer x and y, rather than introducing a shadowed variable, we would need to use a match guard conditional instead

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn multiple_patterns() {
    let x = 2;

    match x {
        1 | 2 => println!("one or two"), // just use | operator
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_range() {
    let x = 5;

    match x { // Ranges are only allowed with numeric values or char values, because the compiler checks that the range isn’t empty at compile time. The only types for which Rust can tell if a range is empty or not are char and numeric values.
        1 ... 5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn match_char() {

    let x = 'c';

    match x {
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// Destructuring

struct Point {
    x: i32,
    y: i32,
}

fn destructuring() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}