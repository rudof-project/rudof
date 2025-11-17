use crate::boolean_expr::BooleanExpr;
use crate::card::Card;
use crate::formal_base_type::FormalBaseType;
use crate::key::Key;
use crate::record_type::RecordType;

use crate::pgs_error::PgsError;
use crate::value_type::ValueType;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValueSpec {
    Closed(PropertyValue),
    Open(PropertyValue),
}

impl PropertyValueSpec {
    pub fn open(property_value: PropertyValue) -> Self {
        PropertyValueSpec::Open(property_value)
    }

    pub fn closed(property_value: PropertyValue) -> Self {
        PropertyValueSpec::Closed(property_value)
    }

    pub fn semantics(&self) -> Result<FormalBaseType, PgsError> {
        let content_semantics = match self {
            PropertyValueSpec::Closed(pv) => pv.semantics(),
            PropertyValueSpec::Open(pv) => {
                let open_semantics: HashSet<_> =
                    pv.semantics().into_iter().map(|v| v.with_open()).collect();
                open_semantics
            }
        };
        Ok(FormalBaseType::new().with_content(content_semantics))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValue {
    EachOf(Box<PropertyValue>, Box<PropertyValue>),
    OneOf(Box<PropertyValue>, Box<PropertyValue>),
    Property(Key, TypeSpec),
    OptionalProperty(Key, TypeSpec),
    Empty,
}

impl PropertyValue {
    pub fn property(key: Key, type_spec: TypeSpec) -> Self {
        PropertyValue::Property(key, type_spec)
    }

    pub fn optional_property(key: Key, type_spec: TypeSpec) -> Self {
        PropertyValue::OptionalProperty(key, type_spec)
    }

    pub fn each_of(left: PropertyValue, right: PropertyValue) -> Self {
        PropertyValue::EachOf(Box::new(left), Box::new(right))
    }

    pub fn one_of(left: PropertyValue, right: PropertyValue) -> Self {
        PropertyValue::OneOf(Box::new(left), Box::new(right))
    }

    pub fn empty() -> Self {
        PropertyValue::Empty
    }

    pub fn semantics(&self) -> HashSet<RecordType> {
        match self {
            PropertyValue::EachOf(left, right) => {
                let left_semantics = left.semantics();
                let right_semantics = right.semantics();
                combine_semantics_sets(left_semantics, right_semantics)
            }
            PropertyValue::OneOf(left, right) => {
                let left_semantics = left.semantics();
                let right_semantics = right.semantics();
                //left_semantics.union(&right_semantics)
                union_semantics_sets(left_semantics, right_semantics)
            }
            PropertyValue::Property(p, type_spec) => {
                let record = RecordType::new().with_key_value(p.str(), type_spec.to_value_type());
                HashSet::from_iter(vec![record])
            }
            PropertyValue::OptionalProperty(p, type_spec) => {
                let record = RecordType::new().with_key_value(p.str(), type_spec.to_value_type());
                HashSet::from_iter(vec![record, RecordType::empty()])
            }
            PropertyValue::Empty => HashSet::new(),
        }
    }
}

impl Display for PropertyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyValue::EachOf(left, right) => write!(f, "{} , {}", left, right),
            PropertyValue::OneOf(left, right) => write!(f, "({} | {})", left, right),
            PropertyValue::Property(key, type_spec) => write!(f, "{}: {}", key, type_spec),
            PropertyValue::OptionalProperty(key, type_spec) => {
                write!(f, "{}: {} (optional)", key, type_spec)
            }
            PropertyValue::Empty => write!(f, "Empty"),
        }
    }
}

fn combine_semantics_sets(
    left: HashSet<RecordType>,
    right: HashSet<RecordType>,
) -> HashSet<RecordType> {
    let mut combined = HashSet::new();
    for l in left {
        for r in &right {
            combined.insert(l.combine(r));
        }
    }
    combined
}

fn union_semantics_sets(
    left: HashSet<RecordType>,
    right: HashSet<RecordType>,
) -> HashSet<RecordType> {
    let mut combined = HashSet::new();
    for l in left {
        combined.insert(l);
    }
    for r in right {
        combined.insert(r);
    }
    combined
}

impl PropertyValueSpec {}

impl Display for PropertyValueSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyValueSpec::Closed(pv) => write!(f, "{}", pv),
            PropertyValueSpec::Open(pv) => write!(f, "{} OPEN", pv),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSpec {
    type_def: Type,
}

impl TypeSpec {
    pub fn string(card: Card) -> Self {
        TypeSpec {
            type_def: Type::ValueType(ValueType::string(card)),
        }
    }

    pub fn integer(card: Card) -> Self {
        TypeSpec {
            type_def: Type::ValueType(ValueType::integer(card)),
        }
    }

    pub fn date(card: Card) -> Self {
        TypeSpec {
            type_def: Type::ValueType(ValueType::date(card)),
        }
    }

    pub fn bool(card: Card) -> Self {
        TypeSpec {
            type_def: Type::ValueType(ValueType::bool(card)),
        }
    }

    pub fn any() -> Self {
        TypeSpec {
            type_def: Type::ValueType(ValueType::Any),
        }
    }

    pub fn cond(value_type: ValueType, cond: BooleanExpr) -> Self {
        TypeSpec {
            type_def: Type::Cond(value_type, cond),
        }
    }

    pub fn intersection(a: TypeSpec, b: TypeSpec) -> Self {
        TypeSpec {
            type_def: Type::Conjunction(Box::new(a.type_def), Box::new(b.type_def)),
        }
    }

    pub fn union(a: TypeSpec, b: TypeSpec) -> Self {
        TypeSpec {
            type_def: Type::Disjunction(Box::new(a.type_def), Box::new(b.type_def)),
        }
    }

    pub fn to_value_type(&self) -> ValueType {
        self.type_def.to_value_type()
    }
}

impl Display for TypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_def.to_value_type())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Conjunction(Box<Type>, Box<Type>),
    Disjunction(Box<Type>, Box<Type>),
    ValueType(ValueType),
    Cond(ValueType, BooleanExpr),
}

impl Type {
    pub fn to_value_type(&self) -> ValueType {
        match self {
            Type::ValueType(value_type) => value_type.clone(),
            Type::Conjunction(a, b) => {
                let avt: ValueType = (*a).to_value_type();
                let bvt: ValueType = (*b).to_value_type();
                ValueType::intersection(avt, bvt)
            }
            Type::Disjunction(a, b) => ValueType::union(a.to_value_type(), b.to_value_type()),
            Type::Cond(value_type, cond) => {
                let vt_cond = ValueType::cond(cond.clone());
                ValueType::intersection(value_type.clone(), vt_cond)
            }
        }
    }
}
