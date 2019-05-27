use std::fs::File;
use std::io;
use std::io::Read;
use std::fs;

fn main() {
    // unwrap returns the result if it's there - othewise it panics
    let f = File::open("hello.txt").unwrap();

    // expect let's us choose the error message
    let f = File::open("hello.txt").expect("Oh no, no file"); // easier to find error messages

    // propogating errors

    // when writing a function whose implementation calls something that might fail, instead of handling the error within thi function, you can return the error to the calling code so that it can decide what to do

    // this is known as PROPOGATING the error - gives more error to the calling code
}

// A function that reads a username from a file. If the file doesn’t exist or can’t be read, this function will return those errors to the code that called this function: 

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file, // if this is okay f becomes a file
        Err(e) => return Err(e), // returns the error
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s), // returns a string
        Err(e) => Err(e), // returns an error
    }
}

// This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

// The ? at the end of the File::open call will return the value inside an Ok to the variable f. If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code. The same thing applies to the ? at the end of the read_to_string call.

fn refactored_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // note the ?
    // works in the same way as match
    
    // if the value of Result is okay this will be returned FROM THE EXPRESSION and the program will continue. If it is an Error this will be returned from the whole function as if we had used the return keyword (so the error gets propogated to the calling code)

    let mut s = String::new();
    f.read_to_string(&mut s)?; // note the ?
    Ok(s)
}

// There is a difference between the match expression and the ?

// Error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. 

// When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function.

// As long as each error type implements the from function to define how to convert itself to the returned error type, the ? operator takes care of the conversion automatically.

fn re_refactored_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; // chaining
    Ok(s)
}

fn re_re_refactored_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// The ? operator can only be used in functions that have a return type of Result, because it is defined to work in the same way as the match expression