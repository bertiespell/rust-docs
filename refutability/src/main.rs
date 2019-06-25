// Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable. An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match. Patterns that can fail to match for some possible value are refutable. An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.

// an example of what happens when we try to use a refutable pattern where Rust requires an irrefutable pattern and vice versa
fn main() {
    //If some_option_value was a None value, it would fail to match the pattern Some(x), meaning the pattern is refutable. However, the let statement can only accept an irrefutable pattern because there is nothing valid the code can do with a None value
    // let Some(x) = Some(1); // THIS WILL NOT COMPILE
    // Err: ^^^^^^^ pattern `None` not covered
}
