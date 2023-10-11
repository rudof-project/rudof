use core::fmt;
use std::result;

use rust_decimal::Decimal;
use serde::{de::Visitor, Deserialize, Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub enum NumericLiteral {
    Integer(usize),
    Decimal(Decimal),
    Double(f64),
}

impl Serialize for NumericLiteral {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NumericLiteral::Integer(n) => {
                let c: u128 = (*n).try_into().unwrap();
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
                let n: usize = v.try_into().unwrap();
                Ok(NumericLiteral::Integer(n))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let n: usize = v.try_into().unwrap();
                Ok(NumericLiteral::Integer(n))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let n: usize = v.try_into().unwrap();
                Ok(NumericLiteral::Integer(n))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let n: usize = v.try_into().unwrap();
                Ok(NumericLiteral::Integer(n))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let n: usize = v.try_into().unwrap();
                Ok(NumericLiteral::Integer(n))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let n: Decimal = v.try_into().unwrap();
                Ok(NumericLiteral::Decimal(n))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let n: Decimal = v.try_into().unwrap();
                Ok(NumericLiteral::Decimal(n))
            }
        }

        deserializer.deserialize_any(NumericLiteralVisitor)
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
        let expected = NumericLiteral::Integer(23);
        assert_eq!(deser, expected);
    }

    #[test]
    fn test_serialize_decimal() {
        let n = NumericLiteral::Decimal(dec!(5.5));
        let expected = r#"5.5"#;
        let json = serde_json::to_string(&n).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_deserialize_decimal() {
        let str = r#"5.5"#;
        let deser: NumericLiteral = serde_json::from_str(str).unwrap();
        let expected = NumericLiteral::Decimal(dec!(5.5));
        assert_eq!(deser, expected);
    }
}
