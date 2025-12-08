use crate::{
    boolean_expr::BooleanExpr, card::Card, evidence::Evidence, pgs_error::PgsError, value::Value,
};
use either::Either;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    Any,
    StringType(Card),
    IntegerType(Card),
    DateType(Card),
    BoolType(Card),
    Intersection(Box<ValueType>, Box<ValueType>),
    Union(Box<ValueType>, Box<ValueType>),
    Cond(BooleanExpr),
}

impl ValueType {
    pub fn integer(card: Card) -> Self {
        ValueType::IntegerType(card)
    }

    pub fn string(card: Card) -> Self {
        ValueType::StringType(card)
    }

    pub fn date(card: Card) -> Self {
        ValueType::DateType(card)
    }
    pub fn bool(card: Card) -> Self {
        ValueType::BoolType(card)
    }
    pub fn intersection(a: ValueType, b: ValueType) -> Self {
        ValueType::Intersection(Box::new(a), Box::new(b))
    }
    pub fn union(a: ValueType, b: ValueType) -> Self {
        ValueType::Union(Box::new(a), Box::new(b))
    }
    pub fn cond(expr: BooleanExpr) -> Self {
        ValueType::Cond(expr)
    }
    pub fn conforms(&self, values: &HashSet<Value>) -> Either<Vec<PgsError>, Vec<Evidence>> {
        match self {
            ValueType::StringType(card) => {
                if !card.contains(values.len()) {
                    return Either::Left(vec![PgsError::CardinalityMismatch {
                        expected: card.clone(),
                        count: values.len(),
                    }]);
                }
                check_all(values, |v| v.is_string(), "is_string")
            }
            ValueType::IntegerType(card) => {
                if !card.contains(values.len()) {
                    return Either::Left(vec![PgsError::CardinalityMismatch {
                        expected: card.clone(),
                        count: values.len(),
                    }]);
                }
                check_all(values, |v| v.is_integer(), "is_integer")
            }
            ValueType::DateType(card) => {
                if !card.contains(values.len()) {
                    return Either::Left(vec![PgsError::CardinalityMismatch {
                        expected: card.clone(),
                        count: values.len(),
                    }]);
                }
                check_all(values, |v| v.is_date(), "is_date")
            }
            ValueType::Intersection(a, b) => {
                let a_evidence = a.conforms(values);
                let b_evidence = b.conforms(values);
                match (a_evidence, b_evidence) {
                    (Either::Left(errs_a), Either::Left(errs_b)) => {
                        Either::Left([errs_a, errs_b].concat())
                    }
                    (Either::Left(errs), Either::Right(_)) => Either::Left(errs),
                    (Either::Right(_), Either::Left(errs)) => Either::Left(errs),
                    (Either::Right(evidences_a), Either::Right(evidences_b)) => {
                        Either::Right([evidences_a, evidences_b].concat())
                    }
                }
            }
            ValueType::Union(a, b) => {
                let a_evidence = a.conforms(values);
                let b_evidence = b.conforms(values);
                match (a_evidence, b_evidence) {
                    (Either::Left(errs_a), Either::Left(errs_b)) => {
                        Either::Left([errs_a, errs_b].concat())
                    }
                    (Either::Left(_), Either::Right(es)) => Either::Right(es),
                    (Either::Right(es), Either::Left(_)) => Either::Right(es),
                    (Either::Right(evidences_a), Either::Right(evidences_b)) => {
                        Either::Right([evidences_a, evidences_b].concat())
                    }
                }
            }
            ValueType::Cond(cond) => {
                for value in values {
                    match cond.check(value) {
                        Ok(true) => continue,
                        Ok(false) => {
                            return Either::Left(vec![PgsError::ConditionFailed {
                                condition: format!("{}", cond),
                                value: format!("{}", value),
                            }]);
                        }
                        Err(e) => return Either::Left(vec![e]),
                    }
                }
                Either::Right(vec![Evidence::ConditionPassed {
                    condition: format!("{}", cond),
                    values: format!("{:?}", values),
                }])
            }
            ValueType::Any => Either::Right(vec![Evidence::Any {
                values: format!("{:?}", values),
            }]),
            ValueType::BoolType(card) => {
                if !card.contains(values.len()) {
                    return Either::Left(vec![PgsError::CardinalityMismatch {
                        expected: card.clone(),
                        count: values.len(),
                    }]);
                }
                check_all(values, |v| v.is_bool(), "is_bool")
            }
        }
    }
}

fn check_all<F>(
    values: &HashSet<Value>,
    predicate: F,
    predicate_name: &str,
) -> Either<Vec<PgsError>, Vec<Evidence>>
where
    F: Fn(&Value) -> bool,
{
    if values.iter().all(&predicate) {
        Either::Right(vec![]) // All values conform
    } else {
        Either::Left(vec![PgsError::PredicateFailed {
            predicate_name: predicate_name.to_string(),
            value: values
                .iter()
                .find(|v| !predicate(v))
                .cloned()
                .unwrap_or(Value::String("".to_string())),
        }])
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::StringType(card) => write!(f, "String({})", card),
            ValueType::IntegerType(card) => write!(f, "Integer({})", card),
            ValueType::DateType(card) => write!(f, "Date({})", card),
            ValueType::Intersection(a, b) => write!(f, "({} ∩ {})", a, b),
            ValueType::Union(a, b) => write!(f, "({} ∪ {})", a, b),
            ValueType::Cond(expr) => write!(f, "Condition({})", expr),
            ValueType::Any => write!(f, "ANY"),
            ValueType::BoolType(card) => write!(f, "Bool({})", card),
        }
    }
}
