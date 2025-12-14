# Example Link

This example demonstrates advanced usage of linkme's distributed slice functionality for registering and calling functions across different crates, including async functions with tokio runtime.

## Environment Information

- **Rust Version**: 1.75.0 (or later)
- **Rust Toolchain Triple**: x86_64-apple-darwin (macOS)
- **Operating System**: macOS Darwin 25.1.0
- **linkme Version**: 0.3.35
- **tokio Version**: 1.48.0

## Overview

This crate demonstrates:

1. **Function Registration**: Registering synchronous functions in distributed slices
2. **Async Function Registration**: Registering async functions with tokio runtime
3. **Cross-crate Value Contribution**: Adding values to distributed slices defined in other crates
4. **Dynamic Function Invocation**: Calling registered functions at runtime

## Key Components

### Distributed Slices

- `HOOK_FUNCTIONS`: Stores synchronous function pointers with signature `fn(&str) -> String`
- `ASYNC_HOOK_FUNCTIONS`: Stores async function pointers returning `Pin<Box<dyn Future<Output = String> + Send>>`

### Function Types

#### Synchronous Functions

```rust
#[distributed_slice(HOOK_FUNCTIONS)]
fn my_function(name: &str) -> String {
    format!("Processing: {}", name)
}
```

#### Asynchronous Functions

```rust
#[distributed_slice(ASYNC_HOOK_FUNCTIONS)]
fn async_function() -> Pin<Box<dyn std::future::Future<Output = String> + Send>> {
    Box::pin(async {
        // Async operations here
        "Result".to_string()
    })
}
```

## Usage Examples

### Registering Functions

Functions can be registered using the `#[distributed_slice(...)]` attribute:

```rust
use linkme::distributed_slice;
use std::pin::Pin;

// Define the distributed slice
#[distributed_slice]
pub static HOOK_FUNCTIONS: [fn(&str) -> String] = [..];

// Register a function
#[distributed_slice(HOOK_FUNCTIONS)]
fn my_hook(name: &str) -> String {
    format!("Hook called with: {}", name)
}
```

### Invoking Registered Functions

```rust
// Call all registered sync functions
for hook_fn in HOOK_FUNCTIONS {
    let result = hook_fn("test_input");
    println!("Result: {}", result);
}

// Call all registered async functions
for async_fn in ASYNC_HOOK_FUNCTIONS {
    let future = async_fn();
    let result = future.await;
    println!("Async result: {}", result);
}
```

### Cross-crate Integration

This crate also demonstrates contributing values to distributed slices defined in other crates:

```rust
use example_array::{STRING_VALUES, INTEGER_VALUES, CUSTOM_DATA, CustomData};

// Add values to slices defined in example_array crate
#[distributed_slice(STRING_VALUES)]
static MY_STRING: &str = "From example_link";

#[distributed_slice(INTEGER_VALUES)]
static MY_INT: i32 = 42;

#[distributed_slice(CUSTOM_DATA)]
static MY_DATA: CustomData = CustomData { /* ... */ };
```

## Benefits

1. **Plugin Architecture**: Enables plugin-like functionality without complex registration code
2. **Zero-cost Abstraction**: No runtime overhead for function registration
3. **Type Safety**: Compile-time checking ensures function signatures match
4. **Async Support**: Full support for async functions with tokio runtime
5. **Cross-crate Integration**: Functions and values can be registered from any crate in the dependency graph