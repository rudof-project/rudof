use crate::rdf_core::vocabs::ShaclVocab;
use crate::rdf_core::{
    FocusRDF, RDFError, SHACLPath,
    parser::rdf_node_parser::{
        RDFNodeParse,
        constructors::{ListParser, SingleValuePropertyParser},
    },
    term::Iri,
};
use iri_s::IriS;

/// Parser that interprets an RDF term as a SHACL path expression.
#[derive(Debug, Clone)]
pub struct ShaclPathParser<RDF>
where
    RDF: FocusRDF,
{
    term: RDF::Term,
}

impl<RDF> ShaclPathParser<RDF>
where
    RDF: FocusRDF,
{
    pub fn new(term: RDF::Term) -> Self {
        Self { term }
    }
}

impl<RDF> RDFNodeParse<RDF> for ShaclPathParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = SHACLPath;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        // Set focus to the term we want to parse as a path
        rdf.set_focus(&self.term);

        // Check if it's a simple IRI path
        if let Ok(iri) = RDF::term_as_iri(&self.term) {
            return Ok(SHACLPath::iri(IriS::new_unchecked(iri.as_str())));
        }

        // Check if it's a blank node (complex path expression)
        if RDF::term_as_bnode(&self.term).is_ok() {
            // Try each path construct in order (Sequence, Alternative, etc.)
            return try_parse_complex_path(rdf);
        }

        // Literals are not valid SHACL paths
        Err(RDFError::InvalidSHACLPathError {
            node: self.term.to_string(),
            error: Box::new(RDFError::ConversionError {
                msg: "SHACL path cannot be a literal".into(),
            }),
        })
    }
}

/// Attempts to parse a complex SHACL path (blank node with path construct).
fn try_parse_complex_path<RDF>(rdf: &mut RDF) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    // Try sequence path (RDF list at focus)
    if let Ok(path) = parse_sequence(rdf) {
        return Ok(path);
    }

    // Try alternative path (sh:alternativePath)
    if let Ok(path) = parse_alternative(rdf) {
        return Ok(path);
    }

    // Try zero-or-more path (sh:zeroOrMorePath)
    if let Ok(path) = parse_zero_or_more(rdf) {
        return Ok(path);
    }

    // Try one-or-more path (sh:oneOrMorePath)
    if let Ok(path) = parse_one_or_more(rdf) {
        return Ok(path);
    }

    // Try zero-or-one path (sh:zeroOrOnePath)
    if let Ok(path) = parse_zero_or_one(rdf) {
        return Ok(path);
    }

    // Try inverse path (sh:inversePath)
    if let Ok(path) = parse_inverse(rdf) {
        return Ok(path);
    }

    Err(RDFError::InvalidSHACLPathError {
        node: rdf.get_focus().map(|t| t.to_string()).unwrap_or_default(),
        error: Box::new(RDFError::ConversionError {
            msg: "Unsupported SHACL path construct".into(),
        }),
    })
}

/// Parses a SHACL sequence path from an RDF list at the current focus.
fn parse_sequence<RDF>(rdf: &mut RDF) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    // Parse the list structure at current focus
    let terms = ListParser::new().parse_focused(rdf)?;

    // Recursively parse each element as a SHACL path
    let mut paths = Vec::new();
    for term in terms {
        let path = ShaclPathParser::new(term).parse_focused(rdf)?;
        paths.push(path);
    }

    Ok(SHACLPath::sequence(paths))
}

/// Parses a SHACL alternative path using `sh:alternativePath`.
fn parse_alternative<RDF>(rdf: &mut RDF) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    // Get the list pointed to by sh:alternativePath
    let list_term = SingleValuePropertyParser::new(ShaclVocab::sh_alternative_path().clone()).parse_focused(rdf)?;
    rdf.set_focus(&list_term);

    // Parse the RDF list
    let terms = ListParser::new().parse_focused(rdf)?;

    // Parse each element as a SHACL path
    let mut paths = Vec::new();
    for term in terms {
        let path = ShaclPathParser::new(term).parse_focused(rdf)?;
        paths.push(path);
    }

    Ok(SHACLPath::alternative(paths))
}

/// Parses a SHACL zero-or-more path using `sh:zeroOrMorePath`.
fn parse_zero_or_more<RDF>(rdf: &mut RDF) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    let term = SingleValuePropertyParser::new(ShaclVocab::sh_zero_or_more_path().clone()).parse_focused(rdf)?;
    let sub_path = ShaclPathParser::new(term).parse_focused(rdf)?;
    Ok(SHACLPath::zero_or_more(sub_path))
}

/// Parses a SHACL one-or-more path using `sh:oneOrMorePath`.
fn parse_one_or_more<RDF>(rdf: &mut RDF) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    let term = SingleValuePropertyParser::new(ShaclVocab::sh_one_or_more_path().clone()).parse_focused(rdf)?;
    let sub_path = ShaclPathParser::new(term).parse_focused(rdf)?;
    Ok(SHACLPath::one_or_more(sub_path))
}

/// Parses a SHACL zero-or-one path using `sh:zeroOrOnePath`.
fn parse_zero_or_one<RDF>(rdf: &mut RDF) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    let term = SingleValuePropertyParser::new(ShaclVocab::sh_zero_or_one_path().clone()).parse_focused(rdf)?;
    let sub_path = ShaclPathParser::new(term).parse_focused(rdf)?;
    Ok(SHACLPath::zero_or_one(sub_path))
}

/// Parses a SHACL inverse path using `sh:inversePath`.
fn parse_inverse<RDF>(rdf: &mut RDF) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    let term = SingleValuePropertyParser::new(ShaclVocab::sh_inverse_path().clone()).parse_focused(rdf)?;
    let sub_path = ShaclPathParser::new(term).parse_focused(rdf)?;
    Ok(SHACLPath::inverse(sub_path))
}
