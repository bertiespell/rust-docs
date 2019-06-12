// Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re called. 
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

// This code works the way the business wants it to now, but let’s say the data science team decides that we need to make some changes to the way we call the simulated_expensive_calculation function in the future. To simplify the update when those changes happen, we want to refactor this code so it calls the simulated_expensive_calculation function only once. We also want to cut the place where we’re currently unnecessarily calling the function twice without adding any other calls to that function in the process. That is, we don’t want to call it if the result isn’t needed, and we still want to call it only once.

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity); // This change unifies all the calls to simulated_expensive_calculation and solves the problem of the first if block unnecessarily calling the function twice. Unfortunately, we’re now calling this function and waiting for the result in all cases, which includes the inner if block that doesn’t use the result value at all.
    
    // TIME TO USE CLOSURE: We want to define code in one place in our program, but only execute that code where we actually need the result. This is a use case for closures!

    // Instead of always calling the simulated_expensive_calculation function before the if blocks, we can define a closure and store the closure in a variable rather than storing the result of the function call

    // We can actually move the whole body of simulated_expensive_calculation within the closure we’re introducing here:

    let expensive_closure = |num: i32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }; // Note that this let statement means expensive_closure contains the definition of an anonymous function, not the resulting value of calling the anonymous function. Recall that we’re using a closure because we want to define the code to call at one point, store that code, and call it at a later point; the code we want to call is now stored in expensive_closure.

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity) // Low-intensity workout plans will recommend a number of push-ups and sit-ups based on the complex algorithm we’re simulating.
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
