use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // at this point the type is inferred
    scores.insert(String::from("Yellow"), 50);

    // data is stored on the heap
    // homogenous all keys and all values must have the same type

    // you can make a hashmap by collecting on a vector of tuples
    // where each tuple consists of a key and its value
    // collect method gathers data into a number of collection types

    let teams = vec!(String::from("Blue"), String::from("Yellow"));
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures and Rust doesn’t know which you want unless you specify.

    let team_name = String::from("Blue");
    let getScore = scores.get(&team_name); // doesn't have to be the same reference as that used to create it
    println!("{:?}", getScore);

    for (key, value) in &scores { // why does &scores and scores do the same here?
        println!("{}, {}", key, value);
    }

    // overwriting a key

    scores.insert(String::from("Blue"), 100); // overwrites it!
    println!("{:?}", scores);

    // only inserting a key if it has new value

    scores.entry(String::from("Blue")).or_insert(1500); // this never gets added as key exists
    scores.entry(String::from("Green")).or_insert(20); // this inserts a new key and adds
    println!("{:?}", scores);

    // Updating an old value based on a new value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // only inserts the word with 0 if it's not there already. Otherwise. It returns the entry but does not insert // actually returns a mutable reference
        *count += 1; // since we have the entry we can increment it - but first we have to dereference it
    } // mutable reference goes out of scope here... so it's safe!

    // Hashmap uses a cryptographically strong hashmap - resists DOS attacks - not fastest but better security

    println!("{:?}", map);
}
