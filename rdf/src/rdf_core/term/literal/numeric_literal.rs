use core::fmt;
use std::fmt::Display;

use rust_decimal::{
    Decimal,
    prelude::{FromPrimitive, ToPrimitive},
};
use serde::{Deserialize, Serialize, Serializer};
use std::hash::Hash;
use prefixmap::IriRef;
use iri_s::IriS;
use crate::rdf_core::{RDFError};

/// Represents RDF numeric literals with XSD datatype semantics.
///
/// This enum supports all XSD numeric types and provides type-safe
/// conversions between them. Uses `#[serde(untagged)]` for flexible
/// deserialization from JSON/other formats.
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum NumericLiteral {
    Integer(i128),  
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
    Long(i64),  
    Decimal(Decimal),
    Double(f64),
    Float(f32),  
}

// Macro to eliminate duplicated decimal conversion methods
macro_rules! impl_decimal_from {
    ($name:ident, $type:ty, $from_method:ident) => {
        /// Converts value to Decimal literal.
        ///
        /// # Errors
        /// Returns error if value cannot be represented as Decimal (e.g., NaN, infinity for floats).
        pub fn $name(d: $type) -> Result<NumericLiteral, RDFError> {
            Decimal::$from_method(d)
                .ok_or_else(|| RDFError::ConversionError {
                    msg: format!("Decimal conversion error from {}: {}", stringify!($type), d)
                })
                .map(NumericLiteral::Decimal)
        }
    };
}

/// ## Constructor Methods 
impl NumericLiteral {
    /// Returns the XSD datatype IRI for this numeric literal.
    ///
    /// Each variant maps to its corresponding XML Schema datatype.
    pub fn datatype(&self) -> IriRef {
        match self {
            NumericLiteral::Integer(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer")),
            NumericLiteral::Decimal(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#decimal")),
            NumericLiteral::Double(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#double")),
            NumericLiteral::Long(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#long")),
            NumericLiteral::Float(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#float")),
            NumericLiteral::Byte(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#byte")),
            NumericLiteral::Short(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#short")),
            NumericLiteral::NonNegativeInteger(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#nonNegativeInteger")),
            NumericLiteral::UnsignedLong(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#unsignedLong")),
            NumericLiteral::UnsignedInt(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#unsignedInt")),
            NumericLiteral::UnsignedShort(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#unsignedShort")),
            NumericLiteral::UnsignedByte(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#unsignedByte")),
            NumericLiteral::PositiveInteger(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#positiveInteger")),
            NumericLiteral::NegativeInteger(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#negativeInteger")),
            NumericLiteral::NonPositiveInteger(_) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#nonPositiveInteger")),
        }
    }

    /// Creates a decimal literal from a Decimal value.
    pub fn decimal(d: Decimal) -> NumericLiteral {
        NumericLiteral::Decimal(d)
    }

    /// Creates a non-positive integer literal from an i128 value.
    ///
    /// # Errors
    /// Returns error if value is greater than 0.
    pub fn non_positive_integer(n: i128) -> Result<NumericLiteral, RDFError> {
        if n > 0 {
            return Err(RDFError::ConversionError { msg: ("nonPositiveInteger (must be <= 0)".to_string()) });
        }
        Ok(NumericLiteral::NonPositiveInteger(n))
    }

    /// Creates a non-negative integer literal from a u128 value.
    pub fn non_negative_integer(n: u128) -> NumericLiteral {
        NumericLiteral::NonNegativeInteger(n)
    }

    /// Creates a positive integer literal (value > 0).
    ///
    /// # Errors
    /// Returns error if value is 0, as XSD positiveInteger requires > 0.
    pub fn positive_integer(n: u128) -> Result<NumericLiteral, RDFError> {
        if n == 0 {
            return Err(RDFError::ConversionError { msg: ("positiveInteger (must be > 0)".to_string()) });
        }
        Ok(NumericLiteral::PositiveInteger(n))
    }

    /// Creates a negative integer literal from an i128 value.
    ///
    /// # Errors
    /// Returns error if value is greater than or equal to 0.
    pub fn negative_integer(n: i128) -> Result<NumericLiteral, RDFError> {
        if n >= 0 {
            return Err(RDFError::ConversionError { msg: ("negativeInteger (must be < 0)".to_string()) });
        }
        Ok(NumericLiteral::NegativeInteger(n))
    }

    /// Creates an unsigned byte literal (0-255).
    pub fn unsigned_byte(n: u8) -> NumericLiteral {
        NumericLiteral::UnsignedByte(n)
    }

    /// Creates an unsigned short literal (0-65535).
    pub fn unsigned_short(n: u16) -> NumericLiteral {
        NumericLiteral::UnsignedShort(n)
    }

    /// Creates an unsigned int literal.
    pub fn unsigned_int(n: u32) -> NumericLiteral {
        NumericLiteral::UnsignedInt(n)
    }

    /// Creates an unsigned long literal.
    pub fn unsigned_long(n: u64) -> NumericLiteral {
        NumericLiteral::UnsignedLong(n)
    }

    /// Creates a decimal from separate whole and fractional parts.
    ///
    /// # Errors
    /// Returns error if the constructed string cannot be parsed as Decimal.
    pub fn decimal_from_parts(whole: i64, fraction: u32) -> Result<NumericLiteral, RDFError> {
        let s = format!("{whole}.{fraction}");
        let d = Decimal::from_str_exact(&s)
            .map_err(|e| RDFError::ConversionError{ msg: e.to_string() })?;
        Ok(NumericLiteral::Decimal(d))
    }

    /// Creates a byte literal (-128 to 127).
    pub fn byte(d: i8) -> NumericLiteral {
        NumericLiteral::Byte(d)
    }

    /// Creates a short literal (-32768 to 32767).
    pub fn short(d: i16) -> NumericLiteral {
        NumericLiteral::Short(d)
    }

    /// Creates a long literal (64-bit signed integer).
    pub fn long(d: i64) -> NumericLiteral {
        NumericLiteral::Long(d)
    }

    /// Creates a float literal (32-bit).
    pub fn float(d: f32) -> NumericLiteral {
        NumericLiteral::Float(d)
    }

    /// Creates an integer literal (unbounded).
    pub fn integer(n: i128) -> NumericLiteral {
        NumericLiteral::Integer(n)
    }

    /// Creates a double literal (64-bit).
    pub fn double(d: f64) -> NumericLiteral {
        NumericLiteral::Double(d)
    }
}

/// ## Conversion Methods 
impl NumericLiteral {
    // Use macro to generate all decimal_from_* methods
    impl_decimal_from!(decimal_from_f64, f64, from_f64);
    impl_decimal_from!(decimal_from_f32, f32, from_f32);
    impl_decimal_from!(decimal_from_isize, isize, from_isize);
    impl_decimal_from!(decimal_from_i32, i32, from_i32);
    impl_decimal_from!(decimal_from_i64, i64, from_i64);
    impl_decimal_from!(decimal_from_i128, i128, from_i128);
    impl_decimal_from!(decimal_from_u32, u32, from_u32);
    impl_decimal_from!(decimal_from_u64, u64, from_u64);
    impl_decimal_from!(decimal_from_u128, u128, from_u128);

    /// Creates an integer literal from i64.
    pub fn integer_from_i64(d: i64) -> NumericLiteral {
        NumericLiteral::Integer(d as i128)
    }

    /// Converts any numeric literal to Decimal for uniform comparison.
    ///
    /// This method enables cross-type numeric comparisons by normalizing
    /// all variants to the Decimal type.
    pub fn to_decimal(&self) -> Option<Decimal> {
        match self {
            NumericLiteral::Integer(n) => Decimal::from_i128(*n),
            NumericLiteral::Double(d) => Decimal::from_f64(*d),
            NumericLiteral::Decimal(d) => Some(*d),
            NumericLiteral::Long(l) => Decimal::from_i64(*l),
            NumericLiteral::Float(f) => Decimal::from_f32(*f),
            NumericLiteral::Byte(b) => Decimal::from_i8(*b),
            NumericLiteral::Short(s) => Decimal::from_i16(*s),
            NumericLiteral::NonNegativeInteger(n) => Decimal::from_u128(*n),
            NumericLiteral::UnsignedLong(n) => Decimal::from_u64(*n),
            NumericLiteral::UnsignedInt(n) => Decimal::from_u32(*n),
            NumericLiteral::UnsignedShort(n) => Decimal::from_u16(*n),
            NumericLiteral::UnsignedByte(n) => Decimal::from_u8(*n),
            NumericLiteral::PositiveInteger(n) => Decimal::from_u128(*n),
            NumericLiteral::NegativeInteger(n) => Decimal::from_i128(*n),
            NumericLiteral::NonPositiveInteger(n) => Decimal::from_i128(*n),
        }
    }
}

/// ## Utility Methods 
impl NumericLiteral {
    /// Returns the lexical form (string representation) of this literal.
    ///
    /// This is equivalent to the Display implementation.
    pub fn lexical_form(&self) -> String {
        self.to_string()
    }

    /// Checks if this numeric literal is less than another.
    ///
    /// Optimized for integer comparisons; falls back to decimal conversion
    /// for mixed-type comparisons.
    pub fn less_than(&self, other: &NumericLiteral) -> bool {
        match (self, other) {
            // Fast path: direct integer comparison
            (NumericLiteral::Integer(n1), NumericLiteral::Integer(n2)) => n1 < n2,
            // Generic path: convert to decimal for comparison
            (v1, v2) => v1.to_decimal() < v2.to_decimal(),
        }
    }

    /// Checks if this numeric literal is less than or equal to another.
    pub fn less_than_or_eq(&self, other: &NumericLiteral) -> bool {
        match (self, other) {
            (NumericLiteral::Integer(n1), NumericLiteral::Integer(n2)) => n1 <= n2,
            (v1, v2) => v1.to_decimal() <= v2.to_decimal(),
        }
    }

    /// Returns the total number of digits in the literal.
    ///
    /// For decimals, this excludes the decimal point and sign.
    /// Returns None for float/double as they don't have a fixed digit count.
    pub fn total_digits(&self) -> Option<usize> {
        match self {
            // For integer types, count digits in string representation
            NumericLiteral::Integer(d) => Some(d.abs().to_string().len()),
            NumericLiteral::Long(d) => Some(d.abs().to_string().len()),
            NumericLiteral::Byte(d) => Some(d.abs().to_string().len()),
            NumericLiteral::Short(d) => Some(d.abs().to_string().len()),
            NumericLiteral::NonNegativeInteger(d) => Some(d.to_string().len()),
            NumericLiteral::UnsignedLong(d) => Some(d.to_string().len()),
            NumericLiteral::UnsignedInt(d) => Some(d.to_string().len()),
            NumericLiteral::UnsignedShort(d) => Some(d.to_string().len()),
            NumericLiteral::UnsignedByte(d) => Some(d.to_string().len()),
            NumericLiteral::PositiveInteger(d) => Some(d.to_string().len()),
            NumericLiteral::NegativeInteger(d) => Some(d.abs().to_string().len()),
            NumericLiteral::NonPositiveInteger(d) => Some(d.abs().to_string().len()),
            // For decimals, normalize and count only digits
            NumericLiteral::Decimal(d) => {
                let normalized = d.normalize();
                let digit_count = normalized
                    .to_string()
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .count();
                Some(digit_count)
            }
            // Float/double don't have meaningful total digits
            NumericLiteral::Double(_) | NumericLiteral::Float(_) => None,
        }
    }

    /// Returns the number of fractional digits.
    ///
    /// Returns 0 for integer types, counts digits after decimal point for decimals,
    /// and None for float/double.
    pub fn fraction_digits(&self) -> Option<usize> {
        match self {
            // All integer types have 0 fractional digits
            NumericLiteral::Integer(_)
            | NumericLiteral::Long(_)
            | NumericLiteral::NonNegativeInteger(_)
            | NumericLiteral::UnsignedLong(_)
            | NumericLiteral::UnsignedInt(_)
            | NumericLiteral::UnsignedShort(_)
            | NumericLiteral::UnsignedByte(_)
            | NumericLiteral::PositiveInteger(_)
            | NumericLiteral::NegativeInteger(_)
            | NumericLiteral::NonPositiveInteger(_)
            | NumericLiteral::Byte(_)
            | NumericLiteral::Short(_) => Some(0),
            // For decimals, find position of decimal point and count digits after it
            NumericLiteral::Decimal(d) => {
                let s = d.to_string();
                Some(s.find('.').map_or(0, |pos| s.len() - pos - 1))
            }
            // Float/double don't have meaningful fraction digits
            NumericLiteral::Double(_) | NumericLiteral::Float(_) => None,
        }
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

/// Eq implementation requires all values to be comparable.
///
/// Note: This implementation is consistent with Hash because both use
/// the exact variant type + value, not normalized decimal values.
impl Eq for NumericLiteral {}

/// Hash implementation that hashes both the variant type and the value.
impl Hash for NumericLiteral {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // First hash the discriminant to distinguish between variants
        core::mem::discriminant(self).hash(state);

        // Then hash the actual value
        match self {
            NumericLiteral::Integer(n) => n.hash(state),
            NumericLiteral::Byte(b) => b.hash(state),
            NumericLiteral::Short(s) => s.hash(state),
            NumericLiteral::NonNegativeInteger(n) => n.hash(state),
            NumericLiteral::UnsignedLong(n) => n.hash(state),
            NumericLiteral::UnsignedInt(n) => n.hash(state),
            NumericLiteral::UnsignedShort(n) => n.hash(state),
            NumericLiteral::UnsignedByte(n) => n.hash(state),
            NumericLiteral::PositiveInteger(n) => n.hash(state),
            NumericLiteral::NegativeInteger(n) => n.hash(state),
            NumericLiteral::NonPositiveInteger(n) => n.hash(state),
            NumericLiteral::Long(l) => l.hash(state),
            NumericLiteral::Decimal(d) => d.hash(state),
            // For floats, hash the bit representation to ensure consistency
            NumericLiteral::Double(d) => d.to_bits().hash(state),
            NumericLiteral::Float(f) => f.to_bits().hash(state),
        }
    }
}

/// Custom serialization to preserve numeric types in target format.
impl Serialize for NumericLiteral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NumericLiteral::Integer(n) => serializer.serialize_i128(*n),
            NumericLiteral::Decimal(d) => {
                // Try to convert to f64 for JSON compatibility
                match d.to_f64() {
                    Some(f) => serializer.serialize_f64(f),
                    None => serializer.serialize_str(&d.to_string()),
                }
            }
            NumericLiteral::Double(d) => serializer.serialize_f64(*d),
            NumericLiteral::Long(n) => serializer.serialize_i64(*n),
            NumericLiteral::Float(f) => serializer.serialize_f32(*f),
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

/// Display implementation for human-readable output.
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

/// Ordering based on decimal conversion for cross-type comparisons.
impl PartialOrd for NumericLiteral {
    // Convert both to Decimal and compare
    // Returns None if conversion fails (e.g., NaN)
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_decimal = self.to_decimal()?;
        let other_decimal = other.to_decimal()?;
        self_decimal.partial_cmp(&other_decimal)
    }
}

// ============================================================================
// Conversions
// ============================================================================

/// Parse numeric literals from strings.
///
/// Attempts to parse as integer first, then float, then decimal.
impl TryFrom<&str> for NumericLiteral {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Try integer first (most common case)
        if let Ok(i) = value.parse::<i128>() {
            return Ok(NumericLiteral::Integer(i));
        }
        // Then try float
        if let Ok(f) = value.parse::<f64>() {
            return Ok(NumericLiteral::Double(f));
        }
        // Finally try decimal
        if let Ok(d) = Decimal::from_str_exact(value) {
            return Ok(NumericLiteral::Decimal(d));
        }

        Err(format!("Cannot parse '{value}' as NumericLiteral"))
    }
}

/// Conversion to oxrdf::Literal for RDF serialization.
impl From<NumericLiteral> for oxrdf::Literal {
    fn from(n: NumericLiteral) -> Self {
        match n {
            NumericLiteral::Integer(i) => oxrdf::Literal::new_typed_literal(
                &i.to_string(),
                oxrdf::vocab::xsd::INTEGER,
            ),
            NumericLiteral::Decimal(d) => {
                // Try converting to f64, otherwise use typed literal
                match d.to_f64() {
                    Some(decimal) => oxrdf::Literal::from(decimal),
                    None => oxrdf::Literal::new_typed_literal(
                        &d.to_string(),
                        oxrdf::vocab::xsd::DECIMAL,
                    ),
                }
            }
            NumericLiteral::Double(d) => oxrdf::Literal::from(d),
            NumericLiteral::Long(l) => oxrdf::Literal::new_typed_literal(
                &l.to_string(),
                oxrdf::vocab::xsd::LONG,
            ),
            NumericLiteral::Float(f) => oxrdf::Literal::from(f),
            NumericLiteral::Byte(b) => oxrdf::Literal::new_typed_literal(
                &b.to_string(),
                oxrdf::vocab::xsd::BYTE,
            ),
            NumericLiteral::Short(s) => oxrdf::Literal::from(s),
            NumericLiteral::NonNegativeInteger(n) => oxrdf::Literal::new_typed_literal(
                &n.to_string(),
                oxrdf::vocab::xsd::NON_NEGATIVE_INTEGER,
            ),
            NumericLiteral::UnsignedLong(n) => {
                oxrdf::Literal::new_typed_literal(&n.to_string(), oxrdf::vocab::xsd::UNSIGNED_LONG)
            }
            NumericLiteral::UnsignedInt(n) => {
                oxrdf::Literal::new_typed_literal(&n.to_string(), oxrdf::vocab::xsd::UNSIGNED_INT)
            }
            NumericLiteral::UnsignedShort(n) => {
                oxrdf::Literal::new_typed_literal(&n.to_string(), oxrdf::vocab::xsd::UNSIGNED_SHORT)
            }
            NumericLiteral::UnsignedByte(n) => {
                oxrdf::Literal::new_typed_literal(&n.to_string(), oxrdf::vocab::xsd::UNSIGNED_BYTE)
            }
            NumericLiteral::PositiveInteger(n) => oxrdf::Literal::new_typed_literal(
                &n.to_string(),
                oxrdf::vocab::xsd::POSITIVE_INTEGER,
            ),
            NumericLiteral::NegativeInteger(n) => oxrdf::Literal::new_typed_literal(
                &n.to_string(),
                oxrdf::vocab::xsd::NEGATIVE_INTEGER,
            ),
            NumericLiteral::NonPositiveInteger(n) => oxrdf::Literal::new_typed_literal(
                &n.to_string(),
                oxrdf::vocab::xsd::NON_POSITIVE_INTEGER,
            ),
        }
    }
}
