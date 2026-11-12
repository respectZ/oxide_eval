use anyhow::Result;
use serde_json::Value;

use crate::util::{number_to_uint32, value_to_number};

pub fn unary_bitwise_not(value: Value) -> Result<Value> {
    let number = number_to_uint32(value_to_number(value)?) as i32;
    Ok(Value::from(!number))
}
