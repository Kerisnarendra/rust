use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();

    println!("{:?}", my_mutex);
    println!("{:?}", mutex_changer);
    *mutex_changer = 6;
    println!("{:?}", mutex_changer);

    let mut other_mutex_changer = my_mutex.try_lock();

    if let Ok(value) = other_mutex_changer {
        println!("The MutexGuard has: {}", value);
    } else {
        println!("Didn't get the lock");
    }
    std::mem::drop(mutex_changer);

    other_mutex_changer = my_mutex.try_lock();

    if let Ok(value) = other_mutex_changer {
        println!("The MutexGuard has: {}", value);
    } else {
        println!("Didn't get the lock");
    }

    println!("{:?}", my_mutex);
}

// Result will be:
// Mutex { data: <locked>, poisoned: false, .. }
// 5
// 6
// Didn't get the lock
// The MutexGuard has: 6
// Mutex { data: 6, poisoned: false, .. }
