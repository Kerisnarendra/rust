use std::thread;
use std::time::Duration;

pub fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Number form spawned thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Continue execution in the main thread
    for i in 1..3 {
        println!("Number from main thread: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();

    println!("Main thread finished executing");
}
