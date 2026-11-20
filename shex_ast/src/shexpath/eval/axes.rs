//! The axes of a step: which items are reachable from an item.

use super::Evaluator;
use crate::shexpath::{Attribute, AttributeOrAny, Location, Segment, ShExPathItem};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// The ShExJ types of triple expressions.
const TRIPLE_EXPR_TYPES: [&str; 3] = ["EachOf", "OneOf", "TripleConstraint"];

impl<'a> Evaluator<'a> {
    /// `parent::`: the item that has this one as a property value or array member. The schema has
    /// no parent.
    pub(super) fn parent(&self, item: &ShExPathItem<'a>) -> Option<ShExPathItem<'a>> {
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
    pub(super) fn ancestors(&self, item: &ShExPathItem<'a>) -> Vec<ShExPathItem<'a>> {
        std::iter::successors(self.parent(item), |parent| self.parent(parent)).collect()
    }

    /// `thisTripleExpr::`: the item and, if it is an `EachOf` or `OneOf`, the triple expressions it
    /// groups, transitively. A label among them is an inclusion, and the walk goes on into the
    /// expression it names; a label nothing declares selects nothing. An expression the walk has
    /// already reported (`seen`) is not reported again.
    pub(super) fn this_triple_expr(
        &self,
        item: &ShExPathItem<'a>,
        seen: &mut HashSet<Location>,
    ) -> Vec<ShExPathItem<'a>> {
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
}

/// `child::attribute`: the value of that property, if the item has it. `child::*`: every property
/// value of an object, or every member of an array.
pub(super) fn children<'a>(item: &ShExPathItem<'a>, attribute: &AttributeOrAny) -> Vec<ShExPathItem<'a>> {
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
pub(super) fn this_shape_expr<'a>(item: &ShExPathItem<'a>) -> Vec<ShExPathItem<'a>> {
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
pub(super) fn descendants<'a>(item: &ShExPathItem<'a>) -> Vec<ShExPathItem<'a>> {
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
