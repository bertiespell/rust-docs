pub fn hello_string() {
    let _s = "hello"; // this is a string literal (stack?) - it is immutable
    let mut _s = String::from("hello"); // this is allocated on the HEAP - stores an amount of text unknown at compile time
    _s.push_str(" world");
    println!("{}", _s);

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
