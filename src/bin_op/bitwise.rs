use anyhow::Result;
use serde_json::Value;

use crate::util::{number_to_uint32, value_to_number};

pub fn bitwise_operation<F>(left: Value, right: Value, operator: F) -> Result<Value>
where
    F: Fn(i32, i32) -> i32,
{
    let left = number_to_uint32(value_to_number(left)?) as i32;
    let right = number_to_uint32(value_to_number(right)?) as i32;
    Ok(Value::from(operator(left, right)))
}

pub fn unsigned_right_shift(left: Value, right: Value) -> Result<Value> {
    let left = number_to_uint32(value_to_number(left)?);
    let right = number_to_uint32(value_to_number(right)?) & 0x1F;
    Ok(Value::from(left >> right))
}
