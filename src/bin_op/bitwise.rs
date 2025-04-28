use anyhow::Result;
use serde_json::Value;

use crate::util::{number_from_f64, value_to_number};

pub fn bitwise_operation<F>(left: Value, right: Value, operator: F) -> Result<Value>
where
    F: Fn(i32, i32) -> i32,
{
    let left = value_to_number(left)? as i32;
    let right = value_to_number(right)? as i32;
    let result = operator(left, right) as f64;
    Ok(Value::Number(number_from_f64(result)?))
}

pub fn unsigned_right_shift(left: Value, right: Value) -> Result<Value> {
    let left = (value_to_number(left)? as i32) as u32;
    let right = value_to_number(right)? as u32 & 0x1F;
    Ok(Value::Number(number_from_f64((left >> right) as f64)?))
}
