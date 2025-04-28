use anyhow::{anyhow, Result};
use serde_json::{Number, Value};

use crate::unary::unary_plus;

pub static OBJ_STR: &str = "[object Object]";
pub static NULL_STR: &str = "null";

pub fn number_from_f64(value: f64) -> Result<Number> {
    if value.fract() == 0.0 {
        Ok(Number::from(value as i64))
    } else {
        Number::from_f64(value).ok_or_else(|| anyhow!("Invalid float number: {}", value))
    }
}

pub fn vec_to_js_string(vec: &[Value]) -> String {
    vec.iter()
        .map(|v| match v {
            Value::Null => "".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(a) => vec_to_js_string(a), // arrays are recursively flattened
            Value::Object(_) => OBJ_STR.to_string(),
        })
        .collect::<Vec<_>>()
        .join(",")
}

pub fn boolean_to_number(value: &bool) -> i32 {
    if *value {
        1
    } else {
        0
    }
}

pub fn value_to_number(value: Value) -> Result<f64> {
    match value {
        Value::Null => Ok(0.0),
        Value::Bool(b) => Ok(if b { 1.0 } else { 0.0 }),
        Value::Number(n) => Ok(n.as_f64().unwrap_or_default()),
        Value::String(s) => Ok(s.parse::<f64>().unwrap_or(0.0)),
        Value::Array(arr) => match unary_plus(Value::Array(arr))? {
            Value::Number(n) => Ok(n.as_f64().unwrap_or_default()),
            _ => Ok(0.0),
        },
        Value::Object(_) => Ok(f64::NAN),
    }
}

pub fn value_to_string(value: Value) -> String {
    match value {
        Value::Null => NULL_STR.to_string(),
        Value::Bool(b) => {
            if b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        Value::Number(n) => n.to_string(),
        Value::String(s) => s,
        Value::Array(arr) => vec_to_js_string(&arr),
        Value::Object(_) => OBJ_STR.to_string(),
    }
}

pub fn value_to_primitive(value: Value) -> Result<Value> {
    match value {
        Value::Object(_) => Ok(Value::String(OBJ_STR.to_string())),
        Value::Array(arr) => Ok(Value::String(vec_to_js_string(&arr))),
        other => Ok(other),
    }
}

pub fn is_string(value: &Value) -> bool {
    matches!(value, Value::String(_))
}
