use anyhow::Result;
use serde_json::Value;

use crate::util::{number_from_f64, value_to_number};

pub fn remainder(left: Value, right: Value) -> Result<Value> {
    let lnum = value_to_number(left)?;
    let rnum = value_to_number(right)?;
    match lnum {
        0.0 => Ok(Value::Null),
        lnum => Ok(Value::Number(number_from_f64(lnum % rnum)?)),
    }
}
