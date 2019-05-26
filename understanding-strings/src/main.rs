fn main() {
    let string_literal = "My little string"; // this is an str
    let mut mutable_string_literal = "My other little string"; // this is an str
    let heap_string = String::from("Hello again, little world");
    let mut mutable_heap_string  = String::from("What's the difference?");
    println!("{}, {}, {}, {}", string_literal, mutable_string_literal, heap_string, mutable_heap_string);

    let a: String = "hello rust".into(); // converts an str into <T>

    mutable_string_literal.trim();
    mutable_heap_string.trim();

    // Try passing them directly: When method expects a reference and gets ownership

    // grow_string(string_literal); // Err: mistmatched types: str != &String
    // grow_string(mutable_string_literal); // Err: mistmatched types: str != &String
    // grow_string(heap_string); // Err: expected reference
    // grow_string(mutable_heap_string); // Err: expected reference

    // Try adding references: When the method takes &String only the strings work

    // grow_string(&string_literal); // Err: mistmatched types: &str != &String
    // grow_string(&mutable_string_literal); // Err: mistmatched types: &str != &String
    grow_string(&heap_string); // yay
    grow_string(&mutable_heap_string); // yay

    // Try when the method takes &str they all work
    grow_string_ref(&string_literal); // yay
    grow_string_ref(&mutable_string_literal); // yay
    grow_string_ref(&heap_string); // yay
    grow_string_ref(&mutable_heap_string); // yay

    // When method expects ownership of a string
    // my_ownable_string(string_literal); // nope! This is an str
    // my_ownable_string(mutable_string_literal); // nope! This is an str
    my_ownable_string(heap_string); // yay
    my_ownable_string(mutable_heap_string); // yay
}

fn grow_string(my_growable_string: &String) -> &str {
    let trimmed_string = my_growable_string.trim();
    trimmed_string
}

fn my_ownable_string(my_owned_string: String) -> String {
    my_owned_string
}

fn return_string(my_growable_string: &String) -> String {
    let ooooo_cloned = my_growable_string.clone();
    ooooo_cloned // aha! We got an owned string
}


fn grow_string_ref(my_growable_string: &str) -> &str {
    let trimmed_string = my_growable_string.trim();
    trimmed_string
}

fn transforming_strings(my_growable_string: &str) -> String {
    let new_string = my_growable_string.to_string(); // gives an OWNED reference
    new_string
}

// fn back_the_other_way(stringy: String) -> &str { // this doesn't compile - missing lifetime specifier!
//     &stringy
// }

fn back_the_other_way(stringy: &String) -> &str { // but this does
    &stringy
}

// This errors - because the size can't be known at compile time

// fn interacting_with_str(stringy: str) -> &str { 
    // ERR: the size for values of type `str` cannot be known at compilation time 
    // ALSO ERR (on return type): Missing lifetime specifier
    // &stringy
// }