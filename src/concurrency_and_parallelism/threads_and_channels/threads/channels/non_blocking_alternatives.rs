use std::sync::mpsc::sync_channel;
use std::thread;

pub fn main() {
    // Create a sync channel with buffer size 1
    let (sync_sender, receiver) = sync_channel(1);
    let sync_sender2 = sync_sender.clone();

    // First thread onws sync_sender
    thread::spawn(move || {
        sync_sender.send(1).unwrap();
        sync_sender.send(2).unwrap();
        // Thread blocked
    });

    // Second thread owns sync_sender2
    thread::spawn(move || {
        let _ = sync_sender2.try_send(3);
    });

    let mut message;
    message = receiver.recv().unwrap();
    println!("message {message} received");

    message = receiver.recv().unwrap();
    println!("message {message} received");

    match receiver.try_recv() {
        Ok(message) => println!("message {message} received"),
        Err(_) => println!("the third message was never sent"),
    }
}

//  Output will be:
// message 1 received
// message 2 received
// the third message was never sent
