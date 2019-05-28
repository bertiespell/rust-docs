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
