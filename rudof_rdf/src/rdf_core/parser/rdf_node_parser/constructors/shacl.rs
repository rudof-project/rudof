use crate::rdf_core::vocabs::ShaclVocab;
use crate::rdf_core::{
    FocusRDF, RDFError, SHACLPath,
    parser::rdf_node_parser::{
        RDFNodeParse,
        constructors::{ListParser, SingleValuePropertyParser},
    },
    term::Iri,
};
use rudof_iri::IriS;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Real paths nest a few levels; each level is a stack frame here and in the derived impls of [`SHACLPath`].
pub const MAX_PATH_DEPTH: usize = 64;
/// Steps that parsing the SHACL paths and node expressions of a graph may take, however few triples it has.
pub const MIN_PARSE_STEPS: usize = 4096;
/// Further steps that parsing may take per triple of the graph. A path step or expression has triples
/// of its own unless it is a blank node used more than once, which is what makes a parse outgrow its graph.
pub const PARSE_STEPS_PER_TRIPLE: usize = 16;

/// The steps that parsing SHACL paths and node expressions may take, shared by every parse it is
/// handed to, so a path or expression shared by many shapes cannot multiply the work.
/// Atomics rather than cells keep the parsers that hold it `Send` and `Sync`.
#[derive(Debug)]
pub struct ParseBudget {
    steps_left: AtomicUsize,
    graph_counted: AtomicBool,
}

impl Default for ParseBudget {
    fn default() -> Self {
        Self {
            steps_left: AtomicUsize::new(MIN_PARSE_STEPS),
            graph_counted: AtomicBool::new(false),
        }
    }
}

impl ParseBudget {
    /// Runs `parse` with the steps left. Most parses are small, so the graph is only counted the first
    /// time the steps run out, adding `PARSE_STEPS_PER_TRIPLE` per triple, and that parse is run again.
    pub fn spend<RDF: FocusRDF, T>(
        &self,
        rdf: &mut RDF,
        parse: impl Fn(&mut RDF, &mut usize) -> Result<T, RDFError>,
    ) -> Result<T, RDFError> {
        let before = self.steps_left.load(Ordering::Relaxed);
        let mut steps = before;
        let mut out = parse(rdf, &mut steps);
        if out.is_err() && steps == 0 && !self.graph_counted.swap(true, Ordering::Relaxed) {
            let triples = rdf.triples().map_or(0, Iterator::count);
            steps = before + PARSE_STEPS_PER_TRIPLE * triples;
            out = parse(rdf, &mut steps);
        }
        self.steps_left.store(steps, Ordering::Relaxed);
        out
    }
}

/// Parser that interprets an RDF term as a SHACL path expression.
#[derive(Debug, Clone)]
pub struct ShaclPathParser<RDF>
where
    RDF: FocusRDF,
{
    term: RDF::Term,
    budget: Arc<ParseBudget>,
}

impl<RDF> ShaclPathParser<RDF>
where
    RDF: FocusRDF,
{
    /// A parser with a budget of its own, as for a path read from a report.
    pub fn new(term: RDF::Term) -> Self {
        Self::with_budget(term, Arc::default())
    }

    /// A parser that takes its steps from `budget`.
    pub fn with_budget(term: RDF::Term, budget: Arc<ParseBudget>) -> Self {
        Self { term, budget }
    }
}

impl<RDF> RDFNodeParse<RDF> for ShaclPathParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = SHACLPath;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        self.budget
            .spend(rdf, |rdf, steps| parse_path(&self.term, rdf, 0, steps))
    }
}

/// Parses `term` as a SHACL path nested `depth` levels deep, taking one of the `steps` left for each path.
pub fn parse_path<RDF>(term: &RDF::Term, rdf: &mut RDF, depth: usize, steps: &mut usize) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    // A cyclic path would nest forever, and a blank node used twice is parsed twice,
    // so N nested `( _:x _:x )` would make 2^N steps
    if depth > MAX_PATH_DEPTH {
        return Err(RDFError::InvalidSHACLPathError {
            node: term.to_string(),
            error: Box::new(RDFError::ConversionError {
                msg: format!("SHACL path is cyclic or nested more than {MAX_PATH_DEPTH} levels deep"),
            }),
        });
    }
    let Some(left) = steps.checked_sub(1) else {
        return Err(RDFError::InvalidSHACLPathError {
            node: term.to_string(),
            error: Box::new(RDFError::ConversionError {
                msg: format!(
                    "parsing takes more than {MIN_PARSE_STEPS} steps plus {PARSE_STEPS_PER_TRIPLE} per triple of the graph"
                ),
            }),
        });
    };
    *steps = left;

    // Set focus to the term we want to parse as a path
    rdf.set_focus(term);

    // Check if it's a simple IRI path
    if let Ok(iri) = RDF::term_as_iri(term) {
        return Ok(SHACLPath::iri(IriS::new_unchecked(iri.as_str())));
    }

    // Check if it's a blank node (complex path expression)
    if RDF::term_as_bnode(term).is_ok() {
        return try_parse_complex_path(rdf, depth, steps);
    }

    // Literals are not valid SHACL paths
    Err(RDFError::InvalidSHACLPathError {
        node: term.to_string(),
        error: Box::new(RDFError::ConversionError {
            msg: "SHACL path cannot be a literal".into(),
        }),
    })
}

/// Attempts to parse a complex SHACL path (blank node with path construct).
fn try_parse_complex_path<RDF>(rdf: &mut RDF, depth: usize, steps: &mut usize) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    // Try each path construct in order. Only parse_path returns InvalidSHACLPathError,
    // so the construct was found and one of its sub-paths is invalid.
    for parse in [
        parse_sequence,
        parse_alternative,
        parse_zero_or_more,
        parse_one_or_more,
        parse_zero_or_one,
        parse_inverse,
    ] {
        match parse(rdf, depth, steps) {
            Ok(path) => return Ok(path),
            Err(e @ RDFError::InvalidSHACLPathError { .. }) => return Err(e),
            Err(_) => {},
        }
    }

    Err(RDFError::InvalidSHACLPathError {
        node: rdf.get_focus().map(|t| t.to_string()).unwrap_or_default(),
        error: Box::new(RDFError::ConversionError {
            msg: "Unsupported SHACL path construct".into(),
        }),
    })
}

/// Parses a SHACL sequence path from an RDF list at the current focus.
fn parse_sequence<RDF>(rdf: &mut RDF, depth: usize, steps: &mut usize) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    // Parse the list structure at current focus
    let terms = ListParser::new().parse_focused(rdf)?;

    // Recursively parse each element as a SHACL path
    let mut paths = Vec::new();
    for term in terms {
        let path = parse_path(&term, rdf, depth + 1, steps)?;
        paths.push(path);
    }

    Ok(SHACLPath::sequence(paths))
}

/// Parses a SHACL alternative path using `sh:alternativePath`.
fn parse_alternative<RDF>(rdf: &mut RDF, depth: usize, steps: &mut usize) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    // Get the list pointed to by sh:alternativePath
    let list_term = SingleValuePropertyParser::new(ShaclVocab::sh_alternative_path()).parse_focused(rdf)?;
    rdf.set_focus(&list_term);

    // Parse the RDF list
    let terms = ListParser::new().parse_focused(rdf)?;

    // Parse each element as a SHACL path
    let mut paths = Vec::new();
    for term in terms {
        let path = parse_path(&term, rdf, depth + 1, steps)?;
        paths.push(path);
    }

    Ok(SHACLPath::alternative(paths))
}

/// Parses a SHACL zero-or-more path using `sh:zeroOrMorePath`.
fn parse_zero_or_more<RDF>(rdf: &mut RDF, depth: usize, steps: &mut usize) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    let term = SingleValuePropertyParser::new(ShaclVocab::sh_zero_or_more_path()).parse_focused(rdf)?;
    let sub_path = parse_path(&term, rdf, depth + 1, steps)?;
    Ok(SHACLPath::zero_or_more(sub_path))
}

/// Parses a SHACL one-or-more path using `sh:oneOrMorePath`.
fn parse_one_or_more<RDF>(rdf: &mut RDF, depth: usize, steps: &mut usize) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    let term = SingleValuePropertyParser::new(ShaclVocab::sh_one_or_more_path()).parse_focused(rdf)?;
    let sub_path = parse_path(&term, rdf, depth + 1, steps)?;
    Ok(SHACLPath::one_or_more(sub_path))
}

/// Parses a SHACL zero-or-one path using `sh:zeroOrOnePath`.
fn parse_zero_or_one<RDF>(rdf: &mut RDF, depth: usize, steps: &mut usize) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    let term = SingleValuePropertyParser::new(ShaclVocab::sh_zero_or_one_path()).parse_focused(rdf)?;
    let sub_path = parse_path(&term, rdf, depth + 1, steps)?;
    Ok(SHACLPath::zero_or_one(sub_path))
}

/// Parses a SHACL inverse path using `sh:inversePath`.
fn parse_inverse<RDF>(rdf: &mut RDF, depth: usize, steps: &mut usize) -> Result<SHACLPath, RDFError>
where
    RDF: FocusRDF,
{
    let term = SingleValuePropertyParser::new(ShaclVocab::sh_inverse_path()).parse_focused(rdf)?;
    let sub_path = parse_path(&term, rdf, depth + 1, steps)?;
    Ok(SHACLPath::inverse(sub_path))
}
