use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

/// This code demonstrates the compilation error that occurs when trying to send
/// a MutexGuard across threads without the send_guard feature enabled.
///
/// The parking_lot crate's MutexGuard is not Send by default, which means
/// it cannot be transferred between threads. This is a safety feature to
/// prevent potential deadlocks and race conditions.
pub fn attempt_send_guard_without_feature() {
    println!("=== Attempting to Send Guard Without send_guard Feature ===");

    let counter = Arc::new(Mutex::new(0i32));

    // This will cause a compilation error because MutexGuard is not Send
    // without the send_guard feature
    let (sender, receiver) = std::sync::mpsc::channel();

    let counter_clone = Arc::clone(&counter);
    let handle1 = thread::spawn(move || {
        println!("Thread 1: Acquiring lock...");
        let guard = counter_clone.lock();
        println!("Thread 1: Lock acquired, current value: {}", *guard);

        // ERROR: This line will cause a compilation error!
        // MutexGuard does not implement Send without the send_guard feature
        sender.send(guard).expect("Failed to send guard");
        //           ^^^^^
        // Error: `parking_lot::MutexGuard<'_, i32>` cannot be sent between threads safely

        println!("Thread 1: This line will never be reached due to compilation error");
    });

    let handle2 = thread::spawn(move || {
        println!("Thread 2: Waiting to receive guard...");
        let mut guard = receiver.recv().expect("Failed to receive guard");
        println!("Thread 2: Guard received, current value: {}", *guard);
        *guard += 42;
        println!("Thread 2: Value updated to: {}", *guard);
    });

    handle1.join().expect("Thread 1 panicked");
    handle2.join().expect("Thread 2 panicked");

    let final_value = *counter.lock();
    println!("Final counter value: {}", final_value);
}

/// Alternative approach that works without send_guard feature:
/// Instead of sending the guard, we send the data and re-acquire the lock
pub fn working_alternative_without_send_guard() {
    println!("\n=== Working Alternative Without send_guard Feature ===");

    let counter = Arc::new(Mutex::new(0i32));
    let counter_clone = Arc::clone(&counter);

    // Instead of sending the guard, we work with the data directly
    let (sender, receiver) = std::sync::mpsc::channel();

    let handle1 = thread::spawn(move || {
        println!("Thread 1: Acquiring lock...");
        {
            let mut guard = counter_clone.lock();
            println!("Thread 1: Lock acquired, current value: {}", *guard);
            *guard += 10;
            println!("Thread 1: Updated value to: {}", *guard);
            // Guard is dropped here, releasing the lock
        }

        // Send a signal to the other thread instead of the guard
        sender.send(()).expect("Failed to send signal");
        println!("Thread 1: Signal sent to thread 2");
    });

    let counter_clone2 = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        println!("Thread 2: Waiting for signal...");
        receiver.recv().expect("Failed to receive signal");

        // Re-acquire the lock in this thread
        let mut guard = counter_clone2.lock();
        println!("Thread 2: Lock re-acquired, current value: {}", *guard);
        *guard += 32;
        println!("Thread 2: Updated value to: {}", *guard);
    });

    handle1.join().expect("Thread 1 panicked");
    handle2.join().expect("Thread 2 panicked");

    let final_value = *counter.lock();
    println!("Final counter value: {}", final_value);
    assert_eq!(final_value, 42);

    println!("=== Alternative approach completed successfully! ===");
}

fn main() {
    // This function will cause a compilation error
    attempt_send_guard_without_feature();

    // This function shows a working alternative
    working_alternative_without_send_guard();
}
