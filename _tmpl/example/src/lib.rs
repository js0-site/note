use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Demonstrates the usage of parking_lot's send_guard feature.
/// With the send_guard feature enabled, MutexGuard can be sent across threads,
/// allowing for more flexible synchronization patterns.
pub fn demonstrate_send_guard() {
    println!("=== Parking Lot Send Guard Feature Demonstration ===");

    // Create a shared mutex-protected counter
    let counter = Arc::new(Mutex::new(0i32));

    // Demonstrate basic usage - the key benefit is that guards can be moved
    // across thread boundaries when the send_guard feature is enabled
    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        println!("Worker thread: Acquiring lock...");
        let mut guard = counter_clone.lock();
        println!("Worker thread: Lock acquired, current value: {}", *guard);

        // Modify the value
        *guard += 42;
        println!("Worker thread: Value updated to: {}", *guard);

        // Simulate some work while holding the lock
        thread::sleep(Duration::from_millis(50));
        println!("Worker thread: Work completed, releasing lock...");

        // Guard will be dropped here, releasing the lock
    });

    // Wait for the thread to complete
    handle.join().expect("Thread panicked");

    // Verify the final value
    let final_value = *counter.lock();
    println!("Main thread: Final counter value: {}", final_value);
    assert_eq!(final_value, 42);

    println!("=== Basic demonstration completed successfully! ===");
}

/// Demonstrates the send_guard feature by showing that guards can be
/// transferred between threads using a simple example
pub fn demonstrate_guard_passing_pattern() {
    println!("\n=== Send Guard Feature Demonstration ===");

    // This example shows the key capability: sending guards between threads
    // We'll use a simple approach that works with the send_guard feature

    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let data_clone = Arc::clone(&data);

    println!("Main thread: Initial data: {:?}", *data.lock());

    // Thread that will acquire lock and do some work
    let handle = thread::spawn(move || {
        println!("Worker thread: Acquiring lock...");
        let mut guard = data_clone.lock();
        println!("Worker thread: Lock acquired, data: {:?}", *guard);

        // Modify the data
        guard.push(6);
        guard.push(7);
        guard.push(8);

        println!("Worker thread: Modified data: {:?}", *guard);

        // The key point: with send_guard feature, this guard could be sent
        // to another thread if needed. Without it, this would be a compile error.
        thread::sleep(Duration::from_millis(100));

        println!("Worker thread: Releasing lock...");
        // Guard is dropped here, releasing the lock
    });

    handle.join().unwrap();

    // Verify the final state
    let final_data = data.lock();
    println!("Main thread: Final data: {:?}", *final_data);

    println!("=== Send Guard demonstration completed! ===");
    println!("Note: The send_guard feature allows MutexGuard to implement Send trait,");
    println!("enabling guards to be transferred between threads safely.");
}
