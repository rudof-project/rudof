//! RDF terms as ShExMap sees them: JSON forms shared with shex.js and PyShEx, N3 forms for
//! keys and digests, and the triple type the module produces.

use prefixmap::IriRef;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::{
    Object,
    literal::{ConcreteLiteral, Lang},
};
use serde_json::{Map, Value, json};

use crate::shexmap::error::ShExMapError;

const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";

/// A triple of the graph being read or built: an IRI or blank node, a predicate, a value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MapTriple {
    pub subject: Object,
    pub predicate: IriS,
    pub object: Object,
}

impl MapTriple {
    pub fn new(subject: Object, predicate: IriS, object: Object) -> Self {
        MapTriple {
            subject,
            predicate,
            object,
        }
    }

    /// The triple in N-Triples syntax (blank nodes by label).
    pub fn n3(&self) -> String {
        format!(
            "{} <{}> {} .",
            n3(&self.subject),
            self.predicate.as_str(),
            n3(&self.object)
        )
    }
}

impl std::fmt::Display for MapTriple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.n3())
    }
}

/// Whether a term can be the subject of a triple.
pub fn is_resource(term: &Object) -> bool {
    matches!(term, Object::Iri(_) | Object::BlankNode(_))
}

/// A term in N3: `<iri>`, `_:label`, `"lexical"`, `"lexical"@lang` or `"lexical"^^<datatype>`.
pub fn n3(term: &Object) -> String {
    match term {
        Object::Iri(iri) => format!("<{}>", iri.as_str()),
        Object::BlankNode(b) => format!("_:{b}"),
        Object::Literal(lit) => {
            let lex = escape(&lit.lexical_form());
            if let Some(lang) = lit.lang() {
                format!("\"{lex}\"@{lang}")
            } else {
                match lit.datatype().get_iri() {
                    Ok(dt) if dt.as_str() != XSD_STRING => format!("\"{lex}\"^^<{}>", dt.as_str()),
                    _ => format!("\"{lex}\""),
                }
            }
        },
        Object::Triple { .. } => term.to_string(),
    }
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some(other) => out.push(other),
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// A term as shex.js's bindings JSON writes it: a literal as `{"value", "type"?, "language"?}`,
/// a blank node as `"_:label"`, an IRI as a string.
pub fn term_to_json(term: &Object) -> Value {
    match term {
        Object::Iri(iri) => Value::String(iri.as_str().to_string()),
        Object::BlankNode(b) => Value::String(format!("_:{b}")),
        Object::Literal(lit) => {
            let mut out = Map::new();
            out.insert("value".to_string(), Value::String(lit.lexical_form()));
            if let Some(lang) = lit.lang() {
                out.insert("language".to_string(), Value::String(lang.to_string()));
            } else if let Ok(dt) = lit.datatype().get_iri()
                && dt.as_str() != XSD_STRING
            {
                out.insert("type".to_string(), Value::String(dt.as_str().to_string()));
            }
            Value::Object(out)
        },
        Object::Triple { .. } => json!({"value": term.to_string()}),
    }
}

/// The inverse of [`term_to_json`]; also reads a quoted N3 literal string.
pub fn term_from_json(value: &Value) -> Result<Object, ShExMapError> {
    match value {
        Value::Object(m) => {
            let lex = m
                .get("value")
                .and_then(Value::as_str)
                .ok_or_else(|| ShExMapError::binding_tree(format!("a literal needs a \"value\": {value}")))?;
            let lang = m.get("language").and_then(Value::as_str);
            let dt = m.get("type").and_then(Value::as_str);
            literal(lex, lang, dt)
        },
        Value::String(s) => parse_term(s),
        _ => Err(ShExMapError::binding_tree(format!("not a ShExMap bound term: {value}"))),
    }
}

/// A term from its string form: `_:label`, `<iri>`, `"literal"`, or a bare IRI.
pub fn parse_term(s: &str) -> Result<Object, ShExMapError> {
    let s = s.trim();
    if let Some(label) = s.strip_prefix("_:") {
        return Ok(Object::bnode(label.to_string()));
    }
    if s.starts_with('"') {
        return n3_literal(s);
    }
    let iri = if s.starts_with('<') && s.ends_with('>') {
        &s[1..s.len() - 1]
    } else {
        s
    };
    Ok(Object::iri(IriS::new_unchecked(iri)))
}

fn n3_literal(s: &str) -> Result<Object, ShExMapError> {
    let close = s
        .rfind('"')
        .filter(|&i| i > 0)
        .ok_or_else(|| ShExMapError::binding_tree(format!("not a quoted literal: {s}")))?;
    let lex = unescape(&s[1..close]);
    let rest = &s[close + 1..];
    if rest.is_empty() {
        return literal(&lex, None, None);
    }
    if let Some(lang) = rest.strip_prefix('@') {
        return literal(&lex, Some(lang), None);
    }
    if let Some(dt) = rest.strip_prefix("^^") {
        let dt = dt.trim_start_matches('<').trim_end_matches('>');
        return literal(&lex, None, Some(dt));
    }
    Err(ShExMapError::binding_tree(format!("not a quoted literal: {s}")))
}

/// A literal from its parts.
pub fn literal(lex: &str, lang: Option<&str>, datatype: Option<&str>) -> Result<Object, ShExMapError> {
    let lit = match (lang, datatype) {
        (Some(lang), _) => ConcreteLiteral::lang_str(
            lex,
            Lang::new(lang).map_err(|e| ShExMapError::binding_tree(format!("bad language tag {lang}: {e}")))?,
        ),
        (None, Some(dt)) if dt != XSD_STRING => typed_literal(lex, &IriS::new_unchecked(dt)),
        _ => ConcreteLiteral::str(lex),
    };
    Ok(Object::literal(lit))
}

const LEXICAL_KEPT: [&str; 4] = [
    "http://www.w3.org/2001/XMLSchema#integer",
    "http://www.w3.org/2001/XMLSchema#decimal",
    "http://www.w3.org/2001/XMLSchema#double",
    "http://www.w3.org/2001/XMLSchema#float",
];

/// A typed literal in the form the RDF parsers give it, so that a term read from JSON equals
/// the same term read from a graph: the bare numeric types keep their lexical form, other
/// known datatypes are parsed to their value (a dateTime, a boolean, ...), unknown ones are
/// kept as written.
pub fn typed_literal(lex: &str, datatype: &IriS) -> ConcreteLiteral {
    let raw = ConcreteLiteral::lit_datatype(lex, &IriRef::iri(datatype.clone()));
    if LEXICAL_KEPT.contains(&datatype.as_str()) {
        return raw;
    }
    raw.clone().into_checked_literal().unwrap_or(raw)
}

/// A plain literal: whether it has neither a language tag nor a datatype other than xsd:string.
pub fn is_plain_literal(term: &Object) -> bool {
    match term {
        Object::Literal(lit) => {
            lit.lang().is_none()
                && lit
                    .datatype()
                    .get_iri()
                    .map(|d| d.as_str() == XSD_STRING)
                    .unwrap_or(false)
        },
        _ => false,
    }
}

/// A short, reproducible digest of some strings: FNV-1a over them, NUL-separated, as 16
/// hex digits.  Used for the labels of minted blank nodes and for deduplication keys.
pub fn digest<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut first = true;
    for part in parts {
        if !first {
            h ^= 0;
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
        first = false;
        for b in part.bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    }
    format!("{h:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_round_trip() {
        let terms = vec![
            Object::iri(IriS::new_unchecked("http://a.example/x")),
            Object::bnode("b0".to_string()),
            literal("Sue", None, None).unwrap(),
            literal("110", None, Some("http://www.w3.org/2001/XMLSchema#float")).unwrap(),
            literal("bonjour", Some("fr"), None).unwrap(),
        ];
        for t in terms {
            assert_eq!(term_from_json(&term_to_json(&t)).unwrap(), t, "{t}");
            assert_eq!(parse_term(&n3(&t)).unwrap(), t, "{}", n3(&t));
        }
        assert_eq!(
            term_to_json(&literal("110", None, Some("http://www.w3.org/2001/XMLSchema#float")).unwrap()),
            json!({"value": "110", "type": "http://www.w3.org/2001/XMLSchema#float"})
        );
        assert_eq!(
            term_to_json(&literal("Sue", None, None).unwrap()),
            json!({"value": "Sue"})
        );
    }

    #[test]
    fn digests_are_stable() {
        assert_eq!(digest(["a", "b"]), digest(["a", "b"]));
        assert_ne!(digest(["a", "b"]), digest(["ab"]));
        assert_eq!(digest(["x"]).len(), 16);
    }
}
