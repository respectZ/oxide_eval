use anyhow::Result;
use serde_json::Value;

use crate::util::{number_from_f64, value_to_number};

pub fn division(left: Value, right: Value) -> Result<Value> {
    let left_num = value_to_number(left)?;
    let right_num = value_to_number(right)?;

    if right_num == 0.0 {
        // Division by 0 rules
        if left_num == 0.0 {
            Ok(Value::Null) // 0 / 0 => Null (like NaN in JS)
        } else if left_num > 0.0 {
            Ok(Value::Number(number_from_f64(f64::INFINITY)?))
        } else {
            Ok(Value::Number(number_from_f64(f64::NEG_INFINITY)?))
        }
    } else {
        let result = left_num / right_num;
        Ok(Value::Number(number_from_f64(result)?))
    }
}
