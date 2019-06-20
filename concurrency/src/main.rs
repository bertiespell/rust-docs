use std::thread;
use std::time::Duration;

fn main2() {
    let handle = thread::spawn(|| {
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
fn main() {
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