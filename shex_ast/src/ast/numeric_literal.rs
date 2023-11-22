use core::fmt;
use std::result;

use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{de::Visitor, Deserialize, Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub enum NumericLiteral {
    Integer(isize),
    Decimal(Decimal),
    Double(Decimal),
}

impl NumericLiteral {
    /*     pub fn double(d: f64) -> NumericLiteral {
        NumericLiteral::Double(d)
    }*/

    pub fn decimal(whole: i64, fraction: u32) -> NumericLiteral {
        let s = format!("{whole}.{fraction}");
        let d = Decimal::from_str_exact(s.as_str()).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_f64(d: f64) -> NumericLiteral {
        let d: Decimal = Decimal::from_f64(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn decimal_from_f32(d: f32) -> NumericLiteral {
        let d: Decimal = Decimal::from_f32(d).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn integer(n: isize) -> NumericLiteral {
        NumericLiteral::Integer(n)
    }
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
            NumericLiteral::Double(d) => {
                let f: f64 = (*d).try_into().unwrap();
                serializer.serialize_f64(f)
            }
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
                let n: isize = v.try_into().unwrap();
                Ok(NumericLiteral::Integer(n))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let n: isize = v.try_into().unwrap();
                Ok(NumericLiteral::Integer(n))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let n: isize = v.try_into().unwrap();
                Ok(NumericLiteral::Integer(n))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let n: isize = v.try_into().unwrap();
                Ok(NumericLiteral::Integer(n))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let n: isize = v.try_into().unwrap();
                Ok(NumericLiteral::Integer(n))
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

impl ToString for NumericLiteral {
    fn to_string(&self) -> String {
        todo!()
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
