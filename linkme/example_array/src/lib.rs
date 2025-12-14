use linkme::distributed_slice;

// Define a distributed slice for storing string values
#[distributed_slice]
pub static STRING_VALUES: [&'static str] = [..];

// Define a distributed slice for storing integer values
#[distributed_slice]
pub static INTEGER_VALUES: [i32] = [..];

// Define a distributed slice for storing custom struct values
#[derive(Debug, Clone)]
pub struct CustomData {
    pub id: u32,
    pub name: &'static str,
    pub value: f64,
}

#[distributed_slice]
pub static CUSTOM_DATA: [CustomData] = [..];

// Include the data providers module
mod data_providers;
