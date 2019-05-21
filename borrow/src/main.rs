fn main() {
    let s1 = String::from("Helllo");

    let len = calculate_length(&s1); // the &s1 syntax creates a reference that refers to s1 but does not own it. Because it does not own it, it cannot be dropped when it goes out of scope (at the end of &s1)

    println!("I can still use both {} {}", s1, len);
}

fn calculate_length(myString: &String) -> usize { // we call having references as function paramteres borrowing
    myString.len()
}

// opposite of referencing (using &) is deferencing (using *)

// what if we want to actually edit the content?

fn change_string(mystring: &mut String) {
    mystring.push_str("Add more");
}

fn test_change_str() {
    let mut s = String::from("A string"); // need to specify this as mutable

    change_string(&mut s);

    // BUT you can only have one mutable reference to a piece of data within a particular scope

    let r1 = &mut s;
    let r2 = &mut s; // Q: why does Rust even let you declare/assign this? If you can never use it?

    // println!("{}, {}", r1, r2); // this won't work

    // These same rules apply to mutable and immutable references

    let mut newString = String::from("aggggain");
    let z1 = &newString; // fine
    let z2 = &newString; // fine - multiple immutable references are fine as the underlying data isn't changing
    let z3 = &mut newString;// NOT fine

    // println!("{}, {}, {}", z1, z2, z3); // This is not cool!

}
