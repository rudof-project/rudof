//! ShExMap binding functions: `regex(...)`, `hashmap(...)` and `id(...)`.
//!
//! A `%Map:{ ... %}` action is either a variable name (`bp:given` or `<http://...>`),
//! which binds the matched value, or a function call.  Each function works in two
//! directions, as in shex.js's extension-map:
//!
//! * **lift** runs while validating the input: it turns the matched value into bindings.
//! * **lower** runs while materializing the output: it turns bindings back into a value.
//!
//! `regex(/(?<bp:family>[a-zA-Z]+), (?<bp:given>[a-zA-Z]+)/)`
//!   lift: match the value and bind each named group to the variable it names.
//!   lower: replace each named group with its variable's value, e.g. "Walker, Alice".
//!
//! `hashmap(bp:status, {"D": "Divorced", "M": "Married"})`
//!   lift: bind the variable to the map's value for the matched key.
//!   lower: find the key whose value the variable holds.
//!
//! `id(bp:mrn)`, `id(@node)`, `id(<http://a.example/person/{bp:mrn}>)`
//!   names the node a shape-valued constraint builds (see the materializer); it binds
//!   nothing when validating input, so one schema serves both directions.

use std::collections::{BTreeMap, HashMap};

use lazy_regex::{Lazy, Regex, lazy_regex};
use rudof_rdf::rdf_core::term::{Object, literal::ConcreteLiteral};

use crate::shexmap::error::ShExMapError;

/// Prefix declarations for ShExMap variable names: alias to namespace IRI.
pub type Prefixes = HashMap<String, String>;

/// In `id()`: the input node the enclosing iteration matched.
pub const NODE_ARGUMENT: &str = "@node";

static VARIABLE: Lazy<Regex> = lazy_regex!(r"^ *(?:<([^>]*)>|([^:<>\s]*):(\S*)) *$");
static FUNCTION_CALL: Lazy<Regex> = lazy_regex!(r"(?s)^\s*([A-Za-z][A-Za-z0-9]*)\s*\((.*)\)\s*$");
static CAPTURE_NAME: Lazy<Regex> = lazy_regex!(r"\?<((?:[A-Za-z0-9_.\-]*:[^>\s]*)|(?:<[^>]+>))>");
static CAPTURE_GROUP: Lazy<Regex> = lazy_regex!(r"\(\?<((?:[A-Za-z0-9_.\-]*:[^>\s]*)|(?:<[^>]+>))>[^)]+\)");
static REGEX_BODY: Lazy<Regex> = lazy_regex!(r"(?s)^\s*/(.*)/\s*$");
static HASHMAP_ARGS: Lazy<Regex> = lazy_regex!(r"(?s)^\s*([\w:<>/#.\-]+)\s*,\s*(\{.*\})\s*$");
static PLACEHOLDER: Lazy<Regex> = lazy_regex!(r"\{([^{}]*)\}");
static DOUBLE_SPACES: Lazy<Regex> = lazy_regex!(r"  +");
static ESCAPED_META: Lazy<Regex> = lazy_regex!(r"\\([/^$])");

fn function_error(msg: String) -> ShExMapError {
    ShExMapError::Function { msg }
}

/// Expand a ShExMap variable name -- `prefix:local` or `<iri>` -- to an IRI string.
pub fn expand_variable(name: &str, prefixes: &Prefixes) -> Result<String, ShExMapError> {
    let caps = VARIABLE
        .captures(name)
        .ok_or_else(|| function_error(format!("\"{name}\" is not a ShExMap variable (prefix:name or <iri>)")))?;
    if let Some(iri) = caps.get(1) {
        return Ok(iri.as_str().to_string());
    }
    let prefix = caps.get(2).map_or("", |m| m.as_str());
    let local = caps.get(3).map_or("", |m| m.as_str());
    match prefixes.get(prefix) {
        Some(ns) => Ok(format!("{ns}{local}")),
        None => Err(function_error(format!(
            "Unknown prefix \"{prefix}:\" in ShExMap variable \"{name}\""
        ))),
    }
}

/// Whether a Map code is a function call rather than a variable name.
pub fn is_function_call(code: &str) -> bool {
    FUNCTION_CALL.is_match(code)
}

/// Whether a Map code is an `id(...)` call.
pub fn is_key_code(code: &str) -> bool {
    FUNCTION_CALL.captures(code).is_some_and(|c| &c[1] == "id")
}

fn split_call(code: &str) -> Result<(String, String), ShExMapError> {
    let caps = FUNCTION_CALL
        .captures(code)
        .ok_or_else(|| function_error(format!("Invalid ShExMap function call: {}", code.trim())))?;
    Ok((caps[1].to_string(), caps[2].to_string()))
}

/// The lexical form of a bound value, as the functions see it.
pub fn lexical(value: &Object) -> String {
    match value {
        Object::Iri(iri) => iri.as_str().to_string(),
        Object::BlankNode(b) => b.clone(),
        Object::Literal(lit) => lit.lexical_form(),
        Object::Triple { .. } => value.to_string(),
    }
}

fn plain_literal(s: &str) -> Object {
    Object::literal(ConcreteLiteral::str(s))
}

fn unescape_meta(s: &str) -> String {
    ESCAPED_META.replace_all(s, "$1").into_owned()
}

fn collapse_spaces(s: &str) -> String {
    DOUBLE_SPACES.replace_all(s, " ").into_owned()
}

// -- regex ------------------------------------------------------------------------------

fn regex_body(code: &str, args: &str) -> Result<String, ShExMapError> {
    if let Some(caps) = REGEX_BODY.captures(args) {
        let body = &caps[1];
        if body.is_empty() {
            return Err(function_error(format!(
                "{} is missing the required regex pattern",
                code.trim()
            )));
        }
        return Ok(body.to_string());
    }
    Ok(args.to_string())
}

/// The named groups of a regex code, in order, with the names expanded; and the pattern
/// with the names removed (Rust regex names cannot hold `:`).
fn regex_parts(code: &str, args: &str, prefixes: &Prefixes) -> Result<(Vec<String>, String), ShExMapError> {
    let pattern = regex_body(code, args)?;
    let mut names = Vec::new();
    let mut plain = String::new();
    let mut pos = 0;
    for m in CAPTURE_NAME.find_iter(&pattern) {
        plain.push_str(&pattern[pos..m.start()]);
        let caps = CAPTURE_NAME.captures(m.as_str()).expect("a match of the same regex");
        names.push(expand_variable(&unescape_meta(&caps[1]), prefixes)?);
        pos = m.end();
    }
    plain.push_str(&pattern[pos..]);
    if names.is_empty() {
        return Err(function_error(format!("Found no capture variable in {}", code.trim())));
    }
    Ok((names, plain))
}

fn regex_lift(
    code: &str,
    args: &str,
    value: &Object,
    prefixes: &Prefixes,
) -> Result<BTreeMap<String, Object>, ShExMapError> {
    let (names, plain) = regex_parts(code, args, prefixes)?;
    let text = lexical(value);
    let re = Regex::new(&plain)
        .map_err(|e| function_error(format!("Error matching {} against {text:?}: {e}", code.trim())))?;
    let caps = re
        .captures(&text)
        .ok_or_else(|| function_error(format!("{} found no match for input \"{text}\"", code.trim())))?;
    Ok(names
        .into_iter()
        .enumerate()
        .map(|(i, name)| (name, plain_literal(caps.get(i + 1).map_or("", |m| m.as_str()))))
        .collect())
}

fn regex_lower(
    code: &str,
    args: &str,
    get: &mut dyn FnMut(&str) -> Option<Object>,
    prefixes: &Prefixes,
) -> Result<Option<Object>, ShExMapError> {
    let pattern = regex_body(code, args)?;
    let mut matched = false;
    let mut missing = false;
    let mut text = String::new();
    let mut pos = 0;
    for m in CAPTURE_GROUP.find_iter(&pattern) {
        matched = true;
        text.push_str(&pattern[pos..m.start()]);
        let caps = CAPTURE_GROUP.captures(m.as_str()).expect("a match of the same regex");
        let name = expand_variable(&unescape_meta(&caps[1]), prefixes)?;
        match get(&name) {
            Some(v) => text.push_str(&lexical(&v)),
            None => missing = true,
        }
        pos = m.end();
    }
    text.push_str(&pattern[pos..]);
    if !matched {
        return Err(function_error(format!("Found no capture variable in {}", code.trim())));
    }
    if missing {
        return Ok(None);
    }
    Ok(Some(plain_literal(&unescape_meta(&collapse_spaces(&text)))))
}

// -- hashmap ----------------------------------------------------------------------------

fn hashmap_args(code: &str, args: &str, prefixes: &Prefixes) -> Result<(String, Vec<(String, String)>), ShExMapError> {
    let caps = HASHMAP_ARGS.captures(args).ok_or_else(|| {
        function_error(format!(
            "hashmap needs a variable name and a JSON map, found: {}",
            code.trim()
        ))
    })?;
    let mapping: serde_json::Value = serde_json::from_str(&caps[2])
        .map_err(|e| function_error(format!("hashmap could not parse the map in {}: {e}", code.trim())))?;
    let obj = match mapping {
        serde_json::Value::Object(m) if !m.is_empty() => m,
        _ => {
            return Err(function_error(format!(
                "hashmap needs a non-empty JSON object in {}",
                code.trim()
            )));
        },
    };
    let mut pairs = Vec::new();
    for (k, v) in obj {
        let v = match v {
            serde_json::Value::String(s) => s,
            other => other.to_string(),
        };
        pairs.push((k, v));
    }
    let mut values: Vec<&String> = pairs.iter().map(|(_, v)| v).collect();
    values.sort();
    values.dedup();
    if values.len() != pairs.len() {
        return Err(function_error(format!(
            "hashmap values must be unique so the map can be inverted: {}",
            code.trim()
        )));
    }
    Ok((expand_variable(&caps[1], prefixes)?, pairs))
}

fn hashmap_lift(
    code: &str,
    args: &str,
    value: &Object,
    prefixes: &Prefixes,
) -> Result<BTreeMap<String, Object>, ShExMapError> {
    let (name, mapping) = hashmap_args(code, args, prefixes)?;
    let key = lexical(value);
    let mapped = mapping
        .iter()
        .find(|(k, _)| *k == key)
        .ok_or_else(|| function_error(format!("{} has no entry for \"{key}\"", code.trim())))?;
    Ok(BTreeMap::from([(name, plain_literal(&mapped.1))]))
}

fn hashmap_lower(
    code: &str,
    args: &str,
    get: &mut dyn FnMut(&str) -> Option<Object>,
    prefixes: &Prefixes,
) -> Result<Option<Object>, ShExMapError> {
    let (name, mapping) = hashmap_args(code, args, prefixes)?;
    let Some(val) = get(&name) else {
        return Ok(None);
    };
    let text = lexical(&val);
    match mapping.iter().find(|(_, v)| *v == text) {
        Some((k, _)) => Ok(Some(plain_literal(k))),
        None => Err(function_error(format!("{} cannot invert \"{text}\"", code.trim()))),
    }
}

// -- id ---------------------------------------------------------------------------------

/// An argument of `id()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyTerm {
    /// A variable, by IRI.
    Variable(String),
    /// [`NODE_ARGUMENT`]: the node the enclosing input iteration matched.
    Node,
    /// An IRI template with `{var}` placeholders.
    Template(Template),
}

/// An IRI template argument of `id()`: `<http://a.example/person/{v:mrn}>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    segments: Vec<Segment>,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Segment {
    Text(String),
    Var(String),
}

impl Template {
    pub fn new(text: &str, prefixes: &Prefixes) -> Result<Template, ShExMapError> {
        let mut segments = Vec::new();
        let mut pos = 0;
        for m in PLACEHOLDER.find_iter(text) {
            if m.start() > pos {
                segments.push(Segment::Text(text[pos..m.start()].to_string()));
            }
            let caps = PLACEHOLDER.captures(m.as_str()).expect("a match of the same regex");
            segments.push(Segment::Var(expand_variable(&caps[1], prefixes)?));
            pos = m.end();
        }
        if pos < text.len() {
            segments.push(Segment::Text(text[pos..].to_string()));
        }
        Ok(Template {
            segments,
            text: text.to_string(),
        })
    }

    /// The variables the template reads.
    pub fn variables(&self) -> Vec<String> {
        self.segments
            .iter()
            .filter_map(|s| match s {
                Segment::Var(v) => Some(v.clone()),
                Segment::Text(_) => None,
            })
            .collect()
    }

    /// The IRI, values percent-encoded as R2RML does; `None` when a variable is unbound.
    pub fn expand(&self, get: &mut dyn FnMut(&str) -> Option<Object>) -> Option<Object> {
        let mut out = String::new();
        for seg in &self.segments {
            match seg {
                Segment::Text(t) => out.push_str(t),
                Segment::Var(v) => out.push_str(&percent_encode(&lexical(&get(v)?))),
            }
        }
        Some(Object::iri(rudof_iri::IriS::new_unchecked(&out)))
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Percent-encode everything but unreserved characters (letters, digits, `-._~`), as
/// R2RML's IRI-safe templates do.
pub fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// The arguments of an `id(...)` code.
pub fn key_terms(code: &str, prefixes: &Prefixes) -> Result<Vec<KeyTerm>, ShExMapError> {
    let (name, args) = split_call(code)?;
    if name != "id" {
        return Err(function_error(format!("{} is not an id() code", code.trim())));
    }
    let parts: Vec<&str> = if args.trim().is_empty() {
        Vec::new()
    } else {
        args.split(',').map(str::trim).collect()
    };
    if parts.is_empty() {
        return Err(function_error(format!(
            "id() needs at least one variable, template or {NODE_ARGUMENT}: {}",
            code.trim()
        )));
    }
    let mut terms = Vec::new();
    for a in parts {
        if a == NODE_ARGUMENT {
            terms.push(KeyTerm::Node);
        } else if a.starts_with('<') && a.ends_with('>') && a.contains('{') {
            terms.push(KeyTerm::Template(Template::new(&a[1..a.len() - 1], prefixes)?));
        } else {
            terms.push(KeyTerm::Variable(expand_variable(a, prefixes)?));
        }
    }
    Ok(terms)
}

/// The variables an `id(...)` code reads (templates' placeholders included), and
/// [`NODE_ARGUMENT`] where it appears.
pub fn key_arguments(code: &str, prefixes: &Prefixes) -> Result<Vec<String>, ShExMapError> {
    let mut out = Vec::new();
    for term in key_terms(code, prefixes)? {
        match term {
            KeyTerm::Variable(v) => out.push(v),
            KeyTerm::Node => out.push(NODE_ARGUMENT.to_string()),
            KeyTerm::Template(t) => out.extend(t.variables()),
        }
    }
    Ok(out)
}

// -- dispatch -----------------------------------------------------------------------------

/// Apply a function call to a matched value, returning the variable bindings it makes.
pub fn lift(code: &str, value: &Object, prefixes: &Prefixes) -> Result<BTreeMap<String, Object>, ShExMapError> {
    let (name, args) = split_call(code)?;
    match name.as_str() {
        "regex" => regex_lift(code, &args, value, prefixes),
        "hashmap" => hashmap_lift(code, &args, value, prefixes),
        "id" => {
            // id() names an output node; validating input it binds nothing
            key_arguments(code, prefixes)?;
            Ok(BTreeMap::new())
        },
        _ => Err(function_error(format!(
            "Unknown ShExMap function {name}() in {}",
            code.trim()
        ))),
    }
}

/// The variables a function call reads when lowering (and binds when lifting).
pub fn variables_of(code: &str, prefixes: &Prefixes) -> Result<Vec<String>, ShExMapError> {
    let (name, args) = split_call(code)?;
    match name.as_str() {
        "regex" => Ok(regex_parts(code, &args, prefixes)?.0),
        "hashmap" => Ok(vec![hashmap_args(code, &args, prefixes)?.0]),
        "id" => Ok(key_arguments(code, prefixes)?
            .into_iter()
            .filter(|a| a != NODE_ARGUMENT)
            .collect()),
        _ => Err(function_error(format!(
            "Unknown ShExMap function {name}() in {}",
            code.trim()
        ))),
    }
}

/// Build a value from bindings with a function call; `None` when a variable is unbound.
pub fn lower(
    code: &str,
    get: &mut dyn FnMut(&str) -> Option<Object>,
    prefixes: &Prefixes,
) -> Result<Option<Object>, ShExMapError> {
    let (name, args) = split_call(code)?;
    match name.as_str() {
        "regex" => regex_lower(code, &args, get, prefixes),
        "hashmap" => hashmap_lower(code, &args, get, prefixes),
        "id" => Err(function_error(format!(
            "{} names a node; it belongs on a shape-valued constraint, not a value",
            code.trim()
        ))),
        _ => Err(function_error(format!(
            "Unknown ShExMap function {name}() in {}",
            code.trim()
        ))),
    }
}

/// The variables a Map code binds or reads: the variable itself, or a function's.
pub fn code_variables(code: &str, prefixes: &Prefixes) -> Result<Vec<String>, ShExMapError> {
    if is_function_call(code) {
        variables_of(code, prefixes)
    } else {
        Ok(vec![expand_variable(code, prefixes)?])
    }
}

/// Prefixes that expand any alias a code mentions, for checking codes when the
/// declarations are not at hand (the validator's semantic-action hook): each alias maps to
/// a placeholder namespace.
pub fn any_prefixes(code: &str) -> Prefixes {
    static ALIAS: Lazy<Regex> = lazy_regex!(r"([A-Za-z0-9_.\-]*):");
    let mut out = Prefixes::new();
    for caps in ALIAS.captures_iter(code) {
        let alias = caps[1].to_string();
        out.entry(alias.clone())
            .or_insert_with(|| format!("{PLACEHOLDER_NAMESPACE}{alias}:"));
    }
    out
}

/// The namespace [`any_prefixes`] gives an alias: a variable expanded with it is not a real
/// IRI.
pub const PLACEHOLDER_NAMESPACE: &str = "urn:shexmap-prefix:";

#[cfg(test)]
mod tests {
    use super::*;

    fn prefixes() -> Prefixes {
        Prefixes::from([("bp".to_string(), "http://shex.io/extensions/Map/#BPDAM-".to_string())])
    }

    fn str_lit(s: &str) -> Object {
        plain_literal(s)
    }

    #[test]
    fn variables_expand() {
        assert_eq!(
            expand_variable(" bp:given ", &prefixes()).unwrap(),
            "http://shex.io/extensions/Map/#BPDAM-given"
        );
        assert_eq!(expand_variable("<http://x/y>", &prefixes()).unwrap(), "http://x/y");
        assert!(expand_variable("zz:a", &prefixes()).is_err());
        assert!(expand_variable("regex(/a/)", &prefixes()).is_err());
    }

    #[test]
    fn regex_lifts_and_lowers() {
        let code = "regex(/(?<bp:family>[a-zA-Z]+), (?<bp:given>[a-zA-Z]+)/)";
        assert!(is_function_call(code));
        let bound = lift(code, &str_lit("Walker, Alice"), &prefixes()).unwrap();
        assert_eq!(
            bound.get("http://shex.io/extensions/Map/#BPDAM-family"),
            Some(&str_lit("Walker"))
        );
        assert_eq!(
            bound.get("http://shex.io/extensions/Map/#BPDAM-given"),
            Some(&str_lit("Alice"))
        );
        let mut get = |v: &str| bound.get(v).cloned();
        assert_eq!(
            lower(code, &mut get, &prefixes()).unwrap(),
            Some(str_lit("Walker, Alice"))
        );
        assert!(lift(code, &str_lit("nope"), &prefixes()).is_err());
        assert_eq!(
            variables_of(code, &prefixes()).unwrap(),
            vec![
                "http://shex.io/extensions/Map/#BPDAM-family".to_string(),
                "http://shex.io/extensions/Map/#BPDAM-given".to_string()
            ]
        );
    }

    #[test]
    fn hashmap_lifts_and_lowers() {
        let code = r#"hashmap(bp:status, {"D": "Divorced", "M": "Married"})"#;
        let bound = lift(code, &str_lit("M"), &prefixes()).unwrap();
        let var = "http://shex.io/extensions/Map/#BPDAM-status";
        assert_eq!(bound.get(var), Some(&str_lit("Married")));
        let mut get = |v: &str| bound.get(v).cloned();
        assert_eq!(lower(code, &mut get, &prefixes()).unwrap(), Some(str_lit("M")));
        assert!(lift(code, &str_lit("X"), &prefixes()).is_err());
        let dup = r#"hashmap(bp:status, {"D": "Same", "M": "Same"})"#;
        assert!(lift(dup, &str_lit("D"), &prefixes()).is_err());
    }

    #[test]
    fn id_terms() {
        assert!(is_key_code(" id(bp:a, @node) "));
        assert!(!is_key_code("bp:a"));
        let terms = key_terms("id(bp:a, @node, <http://a.example/p/{bp:b}/x>)", &prefixes()).unwrap();
        assert_eq!(terms.len(), 3);
        assert_eq!(terms[1], KeyTerm::Node);
        assert_eq!(
            key_arguments("id(bp:a, @node, <http://a.example/p/{bp:b}/x>)", &prefixes()).unwrap(),
            vec![
                "http://shex.io/extensions/Map/#BPDAM-a".to_string(),
                "@node".to_string(),
                "http://shex.io/extensions/Map/#BPDAM-b".to_string()
            ]
        );
        assert!(key_terms("id()", &prefixes()).is_err());
        assert!(lift("id(bp:a)", &str_lit("x"), &prefixes()).unwrap().is_empty());
    }

    #[test]
    fn templates_percent_encode() {
        let t = Template::new("http://a.example/person/{bp:mrn}", &prefixes()).unwrap();
        let mut get = |_: &str| Some(str_lit("a b/c"));
        assert_eq!(
            t.expand(&mut get),
            Some(Object::iri(rudof_iri::IriS::new_unchecked(
                "http://a.example/person/a%20b%2Fc"
            )))
        );
        let mut none = |_: &str| None;
        assert_eq!(t.expand(&mut none), None);
        assert_eq!(percent_encode("A-z.9_~"), "A-z.9_~");
    }
}
