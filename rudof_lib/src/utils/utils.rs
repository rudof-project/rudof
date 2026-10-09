use crate::{
    Result, Rudof,
    errors::{DataError, IriError},
    formats::{IriNormalizationMode, QueryType},
    types::Data,
};
#[cfg(not(target_family = "wasm"))]
use crossterm::terminal;
use prefixmap::{IriRef, PrefixMap};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::{NeighsRDF, Rdf, query::SparqlQuery, term::Object, term::literal::ConcreteLiteral};
use shex_ast::{ObjectValue, ShapeMapParser, shapemap::NodeSelector};
use sparql_service::RdfData;
#[cfg(not(target_family = "wasm"))]
use std::env;
use std::str::FromStr;
#[cfg(not(target_family = "wasm"))]
use url::Url;

/// A term of the loaded RDF data.
pub type RdfTerm = <RdfData as Rdf>::Term;

/// A term of the loaded RDF data in subject position: an IRI or a blank node.
pub type RdfSubject = <RdfData as Rdf>::Subject;

/// A term of the loaded RDF data in predicate position: an IRI.
pub type RdfIri = <RdfData as Rdf>::IRI;

/// Normalizes a node/shape IRI string for `ShapeMapParser` according to `mode`.
///
/// - `Lax`: wraps strings that look like bare absolute IRIs (contain `://`, no leading `<`, `_`,
///   `{` or `"`) in angle brackets. Heuristic — see [`IriNormalizationMode`] docs for edge cases.
/// - `Strict`: returns the trimmed string unchanged; bare IRIs will produce a parser error.
pub(crate) fn normalize_iri_str(s: &str, mode: IriNormalizationMode) -> String {
    let trimmed = s.trim();
    match mode {
        IriNormalizationMode::Strict => trimmed.to_string(),
        IriNormalizationMode::Lax => {
            let is_bare_iri = !trimmed.starts_with('<')
                && !trimmed.starts_with('_')
                && !trimmed.starts_with('{')
                && !trimmed.starts_with('"')
                && trimmed.contains("://");
            if is_bare_iri {
                format!("<{}>", trimmed)
            } else {
                trimmed.to_string()
            }
        },
    }
}

pub fn get_base_iri(rudof: &mut Rudof, base_iri: Option<&str>) -> Result<IriS> {
    if let Some(base_iri) = base_iri {
        let base_iri = IriS::from_str(base_iri).map_err(|error| IriError::ParseError {
            iri: base_iri.to_string(),
            error: error.to_string(),
        })?;

        Ok(base_iri.clone())
    } else if let Some(base_iri) = rudof.config.shex().base() {
        Ok(base_iri.clone())
    } else {
        // There is no current directory to default to on wasm.
        #[cfg(target_family = "wasm")]
        return Err(IriError::WasmNotSupported {
            operation: "defaulting the base IRI to the current directory (provide a base IRI)".to_string(),
        }
        .into());
        #[cfg(not(target_family = "wasm"))]
        {
            let cwd = env::current_dir().map_err(|e| IriError::PathConversionError {
                path: ".".to_string(),
                error: format!("Error resolving base IRI. Failed to get current directory: {e}"),
            })?;

            let url = Url::from_directory_path(&cwd).map_err(|_| IriError::PathConversionError {
                path: cwd.to_string_lossy().to_string(),
                error: "Error resolving base IRI. Cannot convert current directory to a file URL".to_string(),
            })?;
            Ok(url.into())
        }
    }
}

/// Whether tables and other text output use colors and hyperlinks (ANSI
/// escape codes). There is no terminal on wasm, where the output is usually
/// shown in a web page and the escape codes would appear as stray characters.
pub fn terminal_colors() -> bool {
    !cfg!(target_family = "wasm")
}

#[cfg(not(target_family = "wasm"))]
const MAX_TERMINAL_WIDTH: usize = 100;
const DEFAULT_TERMINAL_WIDTH: usize = 80;

/// Width of the terminal, used to lay out tables. There is no terminal on
/// wasm, so the default width is used there.
#[cfg(target_family = "wasm")]
pub fn terminal_width() -> usize {
    DEFAULT_TERMINAL_WIDTH
}

#[cfg(not(target_family = "wasm"))]
pub fn terminal_width() -> usize {
    if let Ok((cols, _)) = terminal::size() {
        sanitize_width(cols as usize)
    } else {
        DEFAULT_TERMINAL_WIDTH
    }
}

#[cfg(not(target_family = "wasm"))]
fn sanitize_width(width: usize) -> usize {
    match width {
        w if w > MAX_TERMINAL_WIDTH => MAX_TERMINAL_WIDTH,
        w if w < 40 => DEFAULT_TERMINAL_WIDTH,
        w => w,
    }
}

// Detect query type from SPARQL string
pub fn detect_query_type(query: &SparqlQuery) -> QueryType {
    if query.is_select() {
        QueryType::Select
    } else if query.is_construct() {
        QueryType::Construct
    } else if query.is_ask() {
        QueryType::Ask
    } else {
        QueryType::Describe
    }
}

/// Parses a node selector string into a `NodeSelector` instance.
pub fn parse_node_selector(node: &str, iri_mode: IriNormalizationMode) -> Result<NodeSelector> {
    let normalized = normalize_iri_str(node, iri_mode);
    ShapeMapParser::parse_node_selector(normalized.as_str()).map_err(|e| {
        Box::new(DataError::FailedNodeSelectorParse {
            node: normalized.as_str().to_string(),
            error: e.to_string(),
        })
        .into()
    })
}

/// Converts predicate strings to IRI objects
pub fn convert_predicate_strings_to_iris<S>(predicates: &[String], rdf: &S) -> Result<Vec<S::IRI>>
where
    S: NeighsRDF,
{
    predicates
        .iter()
        .map(|pred_str| {
            let iri = resolve_iri_str(pred_str, |prefix, local| rdf.resolve_prefix_local(prefix, local))?;
            Ok(iri.into())
        })
        .collect()
}

/// Parses a predicate (an angle-bracketed IRI or a prefixed name) and resolves it
/// against `prefixmap`.
///
/// The single-predicate counterpart of [`convert_predicate_strings_to_iris`], for the
/// callers that resolve against a prefix map rather than the loaded data (see
/// [`resolution_prefixmap`]). A bare IRI is not accepted, as there too.
pub fn parse_predicate_iri(predicate: &str, prefixmap: &PrefixMap) -> Result<IriS> {
    resolve_iri_str(predicate, |prefix, local| prefixmap.resolve_prefix_local(prefix, local))
}

/// Parses an IRI reference and resolves a prefixed one with `resolve`.
fn resolve_iri_str<F>(iri: &str, resolve: F) -> Result<IriS>
where
    F: FnOnce(&str, &str) -> std::result::Result<IriS, prefixmap::error::PrefixMapError>,
{
    match parse_iri_ref(iri)? {
        IriRef::Iri(iri) => Ok(iri),
        IriRef::Prefixed { prefix, local } => resolve(prefix.as_str(), local.as_str()).map_err(|e| {
            Box::new(DataError::FailedPrefixResolution {
                prefix: prefix.to_string(),
                error: e.to_string(),
            })
            .into()
        }),
    }
}

/// The RDF data loaded in the session.
///
/// # Errors
///
/// Returns [`DataError::NoRdfDataLoaded`] if no data is loaded, or if what is loaded is
/// a property graph.
pub fn rdf_data(rudof: &Rudof) -> Result<&RdfData> {
    match rudof.data.as_ref() {
        Some(Data::RDFData(rdf)) => Ok(rdf),
        _ => Err(Box::new(DataError::NoRdfDataLoaded).into()),
    }
}

/// The RDF data loaded in the session, to modify.
///
/// # Errors
///
/// As [`rdf_data`].
pub fn rdf_data_mut(rudof: &mut Rudof) -> Result<&mut RdfData> {
    match rudof.data.as_mut() {
        Some(Data::RDFData(rdf)) => Ok(rdf),
        _ => Err(Box::new(DataError::NoRdfDataLoaded).into()),
    }
}

/// Checks that the loaded RDF data can be modified.
///
/// Only the in-memory graph can: a SPARQL endpoint and the QLever backend are
/// read-only, and so is data federated with SPARQL endpoints, where a triple added to
/// the local graph could never be removed from the endpoint's and the two cannot be
/// told apart once they are read back together.
///
/// # Errors
///
/// Returns [`DataError::ReadOnlyData`] naming what makes the data read-only.
pub fn check_data_is_mutable(rdf: &RdfData) -> Result<()> {
    let backend = rdf.backend();
    if backend.is_read_only() {
        return Err(Box::new(DataError::ReadOnlyData {
            reason: format!("the {} backend holding it is read-only", backend.variant_name()),
        })
        .into());
    }
    #[cfg(not(target_family = "wasm"))]
    if !rdf.use_endpoints().is_empty() {
        let mut endpoints: Vec<&str> = rdf.use_endpoints().keys().map(String::as_str).collect();
        endpoints.sort_unstable();
        return Err(Box::new(DataError::ReadOnlyData {
            reason: format!(
                "it is federated with the read-only SPARQL endpoint(s) {}",
                endpoints.join(", ")
            ),
        })
        .into());
    }
    Ok(())
}

/// Parses the three terms of a triple written as text, as [`parse_term`] does,
/// resolving prefixed names against the prefixes of the loaded data and the session
/// (see [`resolution_prefixmap`]).
///
/// # Errors
///
/// Returns an error if no RDF data is loaded, or if a term cannot be parsed: the
/// subject must be an IRI or a blank node and the predicate an IRI.
pub fn parse_triple(
    rudof: &Rudof,
    subject: &str,
    predicate: &str,
    object: &str,
    iri_mode: IriNormalizationMode,
) -> Result<(RdfSubject, RdfIri, RdfTerm)> {
    let prefixmap = resolution_prefixmap(rudof, rdf_data(rudof)?);
    Ok((
        parse_subject(subject, &prefixmap, iri_mode)?,
        parse_predicate_iri(predicate, &prefixmap)?.into(),
        parse_term(object, &prefixmap, iri_mode)?,
    ))
}

/// The prefix map that resolves prefixed names in terms given as text: the prefixes
/// of the loaded data, filled in with the session's default prefixes for the aliases
/// the data does not declare itself.
///
/// The same precedence [`default_prefix_header`](super::default_prefix_header) gives
/// the default prefixes when a document is parsed: they cover what the data leaves
/// out, they never override what it declares.
pub fn resolution_prefixmap(rudof: &Rudof, rdf: &RdfData) -> PrefixMap {
    let mut prefixmap = rudof.prefixes.clone().unwrap_or_default();
    prefixmap.merge(rdf.prefixmap().unwrap_or_default());
    prefixmap
}

/// Parses a term written as text into an RDF term, resolving
/// prefixed names against `prefixmap`.
///
/// Terms are parsed with the same parser as node selectors, so the syntax is the one
/// the rest of the API accepts, but only a single concrete node is a term: a triple
/// pattern or a SPARQL query selects nodes from the data instead of denoting one, and
/// is rejected.
pub fn parse_term(term: &str, prefixmap: &PrefixMap, iri_mode: IriNormalizationMode) -> Result<RdfTerm> {
    let object = match parse_node_selector(term, iri_mode)? {
        NodeSelector::Node(ObjectValue::IriRef(iri_ref)) => {
            let iri = iri_ref.get_iri_prefixmap(prefixmap).map_err(|e| {
                Box::new(DataError::FailedPrefixResolution {
                    prefix: iri_ref.to_string(),
                    error: e.to_string(),
                })
            })?;
            Object::Iri(iri.into_owned())
        },
        NodeSelector::Node(ObjectValue::Literal(literal)) => {
            Object::Literal(resolve_literal_datatype(literal, prefixmap)?)
        },
        NodeSelector::BNode(label) => Object::BlankNode(label),
        _ => {
            return Err(Box::new(DataError::NotATerm {
                term: term.trim().to_string(),
            })
            .into());
        },
    };
    Ok(object.into())
}

/// Resolves the datatype of a literal written with a prefixed one
/// (`"23"^^xsd:integer`) against `prefixmap`.
fn resolve_literal_datatype(literal: ConcreteLiteral, prefixmap: &PrefixMap) -> Result<ConcreteLiteral> {
    let ConcreteLiteral::DatatypeLiteral { lexical_form, datatype } = &literal else {
        return Ok(literal);
    };
    let IriRef::Prefixed { prefix, local } = datatype else {
        return Ok(literal);
    };
    let datatype = prefixmap.resolve_prefix_local(prefix, local).map_err(|e| {
        Box::new(DataError::FailedPrefixResolution {
            prefix: prefix.to_string(),
            error: e.to_string(),
        })
    })?;
    Ok(ConcreteLiteral::DatatypeLiteral {
        lexical_form: lexical_form.clone(),
        datatype: IriRef::Iri(datatype),
    })
}

/// Parses a term written as text, as [`parse_term`] does, and takes it as a subject.
///
/// A literal cannot be a subject, and is rejected.
pub fn parse_subject(subject: &str, prefixmap: &PrefixMap, iri_mode: IriNormalizationMode) -> Result<RdfSubject> {
    let term = parse_term(subject, prefixmap, iri_mode)?;
    RdfData::term_as_subject(&term).map_err(|_| {
        Box::new(DataError::NotASubject {
            term: subject.trim().to_string(),
        })
        .into()
    })
}

/// Parses an IRI string into an `IriRef` instance.
fn parse_iri_ref(iri: &str) -> Result<IriRef> {
    ShapeMapParser::parse_iri_ref(iri).map_err(|e| {
        Box::new(DataError::FailedIriRefParse {
            iri: iri.to_string(),
            error: e.to_string(),
        })
        .into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::IriNormalizationMode::{Lax, Strict};
    use rudof_rdf::rdf_core::term::literal::Lang;

    fn prefixmap() -> PrefixMap {
        let mut pm = PrefixMap::new();
        pm.add_prefix("ex", IriS::new_unchecked("http://example.org/"));
        pm.add_prefix("xsd", IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#"));
        pm
    }

    fn term(text: &str) -> RdfTerm {
        parse_term(text, &prefixmap(), Lax).unwrap()
    }

    fn alice() -> RdfTerm {
        Object::Iri(IriS::new_unchecked("http://example.org/alice")).into()
    }

    #[test]
    fn test_parse_term_iris() {
        assert_eq!(term("<http://example.org/alice>"), alice());
        assert_eq!(term("ex:alice"), alice());
        // A bare IRI is accepted in the lax mode, and not in the strict one.
        assert_eq!(term("http://example.org/alice"), alice());
        assert!(parse_term("http://example.org/alice", &prefixmap(), Strict).is_err());
    }

    #[test]
    fn test_parse_term_blank_nodes_and_literals() {
        assert_eq!(term("_:b1"), RdfTerm::from(Object::BlankNode("b1".to_string())));
        assert_eq!(
            term("\"Alice\""),
            RdfTerm::from(Object::Literal(ConcreteLiteral::str("Alice")))
        );
        assert_eq!(
            term("\"Alice\"@en"),
            RdfTerm::from(Object::Literal(ConcreteLiteral::lang_str(
                "Alice",
                Lang::new("en").unwrap()
            )))
        );
        assert_eq!(term("23"), RdfTerm::from(Object::Literal(ConcreteLiteral::integer(23))));
        assert_eq!(
            term("\"23\"^^xsd:integer"),
            RdfTerm::from(Object::Literal(ConcreteLiteral::integer(23)))
        );
    }

    #[test]
    fn test_parse_term_keeps_a_prefixed_datatype() {
        // A datatype left as a prefixed name is dropped when the literal becomes a
        // term, which would store `"23"` as a string instead of an integer.
        let typed = term("\"23\"^^xsd:integer");
        assert_eq!(typed, term("\"23\"^^<http://www.w3.org/2001/XMLSchema#integer>"));
        assert_eq!(typed, term("23"));
        assert_ne!(typed, term("\"23\""));

        // And an alias nothing declares is an error, not a silently untyped literal.
        let err = parse_term("\"23\"^^other:integer", &prefixmap(), Lax).unwrap_err();
        assert!(err.to_string().contains("other"), "{err}");
    }

    #[test]
    fn test_parse_term_rejects_what_is_not_a_term() {
        // A prefix nothing declares, and selectors that denote a set of nodes rather
        // than one term.
        for text in [
            "other:alice",
            "{FOCUS ex:p ex:alice}",
            "SPARQL 'SELECT ?x WHERE { ?x ?p ?o }'",
        ] {
            assert!(parse_term(text, &prefixmap(), Lax).is_err(), "{text} parsed as a term");
        }
    }

    #[test]
    fn test_parse_subject() {
        assert_eq!(
            parse_subject("ex:alice", &prefixmap(), Lax).unwrap().to_string(),
            "<http://example.org/alice>"
        );
        assert!(parse_subject("_:b1", &prefixmap(), Lax).is_ok());
        let err = parse_subject("\"Alice\"", &prefixmap(), Lax).unwrap_err();
        assert!(err.to_string().contains("cannot be the subject"), "{err}");
    }

    #[test]
    fn test_normalize_iri_str_wraps_only_bare_iris() {
        assert_eq!(normalize_iri_str("http://example.org/a", Lax), "<http://example.org/a>");
        assert_eq!(
            normalize_iri_str("http://example.org/a", Strict),
            "http://example.org/a"
        );
        assert_eq!(
            normalize_iri_str("<http://example.org/a>", Lax),
            "<http://example.org/a>"
        );
        assert_eq!(normalize_iri_str(":a", Lax), ":a");
        assert_eq!(normalize_iri_str("_:b1", Lax), "_:b1");
    }

    #[test]
    fn test_normalize_iri_str_leaves_literals_alone() {
        // A literal is not a bare IRI, however much of one its datatype looks like.
        assert_eq!(
            normalize_iri_str("\"23\"^^<http://www.w3.org/2001/XMLSchema#integer>", Lax),
            "\"23\"^^<http://www.w3.org/2001/XMLSchema#integer>"
        );
        assert_eq!(normalize_iri_str("\"Alice\"", Lax), "\"Alice\"");
    }

    #[test]
    fn test_resolution_prefixmap_fills_in_the_session_prefixes() {
        use crate::{Rudof, RudofConfig, formats::InputSpec, types::Data};

        let mut rudof = Rudof::new(RudofConfig::default());
        rudof.add_prefix("ex", "http://session.example/").execute().unwrap();
        rudof.add_prefix("other", "http://other.example/").execute().unwrap();
        let data = InputSpec::from_str("prefix ex: <http://data.example/>\nex:a ex:p 1 .").unwrap();
        rudof.load_data().with_data(&[data]).execute().unwrap();

        let Some(Data::RDFData(rdf)) = rudof.data.as_ref() else {
            panic!("no RDF data loaded")
        };
        let prefixmap = resolution_prefixmap(&rudof, rdf);
        assert_eq!(
            prefixmap.find("ex"),
            Some(&IriS::new_unchecked("http://data.example/")),
            "the data's own prefix wins over the session's"
        );
        assert_eq!(
            prefixmap.find("other"),
            Some(&IriS::new_unchecked("http://other.example/")),
            "a session prefix the data does not declare is kept"
        );
    }

    #[test]
    fn test_parse_predicate_iri() {
        let pm = prefixmap();
        assert_eq!(
            parse_predicate_iri("ex:name", &pm).unwrap(),
            IriS::new_unchecked("http://example.org/name")
        );
        assert_eq!(
            parse_predicate_iri("<http://example.org/name>", &pm).unwrap(),
            IriS::new_unchecked("http://example.org/name")
        );
        // As for the predicates of `node_neighborhood`, a bare IRI is not accepted.
        assert!(parse_predicate_iri("http://example.org/name", &pm).is_err());
        assert!(parse_predicate_iri("other:name", &pm).is_err());
    }
}
