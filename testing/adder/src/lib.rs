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

// Any arguments specified after the one required argument to assert! or the two required arguments to assert_eq! and assert_ne! are passed along to the format! macro ===> Use this for specific error messages

pub fn greeting(name: &str) -> String {
    format!("Hello")
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
}

// can test things panic with - should_panic attribute

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value,
        }
    }
}

#[cfg(test)]
mod tests_again {
    use super::*;

    #[test]
    #[should_panic] // need this attribute to test that it panics!
    fn greater_than_100() {
        Guess::new(200);
    }
}

// Tests that use should_panic can be imprecise because they only indicate that the code has caused some panic. A should_panic test would pass even if the test panics for a different reason from the one we were expecting to happen. To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute. The test harness will make sure that the failure message contains the provided text.

// Let's test more specific panic cases

#[cfg(test)]
mod test_further_panic {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}