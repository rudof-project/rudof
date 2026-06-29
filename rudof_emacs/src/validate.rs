//! Stateful ShEx conformance-checking, exposed to Emacs Lisp as a held,
//! mutable `Rudof` handle (a `user-ptr`) plus five functions operating on
//! it -- mirroring `python/src/pyrudof_lib.rs`'s own `read_data`/
//! `read_shex`/`read_shapemap`/`validate_shex` shape: parsing happens
//! entirely inside Rust, and nothing about the loaded schema/data/
//! shapemap is ever represented in Lisp -- only the opaque handle is,
//! plus the small, final, flattened (node, shape, status) triples
//! `validate_shex` produces.
//!
//! Designed for an editor workflow with three buffers (schema, data, and
//! a ShapeMap) where the schema and ShapeMap are loaded once and the data
//! buffer is re-validated on every edit: re-running `read-data` +
//! `validate-shex` on the *same* handle re-parses only the data (cheap),
//! never the schema -- see `read_data`'s own doc on why it always
//! *replaces* rather than accumulates.
//!
//! **Load order matters**: `rudof_lib::Rudof::load_shapemap` itself
//! requires data (and a schema) to already be loaded, since a compact-
//! syntax ShapeMap's node/shape selectors are resolved against the
//! data's/schema's own currently-declared prefixes -- so `read-shapemap`
//! must come *after* both `read-shex` and `read-data`, not before (the
//! order shown below). Once loaded, though, the ShapeMap is independent
//! of the data store going forward -- re-reading data afterward does not
//! require re-reading the ShapeMap too, which is exactly what makes the
//! "load schema + ShapeMap once, re-read data on every edit" workflow
//! cheap.
//!
//! Build with `cargo build --release -p rudof_emacs`, then from Emacs:
//!
//! ```emacs-lisp
//! (module-load "/path/to/target/release/librudof_emacs.dylib") ; .so/.dll elsewhere
//! (require 'rudof-emacs)
//! (let ((rudof (rudof-emacs-new)))
//!   (rudof-emacs-read-shex
//!    rudof
//!    "PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
//!     <http://example.org/PersonShape> { <http://example.org/age> xsd:integer }"
//!    "shexc" nil)
//!   (rudof-emacs-read-data
//!    rudof "<http://example.org/alice> <http://example.org/age> 30 ." "turtle" nil)
//!   (rudof-emacs-read-shapemap
//!    rudof "<http://example.org/alice>@<http://example.org/PersonShape>" nil nil nil)
//!   (rudof-emacs-validate-shex rudof))
//! ;; => (("http://example.org/alice" "http://example.org/PersonShape" "conformant"))
//! ```

use emacs::{Env, Result, Value, defun};
use rudof_lib::formats::{DataFormat, InputSpec, ShExFormat, ShapeMapFormat};
use rudof_lib::{Rudof, RudofConfig};
use std::str::FromStr;

// `mod_in_name = false`: this file's own `mod validate` nesting (an
// internal Rust organization choice) must not leak into the Lisp names
// below -- without it, `read_data` would register as
// `rudof-emacs-validate-read-data`.
#[emacs::module(mod_in_name = false)]
fn init(_: &Env) -> Result<()> {
    Ok(())
}

emacs::plugin_is_GPL_compatible!();

/// Create a new, empty Rudof instance -- an opaque handle threaded through
/// every other function below as their first argument. Nothing about the
/// loaded schema/data/shapemap is ever represented in Lisp; only this
/// handle is (a `user-ptr`, printed as `#<user-ptr ...>`).
#[defun(user_ptr)]
fn new() -> Result<Rudof> {
    Ok(Rudof::new(RudofConfig::default()))
}

/// Load INPUT (RDF text) into RUDOF, *replacing* any data previously
/// loaded into it. This deviates from `rudof_lib::Rudof::load_data`'s own
/// default (which merges new data into whatever was already loaded) --
/// deliberately, since this function exists for an editor buffer holding
/// "the data": re-reading that buffer's *current* text after an edit
/// should always mean the data is now exactly that text, never the
/// previous text with the new text appended to it.
///
/// FORMAT names an RDF format as accepted by the `rudof` CLI (e.g.
/// "turtle", "ntriples", "jsonld"; nil means "turtle"). BASE is an
/// optional base IRI for resolving relative IRIs in INPUT.
#[defun]
pub fn read_data(rudof: &mut Rudof, input: String, format: Option<String>, base: Option<String>) -> Result<()> {
    let format = match format {
        Some(format) => DataFormat::from_str(&format)?,
        None => DataFormat::default(),
    };
    let data = [InputSpec::str(&input)];
    let mut loading = rudof
        .load_data()
        .with_data(&data)
        .with_data_format(&format)
        .with_merge(false);
    if let Some(base) = base.as_deref() {
        loading = loading.with_base(base);
    }
    loading.execute()?;
    Ok(())
}

/// Load INPUT (ShExC/ShExJ/... text) into RUDOF as its ShEx schema,
/// replacing any schema previously loaded (schemas are always compiled as
/// a whole unit -- there is no partial/merge concept to deviate from
/// here, unlike `read_data`). FORMAT names a ShEx format as accepted by
/// the `rudof` CLI (e.g. "shexc", "shexj"; nil means "shexc"). BASE as in
/// `read_data`.
#[defun]
pub fn read_shex(rudof: &mut Rudof, input: String, format: Option<String>, base: Option<String>) -> Result<()> {
    let format = match format {
        Some(format) => ShExFormat::from_str(&format)?,
        None => ShExFormat::default(),
    };
    let input = InputSpec::str(&input);
    let mut loading = rudof.load_shex_schema(&input).with_shex_schema_format(&format);
    if let Some(base) = base.as_deref() {
        loading = loading.with_base(base);
    }
    loading.execute()?;
    Ok(())
}

/// Load INPUT (ShapeMap text, e.g. compact syntax
/// `"<node1>@<Shape1>,<node2>@<Shape2>"`) into RUDOF, replacing any
/// ShapeMap previously loaded. FORMAT names a ShapeMap format as accepted
/// by the `rudof` CLI (nil means "compact"). BASE-NODES/BASE-SHAPES are
/// optional base IRIs for resolving relative node/shape IRIs in INPUT.
///
/// Must be called *after* both `read-shex` and `read-data` -- see this
/// module's own top Commentary on why.
#[defun]
pub fn read_shapemap(
    rudof: &mut Rudof,
    input: String,
    format: Option<String>,
    base_nodes: Option<String>,
    base_shapes: Option<String>,
) -> Result<()> {
    let format = match format {
        Some(format) => ShapeMapFormat::from_str(&format)?,
        None => ShapeMapFormat::default(),
    };
    let input = InputSpec::str(&input);
    let mut loading = rudof.load_shapemap(&input).with_shapemap_format(&format);
    if let Some(base_nodes) = base_nodes.as_deref() {
        loading = loading.with_base_nodes(base_nodes);
    }
    if let Some(base_shapes) = base_shapes.as_deref() {
        loading = loading.with_base_shapes(base_shapes);
    }
    loading.execute()?;
    Ok(())
}

/// Validate RUDOF's currently loaded data against its currently loaded
/// schema, per its currently loaded ShapeMap, returning the result as a
/// flat list of `(NODE SHAPE STATUS REASON)` string quadruples -- one per
/// node/shape association in the ShapeMap -- for e.g. a flymake backend
/// to walk directly, without needing to parse any serialized text. STATUS
/// is one of "conformant"/"nonconformant"/"pending"/"inconsistent"; REASON
/// is `ValidationStatus::reason()`'s human-readable explanation (e.g. why a
/// node failed to conform), suitable as a diagnostic message as-is.
///
/// Signals a Lisp error (with rudof's own message) if no data/schema/
/// ShapeMap is loaded, or if validation otherwise fails to run.
#[defun]
fn validate_shex<'e>(env: &'e Env, rudof: &mut Rudof) -> Result<Value<'e>> {
    let quadruples = validate_shex_quadruples(rudof)?;
    let mut rows = Vec::new();
    for (node, shape, status, reason) in quadruples {
        rows.push(env.list((node, shape, status, reason))?);
    }
    env.list(rows.as_slice())
}

/// The env-free core of [`validate_shex`], split out so it can be exercised
/// directly by `tests/shextest.rs` (an `Env` only exists inside a running
/// Emacs process, so the `#[defun]` above can't be called from `cargo test`).
pub fn validate_shex_quadruples(rudof: &mut Rudof) -> Result<Vec<(String, String, String, String)>> {
    rudof.validate_shex().execute()?;
    let results = rudof
        .shex_validation_results()
        .ok_or_else(|| anyhow::anyhow!("validate_shex succeeded but produced no results"))?;
    Ok(results
        .iter()
        .map(|(node, shape, status)| (node.to_string(), shape.to_string(), status.code(), status.reason()))
        .collect())
}
