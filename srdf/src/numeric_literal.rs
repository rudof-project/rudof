use core::fmt;
use std::fmt::Display;

use rust_decimal::{
    Decimal,
    prelude::{FromPrimitive, ToPrimitive},
};
use serde::{
    Deserialize,
    Serialize,
    Serializer,
    // de::{self, Visitor},
};
use std::hash::Hash;
use tracing::trace;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum NumericLiteral {
    Integer(isize),
    Byte(i8),
    Short(i16),
    NonNegativeInteger(u128),
    UnsignedLong(u64),
    UnsignedInt(u32),
    UnsignedShort(u16),
    UnsignedByte(u8),
    PositiveInteger(u128),
    NegativeInteger(i128),
    NonPositiveInteger(i128),
    Long(isize),
    Decimal(Decimal),
    Double(f64),
    Float(f64),
}

impl NumericLiteral {
    pub fn datatype(&self) -> &str {
        match self {
            NumericLiteral::Integer(_) => "http://www.w3.org/2001/XMLSchema#integer",
            NumericLiteral::Decimal(_) => "http://www.w3.org/2001/XMLSchema#decimal",
            NumericLiteral::Double(_) => "http://www.w3.org/2001/XMLSchema#double",
            NumericLiteral::Long(_) => "http://www.w3.org/2001/XMLSchema#long",
            NumericLiteral::Float(_) => "http://www.w3.org/2001/XMLSchema#float",
            NumericLiteral::Byte(_) => "http://www.w3.org/2001/XMLSchema#byte",
            NumericLiteral::Short(_) => "http://www.w3.org/2001/XMLSchema#short",
            NumericLiteral::NonNegativeInteger(_) => {
                "http://www.w3.org/2001/XMLSchema#nonNegativeInteger"
            }
            NumericLiteral::UnsignedLong(_) => "http://www.w3.org/2001/XMLSchema#unsignedLong",
            NumericLiteral::UnsignedInt(_) => "http://www.w3.org/2001/XMLSchema#unsignedInt",
            NumericLiteral::UnsignedShort(_) => "http://www.w3.org/2001/XMLSchema#unsignedShort",
            NumericLiteral::UnsignedByte(_) => "http://www.w3.org/2001/XMLSchema#unsignedByte",
            NumericLiteral::PositiveInteger(_) => {
                "http://www.w3.org/2001/XMLSchema#positiveInteger"
            }
            NumericLiteral::NegativeInteger(_) => {
                "http://www.w3.org/2001/XMLSchema#negativeInteger"
            }
            NumericLiteral::NonPositiveInteger(_) => {
                "http://www.w3.org/2001/XMLSchema#nonPositiveInteger"
            }
        }
    }

    /// Creates a numeric literal from a decimal
    pub fn decimal(d: Decimal) -> NumericLiteral {
        NumericLiteral::Decimal(d)
    }

    pub fn non_positive_integer(n: isize) -> NumericLiteral {
        let d: i128 = n as i128;
        NumericLiteral::NonPositiveInteger(d)
    }

    pub fn non_negative_integer(n: usize) -> NumericLiteral {
        let d: u128 = n as u128;
        NumericLiteral::NonNegativeInteger(d)
    }

    pub fn positive_integer(n: usize) -> NumericLiteral {
        let d: u128 = n as u128;
        NumericLiteral::PositiveInteger(d)
    }

    pub fn negative_integer(n: isize) -> NumericLiteral {
        let d: i128 = n as i128;
        NumericLiteral::NegativeInteger(d)
    }

    pub fn unsigned_byte(n: u8) -> NumericLiteral {
        NumericLiteral::UnsignedByte(n)
    }

    pub fn unsigned_short(n: u16) -> NumericLiteral {
        NumericLiteral::UnsignedShort(n)
    }

    pub fn unsigned_int(n: u32) -> NumericLiteral {
        NumericLiteral::UnsignedInt(n)
    }

    pub fn unsigned_long(n: u64) -> NumericLiteral {
        NumericLiteral::UnsignedLong(n)
    }

    pub fn decimal_from_parts(whole: i64, fraction: u32) -> NumericLiteral {
        let s = format!("{whole}.{fraction}");
        let d = Decimal::from_str_exact(s.as_str()).unwrap();
        NumericLiteral::Decimal(d)
    }

    pub fn byte(d: i8) -> NumericLiteral {
        NumericLiteral::Byte(d)
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

    pub fn long(d: isize) -> NumericLiteral {
        NumericLiteral::Long(d)
    }

    pub fn float(d: f64) -> NumericLiteral {
        NumericLiteral::Float(d)
    }

    pub fn integer_from_i128(d: i128) -> NumericLiteral {
        NumericLiteral::Integer(d as isize)
    }

    pub fn integer_from_i64(d: i64) -> NumericLiteral {
        NumericLiteral::Integer(d as isize)
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
            NumericLiteral::Long(l) => Decimal::from_isize(*l).unwrap(),
            NumericLiteral::Float(f) => Decimal::from_f64(*f).unwrap(),
            NumericLiteral::Byte(b) => Decimal::from_i8(*b).unwrap(),
            NumericLiteral::Short(s) => Decimal::from_i16(*s).unwrap(),
            NumericLiteral::NonNegativeInteger(n) => Decimal::from_u128(*n).unwrap(),
            NumericLiteral::UnsignedLong(n) => Decimal::from_u64(*n).unwrap(),
            NumericLiteral::UnsignedInt(n) => Decimal::from_u32(*n).unwrap(),
            NumericLiteral::UnsignedShort(n) => Decimal::from_u16(*n).unwrap(),
            NumericLiteral::UnsignedByte(n) => Decimal::from_u8(*n).unwrap(),
            NumericLiteral::PositiveInteger(n) => Decimal::from_u128(*n).unwrap(),
            NumericLiteral::NegativeInteger(n) => Decimal::from_i128(*n).unwrap(),
            NumericLiteral::NonPositiveInteger(n) => Decimal::from_i128(*n).unwrap(),
        }
    }

    pub fn less_than(&self, other: &NumericLiteral) -> bool {
        trace!("less_than: Comparing {self:?} < {other:?}");
        match (self, other) {
            (NumericLiteral::Integer(n1), NumericLiteral::Integer(n2)) => {
                let result = n1 < n2;
                trace!("less_than: {n1} < {n2} = {result}");
                result
            }
            (v1, v2) => v1.as_decimal() < v2.as_decimal(),
        }
    }

    pub fn less_than_or_eq(&self, other: &NumericLiteral) -> bool {
        match (self, other) {
            (NumericLiteral::Integer(n1), NumericLiteral::Integer(n2)) => n1 <= n2,
            (v1, v2) => v1.as_decimal() <= v2.as_decimal(),
        }
    }

    pub fn total_digits(&self) -> Option<usize> {
        match self {
            NumericLiteral::Integer(d) => Some(d.to_string().len()),
            NumericLiteral::Long(d) => Some(d.to_string().len()),
            NumericLiteral::NonNegativeInteger(d) => Some(d.to_string().len()),
            NumericLiteral::UnsignedLong(d) => Some(d.to_string().len()),
            NumericLiteral::UnsignedInt(d) => Some(d.to_string().len()),
            NumericLiteral::UnsignedShort(d) => Some(d.to_string().len()),
            NumericLiteral::UnsignedByte(d) => Some(d.to_string().len()),
            NumericLiteral::PositiveInteger(d) => Some(d.to_string().len()),
            NumericLiteral::NegativeInteger(d) => Some(d.to_string().len()),
            NumericLiteral::NonPositiveInteger(d) => Some(d.to_string().len()),
            NumericLiteral::Byte(d) => Some(d.to_string().len()),
            NumericLiteral::Short(d) => Some(d.to_string().len()),
            NumericLiteral::Decimal(d) => {
                // Normalize removes trailing zeros
                let normalized = d.normalize();
                let s = normalized.to_string();
                let s = s.replace("-", "").replace(".", "");
                Some(s.len())
            }
            NumericLiteral::Double(_d) => None,
            NumericLiteral::Float(_f) => None,
        }
    }

    pub fn fraction_digits(&self) -> Option<usize> {
        match self {
            NumericLiteral::Integer(_) => Some(0),
            NumericLiteral::Long(_) => Some(0),
            NumericLiteral::NonNegativeInteger(_) => Some(0),
            NumericLiteral::UnsignedLong(_) => Some(0),
            NumericLiteral::UnsignedInt(_) => Some(0),
            NumericLiteral::UnsignedShort(_) => Some(0),
            NumericLiteral::UnsignedByte(_) => Some(0),
            NumericLiteral::PositiveInteger(_) => Some(0),
            NumericLiteral::NegativeInteger(_) => Some(0),
            NumericLiteral::NonPositiveInteger(_) => Some(0),
            NumericLiteral::Byte(_) => Some(0),
            NumericLiteral::Short(_) => Some(0),
            NumericLiteral::Decimal(d) => {
                let s = d.to_string();
                if let Some(pos) = s.find('.') {
                    Some(s.len() - pos - 1)
                } else {
                    Some(0)
                }
            }
            NumericLiteral::Double(_d) => None,
            NumericLiteral::Float(_f) => None,
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
                let c: i128 = (*n) as i128;
                serializer.serialize_i128(c)
            }
            NumericLiteral::Decimal(d) => {
                let f: f64 = (*d).try_into().unwrap();
                serializer.serialize_f64(f)
            }
            NumericLiteral::Double(d) => serializer.serialize_f64(*d),
            NumericLiteral::Long(n) => {
                let c: i128 = (*n) as i128;
                serializer.serialize_i128(c)
            }
            NumericLiteral::Float(f) => serializer.serialize_f64(*f),
            NumericLiteral::Byte(b) => serializer.serialize_i8(*b),
            NumericLiteral::Short(s) => serializer.serialize_i16(*s),
            NumericLiteral::NonNegativeInteger(n) => serializer.serialize_u128(*n),
            NumericLiteral::UnsignedLong(n) => serializer.serialize_u64(*n),
            NumericLiteral::UnsignedInt(n) => serializer.serialize_u32(*n),
            NumericLiteral::UnsignedShort(n) => serializer.serialize_u16(*n),
            NumericLiteral::UnsignedByte(n) => serializer.serialize_u8(*n),
            NumericLiteral::PositiveInteger(n) => serializer.serialize_u128(*n),
            NumericLiteral::NegativeInteger(n) => serializer.serialize_i128(*n),
            NumericLiteral::NonPositiveInteger(n) => serializer.serialize_i128(*n),
        }
    }
}

/*
impl<'de> Deserialize<'de> for NumericLiteral {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NumericLiteralVisitor)
    }
} */

/*struct NumericLiteralVisitor;

impl Visitor<'_> for NumericLiteralVisitor {
    type Value = NumericLiteral;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("NumericLiteral")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NumericLiteral::try_from(v)
            .map_err(|e| E::custom(format!("Error parsing NumericLiteral: {e}")))
    }

    /*fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NumericLiteral::decimal_from_i32(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NumericLiteral::integer_from_i64(v))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NumericLiteral::integer_from_i128(v))
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
    }*/
}*/

impl TryFrom<&str> for NumericLiteral {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = value.parse::<isize>() {
            return Ok(NumericLiteral::Integer(i));
        } else if let Ok(f) = value.parse::<f64>() {
            return Ok(NumericLiteral::Double(f));
        } else if let Ok(d) = Decimal::from_str_exact(value) {
            return Ok(NumericLiteral::Decimal(d));
        }
        Err(format!("Cannot parse '{value}' as NumericLiteral"))
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
            NumericLiteral::Double(d) => write!(f, "{d}"),
            NumericLiteral::Integer(n) => write!(f, "{n}"),
            NumericLiteral::Decimal(d) => write!(f, "{d}"),
            NumericLiteral::Long(l) => write!(f, "{l}"),
            NumericLiteral::Float(n) => write!(f, "{n}"),
            NumericLiteral::Byte(b) => write!(f, "{b}"),
            NumericLiteral::Short(s) => write!(f, "{s}"),
            NumericLiteral::NonNegativeInteger(n) => write!(f, "{n}"),
            NumericLiteral::UnsignedLong(n) => write!(f, "{n}"),
            NumericLiteral::UnsignedInt(n) => write!(f, "{n}"),
            NumericLiteral::UnsignedShort(n) => write!(f, "{n}"),
            NumericLiteral::UnsignedByte(n) => write!(f, "{n}"),
            NumericLiteral::PositiveInteger(n) => write!(f, "{n}"),
            NumericLiteral::NegativeInteger(n) => write!(f, "{n}"),
            NumericLiteral::NonPositiveInteger(n) => write!(f, "{n}"),
        }
    }
}

impl PartialOrd for NumericLiteral {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_decimal().partial_cmp(&other.as_decimal()).unwrap())
    }
}

impl From<NumericLiteral> for oxrdf::Literal {
    fn from(n: NumericLiteral) -> Self {
        match n {
            NumericLiteral::Integer(i) => (i as i64).into(),
            NumericLiteral::Decimal(d) => match d.to_f64() {
                Some(decimal) => oxrdf::Literal::from(decimal),
                None => oxrdf::Literal::new_typed_literal(
                    d.to_string().as_str(),
                    oxrdf::vocab::xsd::DECIMAL,
                ),
            },
            NumericLiteral::Double(d) => oxrdf::Literal::from(d),
            NumericLiteral::Long(l) => (l as i64).into(),
            NumericLiteral::Float(f) => oxrdf::Literal::from(f),
            NumericLiteral::Byte(b) => (b as i64).into(),
            NumericLiteral::Short(s) => oxrdf::Literal::from(s),
            NumericLiteral::NonNegativeInteger(n) => {
                let s = n.to_string();
                oxrdf::Literal::new_typed_literal(
                    s.as_str(),
                    oxrdf::vocab::xsd::NON_NEGATIVE_INTEGER,
                )
            }
            NumericLiteral::UnsignedLong(n) => {
                let s = n.to_string();
                oxrdf::Literal::new_typed_literal(s.as_str(), oxrdf::vocab::xsd::UNSIGNED_LONG)
            }
            NumericLiteral::UnsignedInt(n) => {
                let s = n.to_string();
                oxrdf::Literal::new_typed_literal(s.as_str(), oxrdf::vocab::xsd::UNSIGNED_INT)
            }
            NumericLiteral::UnsignedShort(n) => {
                let s = n.to_string();
                oxrdf::Literal::new_typed_literal(s.as_str(), oxrdf::vocab::xsd::UNSIGNED_SHORT)
            }
            NumericLiteral::UnsignedByte(n) => {
                let s = n.to_string();
                oxrdf::Literal::new_typed_literal(s.as_str(), oxrdf::vocab::xsd::UNSIGNED_BYTE)
            }
            NumericLiteral::PositiveInteger(n) => {
                let s = n.to_string();
                oxrdf::Literal::new_typed_literal(s.as_str(), oxrdf::vocab::xsd::POSITIVE_INTEGER)
            }
            NumericLiteral::NegativeInteger(n) => {
                let s = n.to_string();
                oxrdf::Literal::new_typed_literal(s.as_str(), oxrdf::vocab::xsd::NEGATIVE_INTEGER)
            }
            NumericLiteral::NonPositiveInteger(n) => {
                let s = n.to_string();
                oxrdf::Literal::new_typed_literal(
                    s.as_str(),
                    oxrdf::vocab::xsd::NON_POSITIVE_INTEGER,
                )
            }
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
