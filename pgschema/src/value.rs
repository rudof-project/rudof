use std::fmt::Display;

use regex::Regex;
use time::{Date, macros::format_description};

use crate::pgs_error::PgsError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    String(String),
    Integer(i32),
    Date(Date), // Simplified for this example
    Bool(bool),
}

impl Value {
    pub fn str(s: &str) -> Self {
        Value::String(s.to_string())
    }

    pub fn int(i: i32) -> Self {
        Value::Integer(i)
    }

    pub fn date(repr: &str) -> Result<Self, PgsError> {
        let format = format_description!("[year]-[month]-[day]");
        let date = Date::parse(repr, &format).map_err(|e| PgsError::InvalidDate {
            date: repr.to_string(),
            error: e.to_string(),
        })?;
        Ok(Value::Date(date))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn true_() -> Self {
        Value::Bool(true)
    }

    pub fn false_() -> Self {
        Value::Bool(false)
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    pub fn is_date(&self) -> bool {
        matches!(self, Value::Date(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn greater_than(&self, other: &Value) -> Result<bool, PgsError> {
        match (self, other) {
            (Value::Integer(i), Value::Integer(v)) => Ok(i > v),
            (Value::Date(i), Value::Date(v)) => Ok(i > v),
            _ => Err(PgsError::TypeMismatch {
                operation: ">".into(),
                expected: "Integer".into(),
                found: format!("{:?}", other),
            }),
        }
    }

    pub fn less_than(&self, other: &Value) -> Result<bool, PgsError> {
        match (self, other) {
            (Value::Integer(i), Value::Integer(v)) => Ok(i < v),
            (Value::Date(i), Value::Date(v)) => Ok(i < v),
            _ => Err(PgsError::TypeMismatch {
                operation: "<".into(),
                expected: "Integer".into(),
                found: format!("{:?}", other),
            }),
        }
    }

    pub fn less_than_or_equal(&self, other: &Value) -> Result<bool, PgsError> {
        match (self, other) {
            (Value::Integer(i), Value::Integer(v)) => Ok(i <= v),
            (Value::Date(i), Value::Date(v)) => Ok(i <= v),
            _ => Err(PgsError::TypeMismatch {
                operation: "<=".into(),
                expected: "Integer".into(),
                found: format!("{:?}", other),
            }),
        }
    }

    pub fn greater_than_or_equal(&self, other: &Value) -> Result<bool, PgsError> {
        match (self, other) {
            (Value::Integer(i), Value::Integer(v)) => Ok(i >= v),
            (Value::Date(i), Value::Date(v)) => Ok(i >= v),
            _ => Err(PgsError::TypeMismatch {
                operation: ">=".into(),
                expected: "Integer".into(),
                found: format!("{:?}", other),
            }),
        }
    }

    pub fn regex_match(&self, pattern: &str) -> Result<bool, PgsError> {
        match self {
            Value::String(s) => {
                let regex = Regex::new(pattern).map_err(|e| PgsError::InvalidRegex {
                    pattern: pattern.to_string(),
                    error: e.to_string(),
                })?;
                let matches = regex.is_match(s);
                Ok(matches)
            }
            _ => Err(PgsError::TypeMismatch {
                operation: "regex_match".into(),
                expected: "String".into(),
                found: format!("{:?}", self),
            }),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Date(d) => write!(f, "{}", d),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
        /*match (self, other) {
            (Value::Integer(i1), Value::Integer(i2)) => i1.partial_cmp(i2),
            (Value::String(s1), Value::String(s2)) => s1.partial_cmp(s2),
            (Value::Date(d1), Value::Date(d2)) => d1.partial_cmp(d2),
            _ => None,
        }*/
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Integer(i1), Value::Integer(i2)) => i1.cmp(i2),
            (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
            (Value::Date(d1), Value::Date(d2)) => d1.cmp(d2),
            _ => std::cmp::Ordering::Equal, // Fallback for different types
        }
    }
}
