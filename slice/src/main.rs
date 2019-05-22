// slice is another data type which doesn't have ownership

// refers to contiguous sequence of elements in a collection

// write a function that takes a string and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
fn main() {
    slice_example();
    let search_word = String::from("Hello world");
    let found1 = refactored_first_words(&search_word);
    println!("Found1: {}", found1);
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