use anyhow::Result;
use serde_json::Value;

use crate::util::{number_from_f64, value_to_number};

pub fn unary_function<F>(value: Value, f: F) -> Result<Value>
where
    F: Fn(f64) -> f64,
{
    let value = value_to_number(value)?;
    let result = f(value);
    Ok(Value::Number(number_from_f64(result)?))
}

pub fn binary_function<F>(value: Value, second: Value, f: F) -> Result<Value>
where
    F: Fn(f64, f64) -> f64,
{
    let left = value_to_number(value)?;
    let right = value_to_number(second)?;
    let result = f(left, right);
    Ok(Value::Number(number_from_f64(result)?))
}
