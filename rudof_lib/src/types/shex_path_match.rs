use serde::Serialize;
use serde_json::Value;

/// An element of a ShEx schema that a [ShExPath](https://shexspec.github.io/spec/ShExPath) selects.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShExPathMatch {
    /// Where the element is in the ShExJ form of the schema, as a JSON Pointer; the schema's is
    /// the empty string.
    pub location: String,
    /// The element, as ShExJ: an object, an array or an atomic value.
    pub value: Value,
}
