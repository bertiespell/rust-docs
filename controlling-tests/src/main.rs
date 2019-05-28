// Some command line options go to cargo test, and some go to the resulting test binary. To separate these two types of arguments, you list the arguments that go to cargo test followed by the separator -- and then the ones that go to the test binary

// when you run multiple tests, by default they run in parallel using threads

// this means they shouldn't depend on each other, or on any shared state/environment

// If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the --test-threads flag and the number of threads you want to use to the test binary

// $ cargo test -- --test-threads=1

// Showing function output
// Usually the test suite swallows this output (to avoid spamming the console)

// but this can be enabled - using --nocapture flag:
// $ cargo test -- --nocapture

// if the threads are running in parallel this output can be interweaved. If you need them in order then use the test-threads option
fn main() {
    println!("Hello, world!");
}

// can also run a subset of tests by name

// e.g.
// $ cargo test one_hundred

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

// running multiple (yet filtered) tests

// We can specify part of a test name, and any test whose name matches that value will be run. For example, because two of our tests’ names contain add, we can run those two by running cargo test add

// e.g. This would run the top two tests because they match
// $ cargo test add

// ignoring some tests unless specifically requested
// you can instead annotate the time-consuming tests using the ignore attribute to exclude them, as shown here:

#[test]
#[ignore]
fn expensive_test() {
    // code that takes ages to run and our test suite can't be bothered to run it everytime
}

// if we only want to run ignored tests: 
// $ cargo test -- --ignored