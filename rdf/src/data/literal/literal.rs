use rust_decimal::Decimal;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use prefixmap::IriRef;

use crate::{
    data::literal::{Lang, ConcreteLiteral, XsdDateTime},
    vocab::{XSD_BOOLEAN, XSD_DATETIME, XSD_DECIMAL, XSD_DOUBLE, XSD_INTEGER},
};


/// Types that implement this trait can be used as RDF Literals.
///
/// This trait provides methods for accessing literal properties and converting
/// literals to specific Rust types based on their XSD datatype.
#[allow(dead_code)] // TODO: Remove #[allow(dead_code)] once trait implementations are added
pub trait Literal: Debug + Clone + Display + PartialEq + Eq + Hash {
    /// Returns the lexical form of the literal as a string slice.
    fn lexical_form(&self) -> &str;

    /// Returns the language tag if this is a language-tagged literal.
    fn lang(&self) -> Option<Lang>;

    /// Returns the datatype IRI of this literal.
    fn datatype(&self) -> &IriRef;

    /// Converts this literal to an `ConcreteLiteral` if possible.
    fn as_concrete_literal(&self) -> Option<ConcreteLiteral>;

    /// Attempts to convert this literal to a boolean value.
    ///
    /// Returns `Some(bool)` if the literal has datatype `xsd:boolean` and
    /// a valid lexical form ("true" or "false").
    fn as_bool(&self) -> Option<bool> {
        if self.datatype() != &*XSD_BOOLEAN {
            return None;
        }

        match self.lexical_form() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    /// Attempts to convert this literal to an integer value.
    ///
    /// Returns `Some(isize)` if the literal has datatype `xsd:integer` and
    /// a valid parseable lexical form.
    fn as_integer(&self) -> Option<isize> {
        if self.datatype() != &*XSD_INTEGER {
            return None;
        }

        self.lexical_form().parse().ok()
    }

    /// Attempts to convert this literal to a datetime value.
    ///
    /// Returns `Some(XsdDateTime)` if the literal has datatype `xsd:dateTime` and
    /// a valid parseable lexical form.
    fn as_date_time(&self) -> Option<XsdDateTime> {
        if self.datatype() != &*XSD_DATETIME {
            return None;
        }

        XsdDateTime::new(self.lexical_form()).ok()
    }

    /// Attempts to convert this literal to a double-precision float value.
    ///
    /// Returns `Some(f64)` if the literal has datatype `xsd:double` and
    /// a valid parseable lexical form.
    fn as_double(&self) -> Option<f64> {
        if self.datatype() != &*XSD_DOUBLE {
            return None;
        }

        self.lexical_form().parse().ok()
    }

    /// Attempts to convert this literal to a decimal value.
    ///
    /// Returns `Some(Decimal)` if the literal has datatype `xsd:decimal` and
    /// a valid parseable lexical form.
    fn as_decimal(&self) -> Option<Decimal> {
        if self.datatype() != &*XSD_DECIMAL {
            return None;
        }

        self.lexical_form().parse().ok()
    }
}
