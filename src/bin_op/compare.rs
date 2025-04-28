use serde_json::Value;

use crate::util::{vec_to_js_string, OBJ_STR};

use super::util::{number_string, string_number};

pub fn compare<F>(left: &Value, right: &Value, cmp: F) -> bool
where
    F: Fn(&str, &str) -> bool + Copy,
{
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => cmp(
            &l.as_f64().unwrap_or_default().to_string(),
            &r.as_f64().unwrap_or_default().to_string(),
        ),
        (Value::String(l), Value::String(r)) => cmp(l, r),
        (Value::Number(l), Value::String(r)) => {
            number_string(l, r, |l, r| cmp(&l.to_string(), &r.to_string()))
        }
        (Value::String(l), Value::Number(r)) => {
            string_number(l, r, |l, r| cmp(&l.to_string(), &r.to_string()))
        }
        (Value::Array(l), Value::Array(r)) => {
            let l = vec_to_js_string(l);
            let r = vec_to_js_string(r);
            cmp(&l, &r)
        }
        (Value::Array(l), Value::Object(_)) => {
            let l = vec_to_js_string(l);
            cmp(&l, OBJ_STR)
        }
        (Value::Object(_), Value::Array(r)) => {
            let r = vec_to_js_string(r);
            cmp(OBJ_STR, &r)
        }
        _ => false,
    }
}
