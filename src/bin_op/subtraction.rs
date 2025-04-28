use anyhow::Result;
use serde_json::Value;

use crate::util::{number_from_f64, value_to_number};

pub fn subtraction(left: Value, right: Value) -> Result<Value> {
    let lnum = value_to_number(left)?;
    let rnum = value_to_number(right)?;
    let result = lnum - rnum;
    Ok(Value::Number(number_from_f64(result)?))
}
