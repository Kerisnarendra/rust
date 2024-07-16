use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub fn main() {
    // Create a channel for communication
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();

    // Spawn a new thread to send data
    thread::spawn(move || {
        for i in 1..5 {
            tx.send(i).unwrap_or_else(|error| {
                println!("Error sending data: {}", error);
            });
            println!("Send: {}", i);
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // Receive data in the main thread
    for _ in 1..5 {
        let received = rx.recv().unwrap_or_else(|error| {
            println!("Error receiving data: {}", error);
            // Return a default value or handle the error in another way
            0
        });
        println!("Received: {}", received);
    }

    println!("Main thread finished!");
}

// Result will be:
// Received: 1
// Send: 1
// Send: 2
// Received: 2
// Send: 3
// Received: 3
// Send: 4
// Received: 4
// Main thread finished!
