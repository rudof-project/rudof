use oxrdf::{NamedNode as OxNamedNode, Term as OxTerm};
use oxttl::TurtleParser;
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use thiserror::Error;

/// The IRI standing in for the subject and the predicate of the one-triple document a term
/// is parsed as. It never reaches the caller: only the object of that triple is the term.
const PLACEHOLDER_IRI: &str = "urn:rudof:term";

/// Parses a term written in RDF 1.2 Turtle syntax, resolving prefixed names against
/// `prefixmap` and relative IRIs against `base`.
///
/// The term is read as the object of a one-triple Turtle document, so only a single term is
/// accepted: text standing for several triples, such as a reified triple or a collection, is
/// refused rather than having one of its terms picked out.
///
/// # Errors
///
/// Returns [`TermParseError`] if the text is not valid Turtle, if it names no term or more
/// than one, or if it holds an IRI that cannot be made absolute.
pub fn parse_term(term: &str, base: Option<&IriS>, prefixmap: &PrefixMap) -> Result<OxTerm, TermParseError> {
    let term = term.trim();
    let mut parser = TurtleParser::new();
    if let Some(base) = base {
        parser = parser
            .with_base_iri(base.as_str())
            .map_err(|error| TermParseError::Base {
                base: base.to_string(),
                error: error.to_string(),
            })?;
    }
    for (alias, iri) in prefixmap.iter() {
        parser = parser
            .with_prefix(alias, iri.as_str())
            .map_err(|error| TermParseError::Prefix {
                alias: alias.to_string(),
                iri: iri.to_string(),
                error: error.to_string(),
            })?;
    }

    let document = format!("<{PLACEHOLDER_IRI}> <{PLACEHOLDER_IRI}> {term} .");
    let mut triples = Vec::new();
    for triple in parser.for_slice(&document) {
        triples.push(triple.map_err(|error| TermParseError::Syntax {
            term: term.to_string(),
            error: error.to_string(),
        })?);
    }

    let [triple] = triples.as_slice() else {
        return Err(TermParseError::NotASingleTerm {
            term: term.to_string(),
            triples: triples.len(),
        });
    };
    validate_term_iris(&triple.object).map_err(|error| TermParseError::Iri {
        term: term.to_string(),
        error,
    })?;
    Ok(triple.object.clone())
}

/// Checks that every IRI of `term` is absolute, inside a triple term as well.
///
/// The shallow counterpart of this, over the three positions of a triple, is
/// `validate_triple_iris` in the Oxigraph in-memory graph.
fn validate_term_iris(term: &OxTerm) -> Result<(), String> {
    match term {
        OxTerm::NamedNode(iri) => {
            OxNamedNode::new(iri.as_str()).map_err(|e| e.to_string())?;
        },
        OxTerm::Literal(literal) => {
            OxNamedNode::new(literal.datatype().as_str()).map_err(|e| e.to_string())?;
        },
        OxTerm::Triple(triple) => {
            if let oxrdf::NamedOrBlankNode::NamedNode(iri) = &triple.subject {
                OxNamedNode::new(iri.as_str()).map_err(|e| e.to_string())?;
            }
            OxNamedNode::new(triple.predicate.as_str()).map_err(|e| e.to_string())?;
            validate_term_iris(&triple.object)?;
        },
        OxTerm::BlankNode(_) => {},
    }
    Ok(())
}

/// The ways the text of a term can fail to name one.
#[derive(Error, Debug)]
pub enum TermParseError {
    #[error("Term {term} is not valid RDF 1.2 Turtle syntax: {error}")]
    Syntax { term: String, error: String },

    #[error("Term {term} does not name a single term, but {triples} RDF triples")]
    NotASingleTerm { term: String, triples: usize },

    #[error("Term {term} holds an IRI that is not absolute: {error}")]
    Iri { term: String, error: String },

    #[error("Base IRI {base} is not a valid IRI: {error}")]
    Base { base: String, error: String },

    #[error("Prefix {alias} is declared as the invalid IRI {iri}: {error}")]
    Prefix { alias: String, iri: String, error: String },
}
