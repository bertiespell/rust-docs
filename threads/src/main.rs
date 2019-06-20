use std::sync::mpsc; //  multiple producer, single consumer.
use std::thread;

fn main1() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // The transmitting end has a send method that takes the value we want to send. The send method returns a Result<T, E> type, so if the receiving end has already been dropped and there’s nowhere to send a value, the send operation will return an error. In this example, we’re calling unwrap to panic in case of an error. But in a real application, we would handle it properly
    });

    let received = rx.recv().unwrap(); // The receiving end of a channel has two useful methods: recv and try_recv. We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel. Once a value is sent, recv will return it in a Result<T, E>. When the sending end of the channel closes, recv will return an error to signal that no more values will be coming.

    /**
     * The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time. Using try_recv is useful if this thread has other work to do while waiting for messages: we could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other work for a little while until checking again.
     */
    println!("Got: {}", received);
}

// An example of how ownership rules prevent unsafe code - uncommented this wouldn't compile

fn main2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // trying to use val after we've sent it down the channel - won't compile!
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx { // In the main thread, we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator. For each value received, we’re printing it. When the channel is closed, iteration will end.
        println!("Got: {}", received);
    }

    // Because we don’t have any code that pauses or delays in the for loop in the main thread, we can tell that the main thread is waiting to receive values from the spawned thread.
}