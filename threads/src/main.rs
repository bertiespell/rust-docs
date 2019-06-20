use std::sync::mpsc; //  multiple producer, single consumer.
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // The transmitting end has a send method that takes the value we want to send. The send method returns a Result<T, E> type, so if the receiving end has already been dropped and there’s nowhere to send a value, the send operation will return an error. In this example, we’re calling unwrap to panic in case of an error. But in a real application, we would handle it properly
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
