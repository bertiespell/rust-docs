use std::sync::{Mutex, Arc};
use std::thread;
use std::rc::Rc;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // This doesn't compile either! - std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely - the trait bound `Send` is not satisfied

        let handle = thread::spawn(move || {  
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main2() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || { // The error message states that the counter value is moved into the closure and then captured when we call lock. That description sounds like what we wanted, but it’s not allowed!... Rust is telling us that we can’t move ownership of counter into multiple threads. 
        // Instead we want multiple mutable references - a case for RC - BUT - RC is not safe across threads. So actually, we want Arc which is similar but safe across multiple threads.
            // let mut num = counter.lock().unwrap();
            // *num += 1; // When a thread finishes running its closure, num will go out of scope and release the lock so another thread can acquire it.
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // we call join on each handle to make sure all the threads finish.
    }

    println!("Result: {}", *counter.lock().unwrap()); // At that point, the main thread will acquire the lock and print the result of this program.
}

fn main1() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    
    println!("m = {:?}", m);
}
