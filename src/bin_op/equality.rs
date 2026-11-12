use anyhow::Result;
use serde_json::Value;

use crate::util::{string_to_number, value_to_primitive};

pub fn equality(left: Value, right: Value, strict: bool) -> Result<bool> {
    let equal = match (&left, &right) {
        (Value::Null, Value::Null) => true,
        (Value::Number(left), Value::Number(right)) => left.as_f64() == right.as_f64(),
        (Value::String(left), Value::String(right)) => left == right,
        (Value::Bool(left), Value::Bool(right)) => left == right,
        _ if strict => false,
        (Value::Number(number), Value::String(string))
        | (Value::String(string), Value::Number(number)) => {
            number.as_f64().is_some_and(|number| number == string_to_number(string))
        }
        (Value::Bool(value), _) => {
            return equality(Value::from(i32::from(*value)), right, false);
        }
        (_, Value::Bool(value)) => {
            return equality(left, Value::from(i32::from(*value)), false);
        }
        (Value::Array(_) | Value::Object(_), Value::Number(_) | Value::String(_)) => {
            return equality(value_to_primitive(left)?, right, false);
        }
        (Value::Number(_) | Value::String(_), Value::Array(_) | Value::Object(_)) => {
            return equality(left, value_to_primitive(right)?, false);
        }
        _ => false,
    };
    Ok(equal)
}
