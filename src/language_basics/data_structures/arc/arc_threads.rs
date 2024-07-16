use std::sync::{Arc, Mutex};

fn main() {
    let my_number = Arc::new(Mutex::new(0));

    let my_number1 = Arc::clone(&my_number);
    let my_number2 = Arc::clone(&my_number);

    let thread1 = std::thread::spawn(move || {
        // Only the clone goes into Thread 1
        for _ in 0..10 {
            *my_number1.lock().unwrap() += 1; // Lock the Mutex, change the value
        }
    });

    let thread2 = std::thread::spawn(move || {
        // Only the clone goes into Thread 2
        for _ in 0..10 {
            *my_number2.lock().unwrap() += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Value is: {:?}", my_number);
    println!("Exiting the program");
}

// Result will be:
// Value is: Mutex { data: 20, poisoned: false, .. }
// Exiting the program
