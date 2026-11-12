use std::fmt::Write;

use anyhow::{anyhow, bail, Result};
use serde_json::Value;

use crate::util::{vec_to_js_string, OBJ_STR};

pub struct ArrayMethod {
    args: Vec<Value>,
}

impl ArrayMethod {
    pub fn new(args: Vec<Value>) -> Self {
        ArrayMethod { args }
    }
    fn get_string_argument(&self, index: usize) -> Result<&str> {
        self.args
            .get(index)
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Argument {} must be a string", index + 1))
    }
    pub fn join(&self, arr: &Vec<Value>) -> Result<Value> {
        if self.args.len() != 1 {
            bail!("join method requires 1 argument")
        }
        let delimiter = self.get_string_argument(0)?;
        let mut joined = String::new();
        for (index, value) in arr.iter().enumerate() {
            if index > 0 {
                joined.push_str(delimiter);
            }
            match value {
                Value::Null => {}
                Value::Bool(value) => joined.push_str(if *value { "true" } else { "false" }),
                Value::Number(value) => write!(&mut joined, "{}", value)?,
                Value::String(value) => joined.push_str(value),
                Value::Array(value) => joined.push_str(&vec_to_js_string(value)),
                Value::Object(_) => joined.push_str(OBJ_STR),
            }
        }
        Ok(Value::String(joined))
    }
}
