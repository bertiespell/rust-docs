use std::error::Error;
use std::fs;
use std::env;
use std::io::prelude::*; // contains various useful traits for doing I/O, including file I/O.. In the same way that Rust has a general prelude that brings certain types and functions into scope automatically, the std::io module has its own prelude of common types and functions you’ll need when working with I/O. Unlike with the default prelude, we must explicitly add a use statement for the prelude from std::io.

// stdout vs stderr
// At the moment, we’re writing all of our output to the terminal using the println! function. Most terminals provide two kinds of output: standard output (stdout) for general information and standard error (stderr) for error messages. This distinction enables users to choose to direct the successful output of a program to a file but still print error messages to the screen.

// currently, running 

// $ cargo run > output.txt

// this will put error messages in the file - which is likely not what we want. This is because we are currently sending error messages to stdout

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // Box<dyn Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases. This is what the dyn means, it's short for "dynamic."
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone(); // There’s a tendency among many Rustaceans to avoid using clone to fix ownership problems because of its runtime cost.
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    Config::new(args)
    // (query, filename) // we could put the two values into one struct and give each of the struct fields a meaningful name. Doing so will make it easier for future maintainers of this code to understand how the different values relate to each other and what their purpose is.
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { // Notice that we need an explicit lifetime 'a defined in the signature of search and used with the contents argument and the return value. Recall in Chapter 10 that the lifetime parameters specify which argument lifetime is connected to the lifetime of the return value. In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // the same just calls to_lowercase
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}