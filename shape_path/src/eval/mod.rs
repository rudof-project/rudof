//! Evaluation of a ShExPath over a ShExJ document.

mod axes;
mod filters;

use crate::{Axis, Selector, ShExPath, ShExPathError, ShExPathItem, Step};
use axes::{children, descendants, this_shape_expr};
use serde_json::Value;
use std::cell::OnceCell;
use std::collections::{HashMap, HashSet};

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
            // `,`: every item each operand selected, in order, keeping duplicates.
            ShExPath::Sequence(paths) => self.concat(paths, &items),
            // `union`: every item either operand selected, with duplicates removed.
            ShExPath::Union(paths) => Ok(dedupe(self.concat(paths, &items)?)),
            // `intersection`: the items every operand selected, in the first one's order.
            ShExPath::Intersection(paths) => {
                let Some((first, rest)) = paths.split_first() else {
                    return Ok(Vec::new());
                };
                let mut kept = dedupe(self.eval_path(first, items.clone())?);
                for path in rest {
                    let also = self.eval_path(path, items.clone())?;
                    kept.retain(|item| also.iter().any(|other| other.value() == item.value()));
                }
                Ok(kept)
            },
        }
    }

    /// What each of `paths` selects from `items`, in order.
    fn concat(&self, paths: &[ShExPath], items: &[ShExPathItem<'a>]) -> Result<Vec<ShExPathItem<'a>>, ShExPathError> {
        let mut selected = Vec::new();
        for path in paths {
            selected.extend(self.eval_path(path, items.to_vec())?);
        }
        Ok(selected)
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
                // The grammar admits a parenthesized path, but neither the spec nor the reference
                // implementation says how to evaluate one.
                Selector::Path(_) => {
                    return Err(ShExPathError::NotImplemented {
                        feature: "a parenthesized step".to_owned(),
                    });
                },
            }
        }
        step.filters
            .iter()
            .try_fold(selected, |items, filter| self.apply_filter(filter, items))
    }
}

/// The items, the first occurrence of each kept. `union` and `intersection` compare items by
/// value, where the rest of the language compares them by identity (their location).
fn dedupe(items: Vec<ShExPathItem<'_>>) -> Vec<ShExPathItem<'_>> {
    let mut kept: Vec<ShExPathItem<'_>> = Vec::new();
    for item in items {
        if !kept.iter().any(|seen| seen.value() == item.value()) {
            kept.push(item);
        }
    }
    kept
}
