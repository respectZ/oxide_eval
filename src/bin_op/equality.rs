use serde_json::Value;

pub fn equality(left: &Value, right: &Value, strict: bool) -> bool {
    match (left, right) {
        (Value::Null, Value::Null) => true,
        (Value::Number(l), Value::Number(r)) => l.as_f64() == r.as_f64(),
        (Value::String(l), Value::String(r)) => l == r,
        (Value::Bool(l), Value::Bool(r)) => l == r,
        (Value::Number(l), Value::String(r)) | (Value::String(r), Value::Number(l)) => {
            if strict {
                return false;
            }
            match (l.as_f64(), r.parse::<f64>()) {
                (Some(l), Ok(r)) => l == r,
                _ => false,
            }
        }
        _ => false,
    }
}
