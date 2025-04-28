use anyhow::Result;
use serde_json::Value;

use crate::util::{
    is_string, number_from_f64, value_to_number, value_to_primitive, value_to_string,
};

pub fn addition(left: Value, right: Value) -> Result<Value> {
    let left_primitive = value_to_primitive(left)?;
    let right_primitive = value_to_primitive(right)?;
    if is_string(&left_primitive) || is_string(&right_primitive) {
        let lstr = value_to_string(left_primitive);
        let rstr = value_to_string(right_primitive);
        Ok(Value::String(format!("{}{}", lstr, rstr)))
    } else {
        let lnum = value_to_number(left_primitive)?;
        let rnum = value_to_number(right_primitive)?;
        Ok(Value::Number(number_from_f64(lnum + rnum)?))
    }
}
