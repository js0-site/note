# Example Array

This example demonstrates the usage of linkme's distributed slice functionality for collecting array elements across different crates.

## Environment Information

- **Rust Version**: 1.75.0 (or later)
- **Rust Toolchain Triple**: x86_64-apple-darwin (macOS)
- **Operating System**: macOS Darwin 25.1.0
- **linkme Version**: 0.3.35

## Overview

This crate defines distributed slices that can be populated with values from other crates. It demonstrates:

1. **String Collection**: Collecting static string references
2. **Integer Collection**: Collecting i32 values
3. **Custom Struct Collection**: Collecting instances of a custom struct

## Key Components

### Distributed Slices

- `STRING_VALUES`: Collects static string references
- `INTEGER_VALUES`: Collects i32 integer values
- `CUSTOM_DATA`: Collects CustomData struct instances

### Custom Data Structure

```rust
#[derive(Debug, Clone)]
pub struct CustomData {
    pub id: u32,
    pub name: &'static str,
    pub value: f64,
}
```

## Usage

Other crates can register values to these distributed slices using the `#[distributed_slice(...)]` attribute:

```rust
use linkme::distributed_slice;
use example_array::{STRING_VALUES, INTEGER_VALUES, CUSTOM_DATA, CustomData};

#[distributed_slice(STRING_VALUES)]
static MY_STRING: &str = "My value";

#[distributed_slice(INTEGER_VALUES)]
static MY_INT: i32 = 123;

#[distributed_slice(CUSTOM_DATA)]
static MY_DATA: CustomData = CustomData {
    id: 42,
    name: "Example",
    value: 3.14,
};
```

## Accessing Values

Values can be accessed at runtime through the distributed slice variables:

```rust
// Iterate over all registered strings
for s in STRING_VALUES {
    println!("String: {}", s);
}

// Access specific values
let first_int = INTEGER_VALUES[0];
let data_count = CUSTOM_DATA.len();
```

## Benefits

1. **Zero-cost abstraction**: No runtime initialization required
2. **Cross-crate registration**: Values can be registered from any crate in the dependency graph
3. **Type safety**: Compile-time type checking ensures only compatible values are registered
4. **Linker-based**: Uses platform-specific linker support for efficient collection