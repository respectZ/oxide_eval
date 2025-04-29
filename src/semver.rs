use anyhow::{anyhow, bail, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::util::value_to_number;

#[derive(Serialize, Deserialize)]
pub struct SemverParser {
    pub version: Version,
}

impl SemverParser {
    pub fn new(major: Value, minor: Value, patch: Value) -> Result<Self> {
        let major = value_to_number(major)? as u64;
        let minor = value_to_number(minor)? as u64;
        let patch = value_to_number(patch)? as u64;
        Ok(SemverParser {
            version: Version::new(major, minor, patch),
        })
    }
    pub fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::String(value) => Ok(SemverParser {
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
}
