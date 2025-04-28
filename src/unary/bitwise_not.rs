use anyhow::Result;
use serde_json::Value;

use crate::util::value_to_number;

pub fn unary_bitwise_not(value: Value) -> Result<Value> {
    let number = value_to_number(value)?;
    let result = !(number as i64);
    Ok(Value::Number(result.into()))
}
