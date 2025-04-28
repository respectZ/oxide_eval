use anyhow::Result;
use serde_json::Value;

use crate::util::number_from_f64;

use super::plus::unary_plus;

pub fn unary_negation(value: Value) -> Result<Value> {
    let result = unary_plus(value)?;
    match result {
        Value::Number(value) => {
            let result = -value.as_f64().unwrap_or_default();
            Ok(Value::Number(number_from_f64(result)?))
        }
        _ => Ok(Value::Null),
    }
}
