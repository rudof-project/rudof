use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de;
use serde::de::Visitor;
use std::fmt;

/// Represents a max cardinality which can be a fixed integer or `Unbounded`
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Max {
    Unbounded,
    IntMax(usize),
}

impl Max {
    /// Subtracts n from self. If self is Unbounded, it remains Unbounded. If self is an IntMax and n is greater than or equal to the IntMax value, it returns IntMax(0). Otherwise, it returns a new IntMax with the value of self minus n.
    pub fn minus(&self, n: usize) -> Max {
        match self {
            Max::Unbounded => Max::Unbounded,
            Max::IntMax(0) => Max::IntMax(0),
            Max::IntMax(m) => {
                let max = if m > &n { m - n } else { 0 };
                Max::IntMax(max)
            },
        }
    }

    /// Returns true if self is greater than or equal to n. Unbounded is considered greater than or equal to any integer.
    pub fn greater_or_equal(&self, n: usize) -> bool {
        match self {
            Max::IntMax(max) => *max >= n,
            Max::Unbounded => true,
        }
    }

    /// Returns true if self is greater than other. Unbounded is considered greater than any IntMax, and two Unbounded values are not greater than each other.
    pub fn greater_than(&self, other: &Max) -> bool {
        match (self, other) {
            (Max::Unbounded, Max::Unbounded) => false,
            (Max::Unbounded, Max::IntMax(_)) => true,
            (Max::IntMax(_), Max::Unbounded) => false,
            (Max::IntMax(m1), Max::IntMax(m2)) => m1 > m2,
        }
    }

    /// Returns the maximum of two Max values. If either of the Max values is Unbounded, the result will be Unbounded. Otherwise, it returns the maximum of the two integer values.
    pub fn max(&self, other: &Max) -> Max {
        match (self, other) {
            (Max::Unbounded, _) | (_, Max::Unbounded) => Max::Unbounded,
            (Max::IntMax(m1), Max::IntMax(m2)) => Max::IntMax(std::cmp::max(*m1, *m2)),
        }
    }

    /// Returns the minimum of two Max values. If one of the Max values is Unbounded, it returns the other Max value. If both Max values are IntMax, it returns the minimum of the two integer values.
    pub fn min(&self, other: &Max) -> Max {
        match self {
            Max::Unbounded => other.clone(),
            Max::IntMax(m) => match other {
                Max::Unbounded => self.clone(),
                Max::IntMax(n) => Max::IntMax(std::cmp::min(*m, *n)),
            },
        }
    }

    /// Returns the sum of two Max values. If either of the Max values is Unbounded, the result will be Unbounded. Otherwise, it returns the sum of the two integer values.
    pub fn plus(&self, other: &Max) -> Max {
        match (self, other) {
            (Max::Unbounded, _) | (_, Max::Unbounded) => Max::Unbounded,
            (Max::IntMax(m1), Max::IntMax(m2)) => Max::IntMax(m1 + m2),
        }
    }

    ///  Result of v divided by self, rounded up
    pub fn div_up(&self, v: &usize) -> Max {
        match (v, self) {
            // I removed this because the Scala implementation also removes this
            // (0, Max::IntMax(0)) => Max::Unbounded, // 0 divided by 0 is considered unbounded
            (0, _) => Max::IntMax(0),
            (_, Max::Unbounded) => Max::IntMax(1), // Any positive integer divided by unbounded is considered to be 1, since it can be satisfied with at least one occurrence of the symbol.
            (_, Max::IntMax(0)) => Max::Unbounded, // Division by zero is considered unbounded
            (v, Max::IntMax(m)) => Max::IntMax(v.div_ceil(*m)),
        }
    }

    /// Result of v divided by self, rounded down
    pub fn div_down(&self, v: &usize) -> Max {
        match (v, self) {
            (0, Max::IntMax(0)) => Max::Unbounded, // 0 divided by 0 is considered unbounded
            (0, _) => Max::IntMax(0),
            (_, Max::Unbounded) => Max::IntMax(1), // Any positive integer divided by unbounded is considered to be 1, since it can be satisfied with at least one occurrence of the symbol.
            (_, Max::IntMax(0)) => Max::Unbounded, // Division by zero is considered unbounded
            (v, Max::IntMax(m)) => {
                Max::IntMax(v / m) // This is the formula for floor division
            },
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

impl Visitor<'_> for MaxVisitor {
    type Value = Max;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a positive integer or -1")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value < -1 {
            Err(E::custom(format!("value of type i64 {value} should be -1 or positive")))
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
