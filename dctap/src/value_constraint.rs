use iri_s::IriS;
use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum ValueConstraint {
    PickList(Vec<Value>),
    Pattern(String),
    IRIStem(IriS),
    LanguageTag(String),
    MinLength(usize),
    MaxLength(usize),
    MinExclusive(Number),
    MinInclusive(Number),
    MaxExclusive(Number),
    MaxInclusive(Number),
}

impl ValueConstraint {
    pub fn picklist(values: Vec<Value>) -> ValueConstraint {
        ValueConstraint::PickList(values)
    }

    pub fn pattern(str: &str) -> ValueConstraint {
        ValueConstraint::Pattern(str.to_string())
    }
}

impl Display for ValueConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueConstraint::PickList(vs) => {
                write!(f, "[{}]", vs.iter().format(" | "))?;
            }
            ValueConstraint::Pattern(s) => write!(f, "Pattern({s})")?,
            ValueConstraint::IRIStem(s) => write!(f, "IRIStem({s})")?,
            ValueConstraint::LanguageTag(s) => write!(f, "LanguageTag({s})")?,
            ValueConstraint::MinLength(n) => write!(f, "MinLength({n})")?,
            ValueConstraint::MaxLength(n) => write!(f, "MaxLength({n})")?,
            ValueConstraint::MinInclusive(n) => write!(f, "MinInclusive({n})")?,
            ValueConstraint::MaxExclusive(n) => write!(f, "MaxInclusive({n})")?,
            ValueConstraint::MinExclusive(n) => write!(f, "MinExclusive({n})")?,
            ValueConstraint::MaxInclusive(n) => write!(f, "MaxExclusive({n})")?,
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

pub enum Value {
    Iri(IriS),
    Str(String),
}

impl Value {
    pub fn new(str: &str) -> Value {
        Value::Str(str.to_string())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Iri(iri) => write!(f, "Iri({iri})")?,
            Value::Str(s) => write!(f, "{s}")?,
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum ValueConstraintType {
    PickList,
    Pattern,
    IRIStem,
    LanguageTag,
    MinLength,
    MaxLength,
    MinInclusive,
    MinExclusive,
    MaxInclusive,
    MaxExclusive,
}

impl Default for ValueConstraintType {
    fn default() -> Self {
        ValueConstraintType::PickList
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

pub enum Number {
    Int(i64),
    Double(f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(n) => write!(f, "{n}")?,
            Number::Double(n) => write!(f, "{n}")?,
        }
        Ok(())
    }
}
