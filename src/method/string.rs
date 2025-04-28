use anyhow::{anyhow, bail, Result};
use regex::Regex;
use serde_json::Value;

pub struct StringMethod {
    args: Vec<Value>,
}

impl StringMethod {
    pub fn new(args: Vec<Value>) -> Self {
        StringMethod { args }
    }

    fn get_string_argument(&self, index: usize) -> Result<&str> {
        self.args
            .get(index)
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Argument {} must be a string", index + 1))
    }

    fn get_number_argument(&self, index: usize) -> Result<usize> {
        self.args
            .get(index)
            .and_then(|v| v.as_f64())
            .map(|v| v as usize)
            .ok_or_else(|| anyhow!("Argument {} must be a number", index + 1))
    }

    pub fn replace(&self, s: &str) -> Result<Value> {
        if self.args.len() != 2 {
            bail!("replace method requires 2 arguments");
        }
        let old = self.get_string_argument(0)?;
        let new = self.get_string_argument(1)?;
        Ok(Value::String(s.replace(old, new)))
    }

    pub fn contains(&self, s: &str) -> Result<Value> {
        if self.args.len() != 1 {
            bail!("contains method requires 1 argument");
        }
        let substring = self.get_string_argument(0)?;
        Ok(Value::Bool(s.contains(substring)))
    }

    pub fn split(&self, s: &str) -> Result<Value> {
        if self.args.len() != 1 {
            bail!("split method requires 1 argument");
        }
        let delimiter = self.get_string_argument(0)?;
        let result: Vec<String> = if delimiter.is_empty() {
            s.chars().map(|c| c.to_string()).collect()
        } else {
            s.split(delimiter).map(|s| s.to_string()).collect()
        };
        Ok(Value::Array(
            result.into_iter().map(Value::String).collect(),
        ))
    }

    pub fn index_of(&self, s: &str) -> Result<Value> {
        if self.args.len() != 1 {
            bail!("indexOf method requires 1 argument");
        }
        let substring = self.get_string_argument(0)?;
        Ok(Value::Number(
            s.find(substring).map_or(-1, |index| index as i32).into(),
        ))
    }

    pub fn last_index_of(&self, s: &str) -> Result<Value> {
        if self.args.len() != 1 {
            bail!("lastIndexOf method requires 1 argument");
        }
        let substring = self.get_string_argument(0)?;
        Ok(Value::Number(
            s.rfind(substring).map_or(-1, |index| index as i32).into(),
        ))
    }

    pub fn to_upper_case(&self, s: &str) -> Result<Value> {
        if self.args.len() != 0 {
            bail!("toUpperCase method requires no arguments");
        }
        Ok(Value::String(s.to_uppercase()))
    }

    pub fn to_lower_case(&self, s: &str) -> Result<Value> {
        if self.args.len() != 0 {
            bail!("toLowerCase method requires no arguments");
        }
        Ok(Value::String(s.to_lowercase()))
    }

    pub fn substring(&self, s: &str) -> Result<Value> {
        let start = self.get_number_argument(0)?;
        let end = if self.args.len() == 2 {
            self.get_number_argument(1)?
        } else {
            s.len()
        };
        Ok(Value::String(s[start..end].to_string()))
    }

    pub fn starts_with(&self, s: &str) -> Result<Value> {
        if self.args.len() != 1 {
            bail!("startsWith method requires 1 argument");
        }
        let prefix = self.get_string_argument(0)?;
        Ok(Value::Bool(s.starts_with(prefix)))
    }

    pub fn ends_with(&self, s: &str) -> Result<Value> {
        if self.args.len() != 1 {
            bail!("endsWith method requires 1 argument");
        }
        let suffix = self.get_string_argument(0)?;
        Ok(Value::Bool(s.ends_with(suffix)))
    }

    pub fn regex_replace(&self, s: &str) -> Result<Value> {
        if self.args.len() != 2 {
            bail!("regexReplace method requires 2 arguments");
        }
        let pattern = self.get_string_argument(0)?;
        let replacement = self.get_string_argument(1)?;
        let re = Regex::new(pattern).map_err(|e| anyhow!(e))?;
        Ok(Value::String(re.replace_all(s, replacement).to_string()))
    }

    pub fn length(&self, s: &str) -> Result<Value> {
        if self.args.len() != 0 {
            bail!("length method requires no arguments");
        }
        Ok(Value::Number(s.chars().count().into()))
    }

    pub fn trim(&self, s: &str) -> Result<Value> {
        if self.args.len() != 0 {
            bail!("trim method requires no arguments");
        }
        Ok(Value::String(s.trim().to_string()))
    }
}
