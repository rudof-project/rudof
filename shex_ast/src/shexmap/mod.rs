//! ShExMap: map RDF from one ShEx schema to another.
//!
//! A port of the design shared by shex.js's `@shexjs/extension-map` and PyShEx's
//! `pyshex.shexmap`.  Semantic actions in the `http://shex.io/extensions/Map/#` extension
//! annotate triple constraints:
//!
//! * in the **input** schema, `%Map:{ bp:given %}` binds the variable `bp:given` to the
//!   value the triple constraint matched, and `%Map:{ regex(/(?<bp:family>...)/) %}` or
//!   `%Map:{ hashmap(var, {...}) %}` bind variables computed from it;
//! * in the **output** schema, the same annotations say where each bound value goes, and
//!   `%Map:{ id(var, ...) %}` on a shape-valued constraint names the node it builds.
//!
//! ```ignore
//! let bindings = bind(&graph, &input_schema, &focus, None, false)?;
//! let out: OxigraphInMemory = materialize(&output_schema, &bindings, Some(&root), None, Options::default())?;
//! ```
//!
//! or both at once with [`map_graph`].  The `rudof shexmap` command does the same from a
//! terminal.
//!
//! * [`bind`] checks that the focus conforms, then partitions each neighbourhood by an
//!   exhaustive search; when the input conforms in several ways that bind differently it
//!   says so (`.alternatives`), and [`bind_all`] returns every way.  EXTENDS and inverse
//!   (`^p`) constraints are followed.
//! * [`analyse`] checks a schema pair before any data: every output repetition has a list
//!   to iterate, every variable read is bound where it can be read from, and unused
//!   bindings are reported.
//! * [`Materializer`] builds the output from the binding tree's structure: an output
//!   repetition iterates the input list its variables are bound at, a parent's binding is
//!   read in every item, and the alternatives a `OneOf` or an extension leaves open are
//!   kept (`.accepts`), the one that reads the most bindings returned.  It can also update
//!   a graph in place, replacing what the output schema holds at the root.
//!
//! The binding tree's JSON is shex.js's and PyShEx's: `[own, list, ...]` scopes with
//! `@node` on the iterations of repeated shape-valued constraints (see [`bindings`]).
//!
//! The older, flat [`crate::materialize`] module and its `MapState` read what the Map
//! extension records during validation; this module reads the graph itself, after
//! validation, so that repeated and nested constraints keep their structure.

pub mod analysis;
pub mod bindings;
pub mod error;
pub mod functions;
pub mod materializer;
pub mod schema;
pub mod scopes;
pub mod term;

use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{BuildRDF, NeighsRDF};

use crate::ast::{Schema, ShapeExprLabel};

pub use analysis::{Report as AnalysisReport, analyse};
pub use bindings::{BindingTree, Bindings, Frame, MAX_ALTERNATIVES, NODE_KEY, bind, bind_all};
pub use error::{Failure, ShExMapError};
pub use functions::Prefixes;
pub use materializer::{
    Accept, Materializer, Options as MaterializerOptions, Report as MaterializerReport, Source, SourceKind, materialize,
};
pub use schema::MAP_EXTENSION;
pub use term::{MapTriple, parse_term};

/// Bind `focus` against `input_schema` and materialize `root` in `output_schema`.
pub fn map_graph<R: NeighsRDF, G: BuildRDF>(
    graph: &R,
    input_schema: &Schema,
    focus: &Object,
    output_schema: &Schema,
    root: Option<&Object>,
    start: Option<&ShapeExprLabel>,
    output_start: Option<&ShapeExprLabel>,
    options: MaterializerOptions,
    strict: bool,
) -> Result<G, ShExMapError> {
    let bindings = bind(graph, input_schema, focus, start, strict)?;
    materialize(output_schema, &bindings, root, output_start, options)
}
