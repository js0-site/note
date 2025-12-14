use linkme::distributed_slice;
use example_array::{STRING_VALUES, INTEGER_VALUES, CUSTOM_DATA, CustomData};
use std::pin::Pin;

// Define a function pointer type for async hooks
pub type AsyncHookFn = fn() -> Pin<Box<dyn std::future::Future<Output = String> + Send>>;

// Define distributed slices for function registration
#[distributed_slice]
pub static HOOK_FUNCTIONS: [fn(&str) -> String] = [..];

#[distributed_slice]
pub static ASYNC_HOOK_FUNCTIONS: [AsyncHookFn] = [..];

// Register more values to example_array's distributed slices
#[distributed_slice(STRING_VALUES)]
static ADDITIONAL_STR: &str = "Additional string from example_link";

#[distributed_slice(INTEGER_VALUES)]
static ADDITIONAL_INT: i32 = 999;

#[distributed_slice(CUSTOM_DATA)]
static ADDITIONAL_DATA: CustomData = CustomData {
    id: 100,
    name: "From example_link",
    value: 9.99,
};

// Register sync functions to our own distributed slice
#[distributed_slice(HOOK_FUNCTIONS)]
fn sync_greeting(name: &str) -> String {
    format!("Sync greeting from example_link: Hello, {}!", name)
}

#[distributed_slice(HOOK_FUNCTIONS)]
fn sync_processing(name: &str) -> String {
    format!("Sync processing from example_link: Processing '{}'", name)
}

#[distributed_slice(HOOK_FUNCTIONS)]
fn sync_analysis(name: &str) -> String {
    format!("Sync analysis from example_link: Analyzing '{}'", name)
}

// Register async functions to our own distributed slice
#[distributed_slice(ASYNC_HOOK_FUNCTIONS)]
fn async_task1() -> Pin<Box<dyn std::future::Future<Output = String> + Send>> {
    Box::pin(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        "Async task 1 from example_link".to_string()
    })
}

#[distributed_slice(ASYNC_HOOK_FUNCTIONS)]
fn async_task2() -> Pin<Box<dyn std::future::Future<Output = String> + Send>> {
    Box::pin(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        "Async task 2 from example_link".to_string()
    })
}

#[distributed_slice(ASYNC_HOOK_FUNCTIONS)]
fn async_task3() -> Pin<Box<dyn std::future::Future<Output = String> + Send>> {
    Box::pin(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        "Async task 3 from example_link".to_string()
    })
}
