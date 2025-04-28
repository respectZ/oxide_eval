use anyhow::Result;
use serde_json::Value;

use crate::util::{boolean_to_number, number_from_f64};

pub fn unary_plus(value: Value) -> Result<Value> {
    match value {
        Value::Array(value) => evaluate_array(value),
        Value::Bool(value) => {
            let value = boolean_to_number(&value);
            Ok(Value::Number(value.into()))
        }
        Value::Null => Ok(Value::Number(0.into())),
        Value::Number(value) => Ok(Value::Number(value)),
        Value::Object(_) => Ok(Value::Number(0.into())),
        Value::String(str) => {
            if let Ok(value) = str.parse::<f64>() {
                return Ok(Value::Number(number_from_f64(value)?));
            }
            Ok(Value::String("NaN".to_owned()))
        }
    }
}
pub fn evaluate_array(value: Vec<Value>) -> Result<Value> {
    if value.capacity() == 0 {
        return Ok(Value::Number(0.into()));
    }
    if value.capacity() > 1 {
        return Ok(Value::String("NaN".to_owned()));
    }
    let first = value.into_iter().next();
    match first {
        Some(value) => unary_plus(value),
        None => Ok(Value::String("NaN".to_owned())),
    }
}
