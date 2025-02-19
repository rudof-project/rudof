use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;

/// Represents a max cardinality which can be a fixed integer or `Unbounded`
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Max {
    Unbounded,
    IntMax(usize),
}

impl Max {
    pub fn minus(&self, n: usize) -> Max {
        match self {
            Max::Unbounded => Max::Unbounded,
            Max::IntMax(0) => Max::IntMax(0),
            Max::IntMax(m) => {
                let max = if m > &n { m - n } else { 0 };
                Max::IntMax(max)
            }
        }
    }

    pub fn greater_or_equal(&self, n: usize) -> bool {
        match self {
            Max::IntMax(max) => *max >= n,
            Max::Unbounded => true,
        }
    }
}

impl From<usize> for Max {
    fn from(m: usize) -> Self {
        Max::IntMax(m)
    }
}

impl From<i32> for Max {
    fn from(m: i32) -> Self {
        Max::IntMax(m as usize)
    }
}

impl From<i64> for Max {
    fn from(m: i64) -> Self {
        match m {
            -1 => Max::Unbounded,
            n if n > 0 => Max::IntMax(n as usize),
            _ => panic!("Error converting i64 to Max, value {m} < -1"),
        }
    }
}

impl From<u64> for Max {
    fn from(m: u64) -> Self {
        Max::IntMax(m as usize)
    }
}

impl From<isize> for Max {
    fn from(m: isize) -> Self {
        Max::IntMax(m as usize)
    }
}

impl fmt::Display for Max {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Max::Unbounded => write!(dest, "*"),
            Max::IntMax(max) => write!(dest, "{max}"),
        }
    }
}

impl Serialize for Max {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let value: i64 = match *self {
            Max::Unbounded => -1,
            Max::IntMax(n) => n as i64,
        };
        serializer.serialize_i64(value)
    }
}

struct MaxVisitor;

impl<'de> Visitor<'de> for MaxVisitor {
    type Value = Max;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a positive integer or -1")
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
            match value {
                -1 => Ok(Max::Unbounded),
                n => Ok(Max::from(n)),
            }
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Max::from(value))
    }
}

impl<'de> Deserialize<'de> for Max {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(MaxVisitor)
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
