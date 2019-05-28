#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7};
        let smaller = Rectangle { width: 6, height: 3};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // We could also have non-test functions in the tests module to help set up common scenarios or perform common operations, so we need to indicate which functions are tests by using the #[test] attribute.
    fn not_a_test_for_example() {

    }

    #[test]
    fn another() {
        panic!("Lets fail"); // panic makes a test fail
    }

    // using other macros

    // The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true.
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
