use core::fmt;

use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

/// Represents a min cardinality which must be a 0 or positive integer.
#[derive(PartialEq, Eq, Hash, PartialOrd, Debug, Clone, Copy)]
pub struct Min {
    pub value: usize,
}

impl Min {
    pub fn is_0(&self) -> bool {
        self.value == 0
    }
}

impl From<usize> for Min {
    fn from(v: usize) -> Self {
        Min { value: v }
    }
}

impl From<i32> for Min {
    fn from(v: i32) -> Self {
        Min { value: v as usize }
    }
}

impl From<i8> for Min {
    fn from(v: i8) -> Self {
        Min { value: v as usize }
    }
}

impl From<i64> for Min {
    fn from(v: i64) -> Self {
        Min { value: v as usize }
    }
}

impl From<u64> for Min {
    fn from(v: u64) -> Self {
        Min { value: v as usize }
    }
}

impl Serialize for Min {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let value = self.value as u128;
        serializer.serialize_u128(value)
    }
}

struct MinVisitor;

impl<'de> Visitor<'de> for MinVisitor {
    type Value = Min;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a positive integer")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value < -1 {
            Err(E::custom(format!(
                "value of type i8 {} should be -1 or positive",
                value
            )))
        } else {
            let n = Min::from(value);
            Ok(n)
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value < -1 {
            Err(E::custom(format!(
                "value of type i32 {} should be -1 or positive",
                value
            )))
        } else {
            Ok(Min::from(value))
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value < -1 {
            Err(E::custom(format!(
                "value of type i64 {} should be -1 or positive",
                value
            )))
        } else {
            Ok(Min::from(value))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Min::from(value))
    }
}

impl<'de> Deserialize<'de> for Min {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(MinVisitor)
    }
}

#[cfg(test)]
mod tests {
    // use serde_json::*;
    // use serde::Serialize;

    use crate::Min;

    #[test]
    fn test_serialize_min() {
        let min = Min::from(23);
        let str = serde_json::to_string(&min).unwrap();
        assert_eq!(str, "23");
    }

    #[test]
    fn test_deserialize_min() {
        let min = Min::from(23);
        let min_deser = serde_json::from_str("23").unwrap();
        assert_eq!(min, min_deser);
    }
}
