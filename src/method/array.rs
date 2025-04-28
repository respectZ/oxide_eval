use anyhow::{anyhow, bail, Result};
use serde_json::Value;

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
        let result: Vec<String> = arr
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();
        let joined = result.join(delimiter);
        Ok(Value::String(joined))
    }
}
