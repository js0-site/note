# Main - Linkme Hook Demonstration

This is the main demonstration project that showcases the full power of linkme's distributed slice functionality for cross-crate hook registration and execution.

## Environment Information

- **Rust Version**: 1.75.0 (or later)
- **Rust Toolchain Triple**: x86_64-apple-darwin (macOS)
- **Operating System**: macOS Darwin 25.1.0
- **tokio Version**: 1.48.0
- **futures-util Version**: 0.3

## Overview

This project demonstrates a complete hook system built with linkme, featuring:

1. **Cross-crate Value Collection**: Collecting values from multiple crates into distributed slices
2. **Synchronous Hook Execution**: Running registered sync functions
3. **Asynchronous Hook Execution**: Running registered async functions with tokio
4. **Dynamic Hook Invocation**: Demonstrating runtime hook execution patterns
5. **Concurrent Async Execution**: Running multiple async hooks concurrently

## Architecture

The project consists of three crates:

1. **example_array**: Defines distributed slices for collecting values
2. **example_link**: Defines distributed slices for function registration
3. **main**: Demonstrates usage of all distributed slices and executes hooks

## Key Features

### Value Collection

The main crate can access values registered in other crates:

```rust
use example_array::{STRING_VALUES, INTEGER_VALUES, CUSTOM_DATA};

// Access all registered strings
for s in STRING_VALUES {
    println!("String: {}", s);
}

// Access all registered integers
for i in INTEGER_VALUES {
    println!("Integer: {}", i);
}

// Access all registered custom data
for data in CUSTOM_DATA {
    println!("Data: {:?}", data);
}
```

### Hook Registration

Functions are registered across different crates using linkme:

```rust
use linkme::distributed_slice;
use example_link::{HOOK_FUNCTIONS, ASYNC_HOOK_FUNCTIONS};

// Register sync hook
#[distributed_slice(HOOK_FUNCTIONS)]
fn my_sync_hook(input: &str) -> String {
    format!("Processing: {}", input)
}

// Register async hook
#[distributed_slice(ASYNC_HOOK_FUNCTIONS)]
fn my_async_hook() -> Pin<Box<dyn std::future::Future<Output = String> + Send>> {
    Box::pin(async {
        // Async operations
        "Async result".to_string()
    })
}
```

### Hook Execution

The main crate demonstrates different execution patterns:

```rust
// Sequential sync execution
for hook_fn in HOOK_FUNCTIONS.iter() {
    let result = hook_fn(input);
    println!("Result: {}", result);
}

// Concurrent async execution
let futures: Vec<_> = ASYNC_HOOK_FUNCTIONS.iter()
    .map(|async_fn| async_fn())
    .collect();
let results = futures_util::future::join_all(futures).await;
```

## Running the Demonstration

To run the complete demonstration:

```bash
cd main
cargo run
```

This will:
1. Display all collected values from example_array
2. Execute all registered synchronous hooks
3. Execute all registered asynchronous hooks
4. Demonstrate dynamic hook execution patterns

## Benefits Demonstrated

1. **Zero-cost Plugin Architecture**: No runtime initialization required
2. **Cross-crate Integration**: Seamless registration across crate boundaries
3. **Type Safety**: Compile-time verification of all registrations
4. **Async Support**: Full support for tokio async functions
5. **Concurrent Execution**: Efficient parallel execution of async hooks
6. **Dynamic Discovery**: Runtime discovery of all registered components

## Use Cases

This pattern is particularly useful for:
- Plugin systems
- Middleware registration
- Event handling systems
- Test framework extensions
- Configuration-driven architectures
- Modular application architectures