use crate::SLiteral;
use crate::XsdDateTime;
use crate::lang::Lang;
use rust_decimal::Decimal;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

/// Types that implement this trait can be used as RDF Literals
pub trait Literal: Debug + Clone + Display + PartialEq + Eq + Hash {
    fn lexical_form(&self) -> &str;

    fn lang(&self) -> Option<Lang>;

    fn datatype(&self) -> &str;

    fn as_sliteral(&self) -> Option<SLiteral>;

    fn as_bool(&self) -> Option<bool> {
        if self.datatype() == "http://www.w3.org/2001/XMLSchema#boolean" {
            match self.lexical_form() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            }
        } else {
            None
        }
    }

    fn as_integer(&self) -> Option<isize> {
        if self.datatype() == "http://www.w3.org/2001/XMLSchema#integer" {
            self.lexical_form().parse().ok()
        } else {
            None
        }
    }

    fn as_date_time(&self) -> Option<XsdDateTime> {
        if self.datatype() == "http://www.w3.org/2001/XMLSchema#dateTime" {
            XsdDateTime::new(self.lexical_form()).ok()
        } else {
            None
        }
    }

    fn as_double(&self) -> Option<f64> {
        if self.datatype() == "http://www.w3.org/2001/XMLSchema#double" {
            self.lexical_form().parse().ok()
        } else {
            None
        }
    }

    fn as_decimal(&self) -> Option<Decimal> {
        if self.datatype() == "http://www.w3.org/2001/XMLSchema#decimal" {
            self.lexical_form().parse().ok()
        } else {
            None
        }
    }

    /*fn as_literal(&self) -> SLiteral {
        if let Some(bool) = self.as_bool() {
            SLiteral::boolean(bool)
        } else if let Some(int) = self.as_integer() {
            SLiteral::integer(int)
        } else if let Some(decimal) = self.as_double() {
            SLiteral::double(decimal)
        } else if let Some(decimal) = self.as_decimal() {
            SLiteral::decimal(decimal)
        } else if let Some(date_time) = self.as_date_time() {
            SLiteral::DatetimeLiteral(date_time)
        } else if let Some(lang) = self.lang() {
            SLiteral::lang_str(self.lexical_form(), Lang::new_unchecked(lang))
        } else {
            SLiteral::str(self.lexical_form())
        }
    }*/
}
