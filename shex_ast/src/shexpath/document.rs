//! A schema written as ShExJ, which ShExPaths are evaluated over.

use crate::Schema;
use crate::shexpath::eval::Evaluator;
use crate::shexpath::{Location, ShExPath, ShExPathError, ShExPathItem, ShExPathParser, shexj};
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
    /// The ShExJ form of `schema`.
    pub fn new(schema: &Schema) -> Result<Self, ShExPathError> {
        Ok(ShExJDocument {
            doc: shexj(schema)?,
            prefixmap: schema.prefixmap(),
            base: schema.base(),
        })
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
