// slice is another data type which doesn't have ownership

// refers to contiguous sequence of elements in a collection

// write a function that takes a string and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
fn main() {
    slice_example();
    let search_word = String::from("Hello world");
    let found1 = refactored_first_words(&search_word);
    println!("Found1: {}", found1);
    string_literal_example();
    array_slice();
}

fn first_words(s: &String) -> usize { // returning usize on its own BUT...
    // ... it’s only a meaningful number in the context of the &String
    let bytes = s.as_bytes(); // creates as array of bytes

    for (i, &item) in bytes.iter().enumerate() { // create an iterator over an array of bytes
        // iter is a method that returns each element in a collection
        // enumerate wraps the result of iter and returns each element as part of a tupel instead
        if item == b' ' {
            return i; // it returns the index if it finds a blank space

            // if we use i somewhere after the initial string hasn been cleared this would be a bug.. instead we can use slices
        }
    };

    s.len() // otherwise it's one word and it returns this length
}

fn refactored_first_words(myString: &String) -> &str { // this signifies a string slice! -> &str
    let bytes = myString.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &myString[0..i];
        }
    }

    &myString[..]
}

fn slice_example() {
    let s = String::from("hello");

    let slice = &s[2..4]; // from 2 (exclusive of 4)
    println!("Slice one {}", slice);
    let slice2 = &s[..3]; // exclusive of 3
    println!("Slice two {}", slice2);
    let slice3 = &s[2..]; // includes 2
    println!("Slice three {}", slice3);

    // or you can drop both to take a slice of the entire string
    let entire_slice = &s[..];
    println!("Entire slice {}", entire_slice);
}

// Lesson: 
// Slices are harder to mess up - the Rust compiler will help you spot bugs early because a slice is a reference, linked to the data below it. This means that Rust can tell you and throw errors if that data changes or is emptied out, but you're still tyring to use part of a slice.

// example of something that a slice not protects against

fn bad() {
    let s = String::from("Hello");

    let first_word = refactored_first_words(&s); // this is an immutable 

    // s.clear(); // fails cannot borrow as mutable!!!
}

fn string_literal_example() {
    let s = "hello"; // the type here is &str (it is a slice pointing to a specific point in the binary). This is also why string literals are immutable!
    // &str is an immutable reference!

    // you can take slices of literals and String values
    // you can refactor AGAIN - so that you can use the method on both string literals AND string values
    let proper_string = String::from("Hello again");

    let found1 = re_refactored_first_word(&s);
    let found2 = re_refactored_first_word(&proper_string); // Q: why don't I need to do &proper_string[..] to make it a SLICE rather than just a reference?

    println!("Found {} and {}", found1, found2);
}

fn re_refactored_first_word(my_string: &str) -> &str {
    let bytes = my_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("ITEM: {}", item);
            return &my_string[0..i];
        }
    }

    &my_string[..]
}

// OTHER TYPES OF SLICES
// e.g. &[i32]

// Remember: slices work by storing a reference to the first element, and then a length

fn array_slice() {
    let a = [6, 9, 3, 4, 5];
    let little_slice = &a[..2]; // nifty!
    for (item) in little_slice.iter() {
        println!("My little slice {} {}", item, &item);
    }
}