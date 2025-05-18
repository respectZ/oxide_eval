use anyhow::{anyhow, bail, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::util::value_to_number;

#[derive(Serialize, Deserialize)]
pub struct SemverWrapper {
    pub version: Version,
}

impl SemverWrapper {
    pub fn new(major: Value, minor: Value, patch: Value) -> Result<Self> {
        let major = value_to_number(major)? as u64;
        let minor = value_to_number(minor)? as u64;
        let patch = value_to_number(patch)? as u64;
        Ok(SemverWrapper {
            version: Version::new(major, minor, patch),
        })
    }
    pub fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::String(value) => Ok(SemverWrapper {
                version: Version::parse(&value)?,
            }),
            Value::Array(value) => {
                if let (Some(major), Some(minor), Some(patch)) =
                    (value.get(0), value.get(1), value.get(2))
                {
                    Self::new(major.to_owned(), minor.to_owned(), patch.to_owned())
                } else {
                    bail!("array requires size of 3")
                }
            }
            Value::Object(mut value) => {
                let major = value
                    .remove("major")
                    .ok_or_else(|| anyhow!("Missing 'major' field"))?;
                let minor = value
                    .remove("minor")
                    .ok_or_else(|| anyhow!("Missing 'minor' field"))?;
                let patch = value
                    .remove("patch")
                    .ok_or_else(|| anyhow!("Missing 'patch' field"))?;
                Self::new(major, minor, patch)
            }
            _ => bail!("unsupported value type for semver parser: {:?}", value),
        }
    }
    pub fn from_values(mut args: Vec<Value>) -> Result<Self> {
        match args.len() {
            1 => Self::from_value(args.remove(0)),
            3 => {
                let patch = args.pop().unwrap();
                let minor = args.pop().unwrap();
                let major = args.pop().unwrap();
                Self::new(major, minor, patch)
            }
            _ => bail!("semver requires either 1 or 3 args"),
        }
    }
}
