use example_array::{CUSTOM_DATA, INTEGER_VALUES, STRING_VALUES};
use example_link::{ASYNC_HOOK_FUNCTIONS, HOOK_FUNCTIONS};
use linkme::distributed_slice;
use std::pin::Pin;

// Register additional functions to demonstrate cross-crate registration
#[distributed_slice(HOOK_FUNCTIONS)]
fn main_hook_1(name: &str) -> String {
    format!("Main hook 1 called with: {}", name)
}

#[distributed_slice(HOOK_FUNCTIONS)]
fn main_hook_2(name: &str) -> String {
    format!("Main hook 2 processing: {}", name)
}

#[distributed_slice(ASYNC_HOOK_FUNCTIONS)]
fn async_main_hook() -> Pin<Box<dyn std::future::Future<Output = String> + Send>> {
    Box::pin(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        "Async main hook completed".to_string()
    })
}

#[tokio::main]
async fn main() {
    println!("=== Linkme Cross-Crate Hook Demonstration ===\n");

    // Demonstrate string values from example_array
    println!("String Values from example_array:");
    for (i, s) in STRING_VALUES.iter().enumerate() {
        println!("  [{}] {}", i, s);
    }
    println!("Total strings: {}\n", STRING_VALUES.len());

    // Demonstrate integer values from example_array
    println!("Integer Values from example_array:");
    for (i, val) in INTEGER_VALUES.iter().enumerate() {
        println!("  [{}] {}", i, val);
    }
    println!("Total integers: {}\n", INTEGER_VALUES.len());

    // Demonstrate custom data from example_array
    println!("Custom Data from example_array:");
    for (i, data) in CUSTOM_DATA.iter().enumerate() {
        println!(
            "  [{}] ID: {}, Name: {}, Value: {}",
            i, data.id, data.name, data.value
        );
    }
    println!("Total custom data items: {}\n", CUSTOM_DATA.len());

    // Demonstrate synchronous hook functions
    println!("Synchronous Hook Functions:");
    let test_input = "Linkme Demo";
    for (i, hook_fn) in HOOK_FUNCTIONS.iter().enumerate() {
        let result = hook_fn(test_input);
        println!("  [{}] {}", i, result);
    }
    println!("Total hook functions: {}\n", HOOK_FUNCTIONS.len());

    // Demonstrate asynchronous hook functions
    println!("Asynchronous Hook Functions:");
    for (i, async_fn) in ASYNC_HOOK_FUNCTIONS.iter().enumerate() {
        let future = async_fn();
        let result = future.await;
        println!("  [{}] {}", i, result);
    }
    println!(
        "Total async hook functions: {}\n",
        ASYNC_HOOK_FUNCTIONS.len()
    );

    // Demonstrate dynamic hook execution
    println!("=== Dynamic Hook Execution ===");
    execute_hooks("Dynamic Test").await;

    println!("\n=== Demonstration Complete ===");
}

async fn execute_hooks(input: &str) {
    println!("Executing all hooks with input: '{}'", input);

    // Execute sync hooks
    println!("Sync hooks:");
    for hook_fn in HOOK_FUNCTIONS.iter() {
        let result = hook_fn(input);
        println!("  -> {}", result);
    }

    // Execute async hooks concurrently
    println!("Async hooks (concurrent execution):");
    let mut futures = Vec::new();
    for async_fn in ASYNC_HOOK_FUNCTIONS.iter() {
        futures.push(async_fn());
    }

    let results = futures_util::future::join_all(futures).await;
    for (i, result) in results.into_iter().enumerate() {
        println!("  -> [{}] {}", i, result);
    }
}
