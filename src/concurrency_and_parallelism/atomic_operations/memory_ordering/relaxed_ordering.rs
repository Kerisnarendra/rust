use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    // Create an atomic boolean flag
    let flag = Arc::new(AtomicBool::new(false));

    // Clone the Arc for use in the closure
    let flag_clone = Arc::clone(&flag);

    // Create a closure that sets the flag to true with Relaxed ordering
    let set_flag = move || {
        flag_clone.store(true, Ordering::Relaxed);
    };

    // Spawn a new thread that sets the flag
    let handle = thread::spawn(set_flag);

    // Wait for the thread to finish
    handle.join().expect("Thread panicked");

    // Check the flag with Relaxed ordering
    let is_flag_set = flag.load(Ordering::Relaxed);

    if is_flag_set {
        println!("Flag is set!");
    } else {
        println!("Flag is not set!");
    }
}
