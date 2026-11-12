use std::cmp::Ordering;

use anyhow::Result;
use serde_json::Value;

use crate::util::{value_to_number, value_to_primitive};

pub fn compare<F>(left: Value, right: Value, cmp: F) -> Result<bool>
where
    F: FnOnce(Ordering) -> bool,
{
    let left = value_to_primitive(left)?;
    let right = value_to_primitive(right)?;
    let ordering = match (&left, &right) {
        (Value::String(left), Value::String(right)) => {
            Some(left.encode_utf16().cmp(right.encode_utf16()))
        }
        _ => value_to_number(left)?.partial_cmp(&value_to_number(right)?),
    };
    Ok(ordering.is_some_and(cmp))
}
