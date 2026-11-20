//! Evaluation of a ShExPath over a ShExJ document.

use crate::shexpath::{AttributeOrAny, Selector, ShExPath, ShExPathError, ShExPathItem, Step};
use serde_json::Value;

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

    /// Applies `step` to every item and concatenates what it reaches, then applies its filters.
    fn eval_step<'a>(&self, step: &Step, items: Vec<ShExPathItem<'a>>) -> Result<Vec<ShExPathItem<'a>>, ShExPathError> {
        if !step.filters.is_empty() {
            return Err(ShExPathError::NotImplemented {
                feature: "filters".to_string(),
            });
        }
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
        Ok(selected)
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
