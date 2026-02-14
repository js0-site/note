# Parking Lot Send Guard Feature Example

This project demonstrates the correct usage of the `parking_lot` crate's `send_guard` feature, which allows `MutexGuard` objects to be sent across threads.

## Environment Information

- **Rust Version**: 1.94.0-nightly (37aa2135b 2025-12-08)
- **Rust Toolchain Triple**: aarch64-apple-darwin
- **Operating System**: macOS 26.1 (Build 25B78)
- **LLVM Version**: 21.1.5

## Features Demonstrated

1. **Basic Send Guard Usage**: Shows how to acquire a mutex guard in one thread and send it to another thread for processing.

2. **Advanced Guard Passing Pattern**: Demonstrates a producer-consumer pattern where the producer acquires a lock, modifies data, and passes the guard to a consumer thread for further processing.

## Key Benefits of Send Guard Feature

- **Thread Flexibility**: Allows mutex guards to be transferred between threads, enabling more sophisticated synchronization patterns.
- **Performance**: Reduces lock contention by allowing one thread to hold a lock while another thread processes the data.
- **Design Patterns**: Enables advanced patterns like producer-consumer with shared lock ownership.

## Running the Example

```bash
cargo run
```

This will execute both demonstration functions and show the successful usage of the send_guard feature.