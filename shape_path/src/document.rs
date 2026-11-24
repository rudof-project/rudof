//! A schema written as ShExJ, which ShExPaths are evaluated over.

use crate::eval::Evaluator;
use crate::{Location, ShExPath, ShExPathError, ShExPathItem, ShExPathParser};
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use serde_json::Value;

/// A schema written as ShExJ, with the prefixes and base the schema declares.
#[derive(Debug, Clone)]
pub struct ShExJDocument {
    doc: Value,
    prefixmap: Option<PrefixMap>,
    base: Option<IriS>,
}

impl ShExJDocument {
    /// The document `doc`, a schema written as ShExJ, whose paths are read with `prefixmap` and
    /// `base`. `shex_ast::shexpath::shexj_document` makes one from a parsed schema.
    pub fn from_value(doc: Value, prefixmap: Option<PrefixMap>, base: Option<IriS>) -> Self {
        ShExJDocument { doc, prefixmap, base }
    }

    /// The ShExJ document.
    pub fn value(&self) -> &Value {
        &self.doc
    }

    /// The schema, as an item.
    pub fn root(&self) -> ShExPathItem<'_> {
        ShExPathItem::new(&self.doc, Location::root())
    }

    /// Parses `src` with the prefixes and base of the schema and evaluates it from the schema.
    pub fn select(&self, src: &str) -> Result<Vec<ShExPathItem<'_>>, ShExPathError> {
        let path = ShExPathParser::parse(src, self.base.as_ref(), self.prefixmap.as_ref())?;
        self.evaluate(&path)
    }

    /// Evaluates `path` from the schema: `evaluateShExPath(P, S, V)` with `V` the singleton
    /// holding `S`.
    pub fn evaluate(&self, path: &ShExPath) -> Result<Vec<ShExPathItem<'_>>, ShExPathError> {
        self.evaluate_from(path, vec![self.root()])
    }

    /// Evaluates `path` from the items of `context`: `evaluateShExPath(P, S, V)` with `V` the
    /// context. Every path is relative to the value it is given.
    pub fn evaluate_from<'a>(
        &'a self,
        path: &ShExPath,
        context: Vec<ShExPathItem<'a>>,
    ) -> Result<Vec<ShExPathItem<'a>>, ShExPathError> {
        Evaluator::new(&self.doc).eval_path(path, context)
    }
}
