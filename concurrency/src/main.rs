use std::thread;
use std::time::Duration;

fn main2() {
    let handle = thread::spawn(|| { // we’re not using any data from the main thread in the spawned thread’s code (no args to the closure)... To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs. 
        for i in 1..10 { // this typically only gets to 5 before the main thread shut down (no guarantee)
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run.. The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads.
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish
    handle.join().unwrap();
}

// Switch this to the MAIN MAIN method to see handle block the thread - print to 9, BEFORE the next counting starts
fn main1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // implicit captures -  we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.
        println!("Here's a vector: {:?}", v); // without V being moved it errors... Rust infers how to capture v, and because println! only needs a reference to v, the closure tries to borrow v. However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid.
    });

    // drop(v); // WITHOUT move || in the closure above => oh no! This causes some serious issues... WITH move || in the closure => Compiler error, v was moved above :)

    handle.join().unwrap();
}