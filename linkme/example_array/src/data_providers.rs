use linkme::distributed_slice;
use crate::{STRING_VALUES, INTEGER_VALUES, CUSTOM_DATA, CustomData};

// Register string values
#[distributed_slice(STRING_VALUES)]
static STR_VAL_1: &str = "Hello from example_array!";

#[distributed_slice(STRING_VALUES)]
static STR_VAL_2: &str = "Linkme is awesome!";

#[distributed_slice(STRING_VALUES)]
static STR_VAL_3: &str = "Distributed slices work across crates!";

// Register integer values
#[distributed_slice(INTEGER_VALUES)]
static INT_VAL_1: i32 = 42;

#[distributed_slice(INTEGER_VALUES)]
static INT_VAL_2: i32 = 100;

#[distributed_slice(INTEGER_VALUES)]
static INT_VAL_3: i32 = -7;

// Register custom data values
#[distributed_slice(CUSTOM_DATA)]
static DATA_VAL_1: CustomData = CustomData {
    id: 1,
    name: "First Data",
    value: 3.14,
};

#[distributed_slice(CUSTOM_DATA)]
static DATA_VAL_2: CustomData = CustomData {
    id: 2,
    name: "Second Data",
    value: 2.71,
};

#[distributed_slice(CUSTOM_DATA)]
static DATA_VAL_3: CustomData = CustomData {
    id: 3,
    name: "Third Data",
    value: 1.618,
};