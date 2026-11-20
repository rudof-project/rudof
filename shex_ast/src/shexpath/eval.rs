//! Evaluation of a ShExPath over a ShExJ document.

use crate::shexpath::{
    AttributeOrAny, Comparator, Comparison, Filter, Operand, RValue, Selector, ShExPath, ShExPathError, ShExPathItem,
    Step,
};
use prefixmap::IriRef;
use serde_json::Value;
use std::borrow::Cow;

/// Evaluates paths over one document.
pub(crate) struct Evaluator;

impl Evaluator {
    pub(crate) fn new() -> Self {
        Evaluator
    }

    /// The value `path` produces from `items`.
    pub(crate) fn eval_path<'a>(
        &self,
        path: &ShExPath,
        items: Vec<ShExPathItem<'a>>,
    ) -> Result<Vec<ShExPathItem<'a>>, ShExPathError> {
        match path {
            ShExPath::Path(steps) => steps.iter().try_fold(items, |items, step| self.eval_step(step, items)),
            _ => Err(ShExPathError::NotImplemented {
                feature: "junctions".to_string(),
            }),
        }
    }

    /// Applies `step` to every item and concatenates what it reaches, then applies its filters to
    /// that whole value, left to right, each to what the one before it left.
    fn eval_step<'a>(&self, step: &Step, items: Vec<ShExPathItem<'a>>) -> Result<Vec<ShExPathItem<'a>>, ShExPathError> {
        let mut selected = Vec::new();
        for item in items {
            match &step.selector {
                Selector::Child(attribute) => selected.extend(children(&item, attribute)),
                _ => {
                    return Err(ShExPathError::NotImplemented {
                        feature: "axes".to_string(),
                    });
                },
            }
        }
        step.filters
            .iter()
            .try_fold(selected, |items, filter| self.apply_filter(filter, items))
    }

    /// The items of `items` that `filter` keeps. It is evaluated once per item, and knows the item,
    /// the value it is trimming and the item's position in it.
    fn apply_filter<'a>(
        &self,
        filter: &Filter,
        items: Vec<ShExPathItem<'a>>,
    ) -> Result<Vec<ShExPathItem<'a>>, ShExPathError> {
        let mut kept = Vec::new();
        for (index, item) in items.iter().enumerate() {
            if self.keeps(filter, item, index, items.len())? {
                kept.push(item.clone());
            }
        }
        Ok(kept)
    }

    fn keeps(
        &self,
        filter: &Filter,
        item: &ShExPathItem<'_>,
        index: usize,
        count: usize,
    ) -> Result<bool, ShExPathError> {
        match filter {
            // "exactly the filter [type = T]"
            Filter::TermType(term_type) => {
                Ok(item.value().get("type").and_then(Value::as_str) == Some(term_type.as_str()))
            },
            Filter::Test {
                assert,
                operand,
                comparison,
            } => {
                let values = self.operand_values(operand, item, index, count)?;
                let holds = match comparison {
                    None => effective_boolean_value(&values),
                    Some(comparison) => compare(values.first().map(Cow::as_ref), comparison),
                };
                if *assert && !holds {
                    Err(ShExPathError::AssertionFailed {
                        location: item.location().to_string(),
                    })
                } else {
                    Ok(holds)
                }
            },
        }
    }

    /// The value of a filter's operand for an item: what a path selects from it, or its position, or
    /// the size of the value being trimmed.
    fn operand_values<'a>(
        &self,
        operand: &Operand,
        item: &ShExPathItem<'a>,
        index: usize,
        count: usize,
    ) -> Result<Vec<Cow<'a, Value>>, ShExPathError> {
        match operand {
            Operand::Path(path) => Ok(self
                .eval_path(path, vec![item.clone()])?
                .into_iter()
                .map(|item| Cow::Borrowed(item.value()))
                .collect()),
            Operand::Index => Ok(vec![Cow::Owned(Value::from(index))]),
            Operand::Count => Ok(vec![Cow::Owned(Value::from(count))]),
        }
    }
}

/// `child::attribute`: the value of that property, if the item has it. `child::*`: every property
/// value of an object, or every member of an array.
fn children<'a>(item: &ShExPathItem<'a>, attribute: &AttributeOrAny) -> Vec<ShExPathItem<'a>> {
    match (item.value(), attribute) {
        (Value::Object(map), AttributeOrAny::Any) => map
            .iter()
            .map(|(key, value)| ShExPathItem::new(value, item.location().key(key)))
            .collect(),
        (Value::Array(members), AttributeOrAny::Any) => members
            .iter()
            .enumerate()
            .map(|(index, value)| ShExPathItem::new(value, item.location().index(index)))
            .collect(),
        (Value::Object(map), AttributeOrAny::Attribute(attribute)) => map
            .get(attribute.as_str())
            .map(|value| ShExPathItem::new(value, item.location().key(attribute.as_str())))
            .into_iter()
            .collect(),
        _ => Vec::new(),
    }
}

/// The effective boolean value of a value: false for an empty value; true for a value of more than
/// one item; and for a single item, the boolean itself, false for the number 0, false for the empty
/// string, and true otherwise.
fn effective_boolean_value(values: &[Cow<'_, Value>]) -> bool {
    match values {
        [] => false,
        [value] => match value.as_ref() {
            Value::Bool(b) => *b,
            Value::Number(n) => n.as_f64() != Some(0.0),
            Value::String(s) => !s.is_empty(),
            _ => true,
        },
        _ => true,
    }
}

/// Whether the first item a path selects (or a function's value) is `rvalue`. The spec does not
/// say what `<` and `>` compare, and the reference implementation selects nothing with them.
fn compare(value: Option<&Value>, comparison: &Comparison) -> bool {
    match comparison.comparator {
        Comparator::Equal => match (value, &comparison.rvalue) {
            (Some(Value::Number(n)), RValue::Integer(i)) => {
                n.as_i64() == Some(*i) || (n.as_i64().is_none() && n.as_f64() == Some(*i as f64))
            },
            (Some(Value::String(s)), RValue::Iri(IriRef::Iri(iri))) => s == iri.as_str(),
            _ => false,
        },
        Comparator::LessThan | Comparator::GreaterThan => false,
    }
}
