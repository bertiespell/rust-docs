fn main() {
    match_names_variables();
    multiple_patterns();
    match_range();
    destructuring();
    use_literal_destructuring();
    destructure_enum();
    nested_enum_matching();
    ignore_things();
    ignore_nested_parts();
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

fn shorthand_destructuing_with_matching_var_names() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p; // shorter - but the same
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn use_literal_destructuring() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y), // we match this case!
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructure_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}

// NESTED destructuring - Matching can work on nested structures too!

enum Color {
   Rgb(i32, i32, i32),
   Hsv(i32, i32, i32)
}

enum NestedMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn nested_enum_matching() {
    let msg = NestedMessage::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        NestedMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )     
        },
        NestedMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }
}

// When the value we’re matching to our pattern contains a reference, we need to destructure the reference from the value, which we can do by specifying a & in the pattern. Doing so lets us get a variable holding the value that the reference points to rather than getting a variable that holds the reference. This technique is especially useful in closures where we have iterators that iterate over references, but we want to use the values in the closure rather than the references.

fn destructuing_references() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();
}

// Destructuing structs and Tuples
// We can mix, match, and nest destructuring patterns in even more complex ways. The following example shows a complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out:

fn destructure_complex() {
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
}

// Ignore values or ranges with _ or ... 

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn ignore_things() {
    foo(3, 4);
}

// In most cases when you no longer need a particular function parameter, you would change the signature so it doesn’t include the unused parameter. Ignoring a function parameter can be especially useful in some cases, for example, when implementing a trait when you need a certain type signature but the function body in your implementation doesn’t need one of the parameters. The compiler will then not warn about unused function parameters, as it would if you used a name instead.

// you can also ignore PARTS of a value using a NESTED _

fn ignore_nested_parts() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}