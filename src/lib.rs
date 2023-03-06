//! # Derp Numeric
//!
//! Handles numeric values that may be strings or integers.
//!
//! Somewhat adapted but mostly copied from:
//! https://users.rust-lang.org/t/deserialize-a-number-that-may-be-inside-a-string-serde-json/27318/4

use std::fmt;
use std::num::NonZeroU32;

use serde::de::{Deserializer, Visitor};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct DerpNumeric(NonZeroU32);

impl<'de> serde::de::Deserialize<'de> for DerpNumeric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = DerpNumeric;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("integer or string")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match NonZeroU32::new(val as u32) {
                    Some(val) => Ok(DerpNumeric(val)),
                    None => Err(E::custom("invalid integer value")),
                }
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<u64>() {
                    Ok(val) => self.visit_u64(val),
                    Err(_) => Err(E::custom("failed to parse integer")),
                }
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

#[cfg(test)]
mod test {
    use crate::DerpNumeric;

    #[test]
    fn simple() {
        let ret = serde_json::from_str::<DerpNumeric>("1");
        println!("{ret:?}");

        let ret = serde_json::from_str::<DerpNumeric>(r#""2""#);
        println!("{ret:?}");
    }
}
