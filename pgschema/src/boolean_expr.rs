use std::fmt::Display;

use crate::{pgs_error::PgsError, value::Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BooleanExpr {
    True,
    False,
    And(Box<BooleanExpr>, Box<BooleanExpr>),
    Or(Box<BooleanExpr>, Box<BooleanExpr>),
    Not(Box<BooleanExpr>),
    Equals(Value),
    GreaterThan(Value),
    LessThan(Value),
    GreaterThanOrEqual(Value),
    LessThanOrEqual(Value),
    Regex(String),
}

impl BooleanExpr {
    pub fn check(&self, value: &Value) -> Result<bool, PgsError> {
        match self {
            BooleanExpr::And(a, b) => {
                let check_a = a.check(value)?;
                let check_b = b.check(value)?;
                Ok(check_a && check_b)
            },
            BooleanExpr::Or(a, b) => {
                let check_a = a.check(value)?;
                let check_b = b.check(value)?;
                Ok(check_a || check_b)
            },
            BooleanExpr::Not(expr) => {
                let check = expr.check(value)?;
                Ok(!check)
            },
            BooleanExpr::Equals(v) => Ok(value == v),
            BooleanExpr::GreaterThan(v) => value.greater_than(v),
            BooleanExpr::LessThan(v) => value.less_than(v),
            BooleanExpr::GreaterThanOrEqual(v) => value.greater_than_or_equal(v),
            BooleanExpr::LessThanOrEqual(v) => value.less_than_or_equal(v),
            BooleanExpr::Regex(pattern) => value.regex_match(pattern),
            BooleanExpr::True => Ok(true),
            BooleanExpr::False => Ok(false),
        }
    }
}

impl Display for BooleanExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BooleanExpr::And(a, b) => write!(f, "({} AND {})", a, b),
            BooleanExpr::Or(a, b) => write!(f, "({} OR {})", a, b),
            BooleanExpr::Not(expr) => write!(f, "NOT ({})", expr),
            BooleanExpr::Equals(value) => write!(f, "(== {})", value),
            BooleanExpr::GreaterThan(value) => write!(f, "(> {})", value),
            BooleanExpr::LessThan(value) => write!(f, "(< {})", value),
            BooleanExpr::GreaterThanOrEqual(value) => write!(f, "(>= {})", value),
            BooleanExpr::LessThanOrEqual(value) => write!(f, "(<= {})", value),
            BooleanExpr::Regex(pattern) => write!(f, "(REGEX({})", pattern),
            BooleanExpr::True => write!(f, "TRUE"),
            BooleanExpr::False => write!(f, "FALSE"),
        }
    }
}
