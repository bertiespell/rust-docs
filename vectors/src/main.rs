fn main() {
    let v: Vec<i32> = Vec::new(); // yay a vec
    let v2 = vec!(1, 2, 3);
    // Q: what is the difference between the above and
    let v3 = vec![1, 2, 3];
    println!("{:?}, {:?}", v2, v3);

    let mut mutable_vec = Vec::new();
    mutable_vec.push(1); // as with any variable, if we want to make it mutable we have to change its value

    let third: &i32 = &v3[2];
    match v3.get(2) { // a get gives us an Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("Nothing found!"),
    }

    loopy();
} 
// vectors (and their contents!) are freed when they go out of scope
// this gets complicated when you are using references to the elements of vectors

// This code will not compile

fn test() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // this is an immutable reference

    // v.push(6); // but then here we're trying to push (mutably) to the vec - this would break

    println!("The first element is: {}", first);
}

fn loopy() {

    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }

    let mut muty_vec = vec!(1, 2, 3);
    for i in &mut muty_vec {
        *i += 1; // to change the value that the mutable reference refers to we need to use the dereference operator - this actually gets the value in i!
    }
}

// when things need to be different types - use an enum

fn enumy_things() {
    // We can define an enum whose variants will hold the different value types, and then all the enum variants will be considered the same type: that of the enum
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(2.4),
        SpreadsheetCell::Text(String::from("Hello")),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// This pattern of using enums to specify different types in vecs is useful (maintains safety, and lets you do maps and other operations on vecs, using exhaustive type checking)
// But if you don't know all of the types at compile time - then you'll need to use traits