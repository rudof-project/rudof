//! Evaluation of a ShExPath over a ShExJ document.

use crate::shexpath::{
    Attribute, AttributeOrAny, Axis, Comparator, Comparison, Filter, Location, Operand, RValue, Segment, Selector,
    ShExPath, ShExPathError, ShExPathItem, Step,
};
use prefixmap::IriRef;
use serde_json::Value;
use std::borrow::Cow;
use std::cell::OnceCell;
use std::collections::{HashMap, HashSet};

/// The ShExJ types of triple expressions.
const TRIPLE_EXPR_TYPES: [&str; 3] = ["EachOf", "OneOf", "TripleConstraint"];

/// Evaluates paths over one document.
pub(crate) struct Evaluator<'a> {
    root: &'a Value,
    /// The triple expressions declared with a label, by label, built when first needed.
    triple_exprs: OnceCell<HashMap<&'a str, ShExPathItem<'a>>>,
}

impl<'a> Evaluator<'a> {
    pub(crate) fn new(root: &'a Value) -> Self {
        Evaluator {
            root,
            triple_exprs: OnceCell::new(),
        }
    }

    /// The value `path` produces from `items`.
    pub(crate) fn eval_path(
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
    fn eval_step(&self, step: &Step, items: Vec<ShExPathItem<'a>>) -> Result<Vec<ShExPathItem<'a>>, ShExPathError> {
        let mut selected = Vec::new();
        for item in items {
            match &step.selector {
                Selector::Child(attribute) => selected.extend(children(&item, attribute)),
                Selector::Axis(Axis::Itself) => selected.push(item),
                Selector::Axis(Axis::Parent) => selected.extend(self.parent(&item)),
                Selector::Axis(Axis::Ancestor) => selected.extend(self.ancestors(&item)),
                Selector::Axis(Axis::ThisShapeExpr) => selected.extend(this_shape_expr(&item)),
                Selector::Axis(Axis::ThisTripleExpr) => {
                    selected.extend(self.this_triple_expr(&item, &mut HashSet::new()))
                },
                Selector::Axis(Axis::Descendant) => selected.extend(descendants(&item)),
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

    /// `parent::`: the item that has this one as a property value or array member. The schema has
    /// no parent.
    fn parent(&self, item: &ShExPathItem<'a>) -> Option<ShExPathItem<'a>> {
        let location = item.location().parent()?;
        let value = location
            .segments()
            .iter()
            .try_fold(self.root, |value, segment| match segment {
                Segment::Key(key) => value.get(key),
                Segment::Index(index) => value.get(index),
            })?;
        Some(ShExPathItem::new(value, location))
    }

    /// `ancestor::`: the parent, its parent, and so on up to the schema, nearest first.
    fn ancestors(&self, item: &ShExPathItem<'a>) -> Vec<ShExPathItem<'a>> {
        std::iter::successors(self.parent(item), |parent| self.parent(parent)).collect()
    }

    /// `thisTripleExpr::`: the item and, if it is an `EachOf` or `OneOf`, the triple expressions it
    /// groups, transitively. A label among them is an inclusion, and the walk goes on into the
    /// expression it names; a label nothing declares selects nothing. An expression the walk has
    /// already reported (`seen`) is not reported again.
    fn this_triple_expr(&self, item: &ShExPathItem<'a>, seen: &mut HashSet<Location>) -> Vec<ShExPathItem<'a>> {
        match item.value() {
            Value::Array(_) => children(item, &AttributeOrAny::Any)
                .iter()
                .flat_map(|member| self.this_triple_expr(member, seen))
                .collect(),
            Value::String(label) => match self.triple_expr(label) {
                Some(expr) => self.this_triple_expr(&expr, seen),
                None => Vec::new(),
            },
            Value::Object(map) => {
                if !seen.insert(item.location().clone()) {
                    return Vec::new();
                }
                match map.get("type").and_then(Value::as_str) {
                    Some("EachOf" | "OneOf") => {
                        let mut walk = vec![item.clone()];
                        for expressions in children(item, &AttributeOrAny::Attribute(Attribute::Expressions)) {
                            walk.extend(self.this_triple_expr(&expressions, seen));
                        }
                        walk
                    },
                    Some("TripleConstraint") => vec![item.clone()],
                    _ => Vec::new(),
                }
            },
            _ => Vec::new(),
        }
    }

    /// The triple expression declared with `label`, if the schema has one.
    ///
    /// ShExJ has no top-level list of triple expressions the way it has `shapes`, so this looks
    /// through the whole document, once.
    fn triple_expr(&self, label: &str) -> Option<ShExPathItem<'a>> {
        self.triple_exprs
            .get_or_init(|| {
                let mut index = HashMap::new();
                for item in descendants(&ShExPathItem::new(self.root, Location::root())) {
                    if let Some(id) = item.value().get("id").and_then(Value::as_str)
                        && item
                            .value()
                            .get("type")
                            .and_then(Value::as_str)
                            .is_some_and(|ty| TRIPLE_EXPR_TYPES.contains(&ty))
                    {
                        index.entry(id).or_insert(item);
                    }
                }
                index
            })
            .get(label)
            .cloned()
    }

    /// The items of `items` that `filter` keeps. It is evaluated once per item, and knows the item,
    /// the value it is trimming and the item's position in it.
    fn apply_filter(
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
        item: &ShExPathItem<'a>,
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
    fn operand_values(
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

/// `thisShapeExpr::`: the item and, if it is a `ShapeAnd`, `ShapeOr` or `ShapeNot`, the shape
/// expressions it combines, transitively. It stops at a shape reference, which is an IRI rather
/// than an expression, and yields nothing for a `ShapeExternal` or a `ShapeDecl`.
fn this_shape_expr<'a>(item: &ShExPathItem<'a>) -> Vec<ShExPathItem<'a>> {
    match item.value() {
        Value::Array(_) => children(item, &AttributeOrAny::Any)
            .iter()
            .flat_map(this_shape_expr)
            .collect(),
        Value::Object(map) => {
            let combined = match map.get("type").and_then(Value::as_str) {
                Some("ShapeAnd" | "ShapeOr") => Attribute::ShapeExprs,
                Some("ShapeNot") => Attribute::ShapeExpr,
                Some("Shape" | "NodeConstraint") => return vec![item.clone()],
                _ => return Vec::new(),
            };
            std::iter::once(item.clone())
                .chain(
                    children(item, &AttributeOrAny::Attribute(combined))
                        .iter()
                        .flat_map(this_shape_expr),
                )
                .collect()
        },
        _ => Vec::new(),
    }
}

/// `descendant::`: every item below `item`, at any depth, in document order, not including `item`.
///
/// As in the reference implementation, the atomic values are left out: only objects and arrays are
/// collected. The `$` shortcut, the only use of this axis, looks for an `id`, which only an object
/// has.
fn descendants<'a>(item: &ShExPathItem<'a>) -> Vec<ShExPathItem<'a>> {
    let mut found = Vec::new();
    for child in children(item, &AttributeOrAny::Any) {
        if child.value().is_object() || child.value().is_array() {
            let below = descendants(&child);
            found.push(child);
            found.extend(below);
        }
    }
    found
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
