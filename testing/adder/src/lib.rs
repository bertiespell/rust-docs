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
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    // assert_eq! and assert_ne!—to perform this test more conveniently. These macros compare two arguments for equality or inequality, respectively

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
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

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits.

// Because PartialEq and Debug are derivable, this is *usually* as simple as adding #[derive(PartialEq, Debug)] to your code (i.e. for structs and enums that you define (All primitive types nad most standard library types implement these))