use core::fmt;
use std::fmt::Display;

use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use serde::{de::Visitor, Deserialize, Serialize, Serializer};
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone)]
pub enum NumericLiteral {
    Integer(isize),
    Decimal(Decimal),
    Double(f64),
}

impl NumericLiteral {
    /// Creates a numeric literal from a decimal
    pub fn decimal(d: Decimal) -> NumericLiteral {
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_parts(whole: i64, fraction: u32) -> NumericLiteral {
        let s = format!("{whole}.{fraction}");
        let d = Decimal::from_str_exact(s.as_str()).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_f64(d: f64) -> NumericLiteral {
        let d: Decimal = Decimal::from_f64(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_isize(d: isize) -> NumericLiteral {
        let d: Decimal = Decimal::from_isize(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_i32(d: i32) -> NumericLiteral {
        let d: Decimal = Decimal::from_i32(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_i64(d: i64) -> NumericLiteral {
        let d: Decimal = Decimal::from_i64(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_u128(d: u128) -> NumericLiteral {
        let d: Decimal = Decimal::from_u128(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_i128(d: i128) -> NumericLiteral {
        let d: Decimal = Decimal::from_i128(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn integer_from_i128(d: i128) -> NumericLiteral {
        let d: Decimal = Decimal::from_i128(d).unwrap();
        let n: isize = Decimal::to_isize(&d).unwrap();
        NumericLiteral::Integer(n)
    }

    pub fn decimal_from_u64(d: u64) -> NumericLiteral {
        let d: Decimal = Decimal::from_u64(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_u32(d: u32) -> NumericLiteral {
        let d: Decimal = Decimal::from_u32(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_f32(d: f32) -> NumericLiteral {
        let d: Decimal = Decimal::from_f32(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn integer(n: isize) -> NumericLiteral {
        NumericLiteral::Integer(n)
    }

    pub fn double(d: f64) -> NumericLiteral {
        NumericLiteral::Double(d)
    }

    pub fn lexical_form(&self) -> String {
        self.to_string()
    }

    pub fn as_decimal(&self) -> Decimal {
        match self {
            NumericLiteral::Integer(n) => Decimal::from_isize(*n).unwrap(),
            NumericLiteral::Double(d) => Decimal::from_f64(*d).unwrap(),
            NumericLiteral::Decimal(d) => *d,
        }
    }

    pub fn less_than(&self, other: &NumericLiteral) -> bool {
        match (self, other) {
            (NumericLiteral::Integer(n1), NumericLiteral::Integer(n2)) => n1 < n2,
            (v1, v2) => v1.as_decimal() < v2.as_decimal(),
        }
    }
}

impl Eq for NumericLiteral {}

impl Hash for NumericLiteral {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        // self.to_string().hash(state)
    }
}

impl Serialize for NumericLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NumericLiteral::Integer(n) => {
                let c: u128 = (*n) as u128;
                serializer.serialize_u128(c)
            }
            NumericLiteral::Decimal(d) => {
                let f: f64 = (*d).try_into().unwrap();
                serializer.serialize_f64(f)
            }
            NumericLiteral::Double(d) => serializer.serialize_f64(*d),
        }
    }
}

impl<'de> Deserialize<'de> for NumericLiteral {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NumericLiteralVisitor;

        impl<'de> Visitor<'de> for NumericLiteralVisitor {
            type Value = NumericLiteral;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("NumericLiteral")
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumericLiteral::decimal_from_i32(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumericLiteral::decimal_from_i64(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumericLiteral::decimal_from_u64(v))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumericLiteral::decimal_from_u32(v))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumericLiteral::decimal_from_u128(v))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumericLiteral::decimal_from_f64(v))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumericLiteral::decimal_from_f32(v))
            }
        }

        deserializer.deserialize_any(NumericLiteralVisitor)
    }
}

/*impl ToString for NumericLiteral {
    fn to_string(&self) -> String {
        match self {
            NumericLiteral::Double(d) => format!("{}", d),
            NumericLiteral::Integer(n) => n.to_string(),
            NumericLiteral::Decimal(d) => d.to_string(),
        }
    }
}*/

impl Display for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumericLiteral::Double(d) => write!(f, "{}", d),
            NumericLiteral::Integer(n) => write!(f, "{}", n),
            NumericLiteral::Decimal(d) => write!(f, "{}", d),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_serialize_integer() {
        let n = NumericLiteral::Integer(23);
        let json_nc = serde_json::to_string(&n).unwrap();
        assert_eq!(json_nc, "23");
    }

    #[test]
    fn test_deserialize_integer() {
        let str = r#"23"#;
        let deser: NumericLiteral = serde_json::from_str(str).unwrap();
        let expected = NumericLiteral::decimal(dec![23]);
        assert_eq!(deser, expected);
    }

    #[test]
    fn test_serialize_decimal() {
        let n = NumericLiteral::Decimal(dec!(5.35));
        let expected = r#"5.35"#;
        let json = serde_json::to_string(&n).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_deserialize_decimal() {
        let str = r#"5.35"#;
        let deser: NumericLiteral = serde_json::from_str(str).unwrap();
        let expected = NumericLiteral::Decimal(dec!(5.35));
        assert_eq!(deser, expected);
    }
}
