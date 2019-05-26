use std::fs::File;

fn main() {
    // unwrap returns the result if it's there - othewise it panics
    let f = File::open("hello.txt").unwrap();
}
