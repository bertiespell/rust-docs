fn main() {
    // you can iterate over strings in other ways though
    // performing operations on individual unicode scalar values
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // Unicode scalar values may be made up of more than 1 byte
}
