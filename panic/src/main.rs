fn main() {
    // if you need to make the resulting binary as small as possible - you need to make changes to the toml file
    // panic!("Crash and burn!");

    // use panic backtrace
    let v = vec![1, 2, 3];

    v[99]; // C would just give you this. It's called a buffer overread - it can cause security vulnerabilities

    // to find the error in the terminal run
    // $ RUST_BACKTRACE=1 cargo run

    //  In order to get backtraces with this information, debug symbols must be enabled. Debug symbols are enabled by default when using cargo build or cargo run without the --release flag, as we have here.
}
