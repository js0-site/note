# Parking Lot Without Send Guard Feature - Error Demonstration

This project demonstrates the compilation errors that occur when attempting to send `MutexGuard` objects across threads without the `send_guard` feature enabled in the `parking_lot` crate.

## Environment Information

- **Rust Version**: 1.94.0-nightly (37aa2135b 2025-12-08)
- **Rust Toolchain Triple**: aarch64-apple-darwin
- **Operating System**: macOS 26.1 (Build 25B78)
- **LLVM Version**: 21.1.5

## Expected Compilation Error

When running `cargo build` or `cargo run`, you will encounter the following error:

```
error[E0277]: `parking_lot::MutexGuard<'_, i32>` cannot be sent between threads safely
   --> src/lib.rs:XX:XX
    |
XX |         sender.send(guard).expect("Failed to send guard");
    |                     ^^^^^ `parking_lot::MutexGuard<'_, i32>` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for `parking_lot::MutexGuard<'_, i32>`
```

## Why This Error Occurs

By default, `parking_lot::MutexGuard` does not implement the `Send` trait, which means it cannot be safely transferred between threads. This is a deliberate design choice to prevent:

1. **Deadlocks**: Sending guards between threads can lead to complex deadlock scenarios
2. **Lock Ordering Issues**: Guards held across thread boundaries can violate lock ordering
3. **Performance Issues**: Thread migration of guards can impact performance

## Alternative Approaches

The code also demonstrates working alternatives that don't require the `send_guard` feature:

1. **Lock and Release Pattern**: Acquire lock, modify data, release lock, then signal other threads
2. **Data Copying**: Copy the data instead of transferring the guard
3. **Message Passing**: Use channels to coordinate between threads without sharing guards

## Enabling the Feature

To fix the compilation error, you would need to add the `send_guard` feature to your `Cargo.toml`:

```toml
[dependencies]
parking_lot = { version = "0.12", features = ["send_guard"] }
```

However, this project intentionally omits this feature to demonstrate the error.