// strings are implemented as a collection of bytes
// plus some methods to provide useful functionality

// The operations on String - WHICH EVERY COLLECTION TYPE HAS
// creating
// updating
// reading
fn main() {
    println!("Hello, world!");

    let s = String::from("hello");
    let string_literal = "hello";
    string_literal.to_string();
    let another_example = "hello".to_string();
    // strings are UTF-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let hello_2 = hello + &String::from(" world"); // the reason we need a & reference here, and the reason that hello wouldn't be valid after here is because of the signature of the ADD method on String
    // The + operator uses the add method whose signature looks like: 
    // fn add(self, s: &str) -> String { // etc.
    // even though here the s parameter  is a &str, we can still add a &String in because Rust uses a DEREF COERCION
    // this turns &String into &str by COERCING with a &s2[..]
    // println!("{}", hello); // NOT VALID
    // add does not take ownership os s, it will still be a valid String after this! (so the coercion is safe)
    // the signature DOES take ownership over self (hello in the example above)
    //
    println!("{}", hello_2);
    let another_cutie = String::from("erm...");
    let notTakingRef = String::from("thingy");
    let testing_second_thing = another_cutie + &notTakingRef;
    // can we now still use notTakingRef as it's a reference
    println!("{}", notTakingRef);

    let mut mutable_string = String::from("Hello");
    let another_string = String::from("Let's go");
    mutable_string.push_str("nope");
    let but_a_string_literal = "I am a literal little thing";
    mutable_string.push_str(&another_string);
    mutable_string.push_str(but_a_string_literal);
    println!("Let me still use it: {}, {}", another_string, but_a_string_literal);
    mutable_string.push('l'); // uses single quotes now!

    // all this get's pretty unweildy at some point
    // let's use the format! macro :)

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let formatted_string = format!("{}-{}-{}", s1, s2, s3); // similar to println! but returns a new String
    // easier to read!
    // doesn't take ownership of it's parameters!
    println!("{}", formatted_string);
}
