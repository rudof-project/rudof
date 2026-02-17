use crate::rdf_core::{
    FocusRDF, RDFError,
    parser::rdf_node_parser::RDFNodeParse,
    term::{Iri, IriOrBlankNode, Object, literal::Literal},
};
use iri_s::{IriS, iri};
use std::marker::PhantomData;

/// A parser that extracts the focus node as an RDF object.
#[derive(Debug, Clone)]
pub struct ObjectParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> ObjectParser<RDF> {
    pub fn new() -> Self {
        ObjectParser { _marker: PhantomData }
    }
}

impl<RDF> Default for ObjectParser<RDF> {
    fn default() -> Self {
        Self::new()
    }
}

impl<RDF> RDFNodeParse<RDF> for ObjectParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = Object;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match rdf.get_focus() {
            Some(focus) => {
                let object: Object = focus.clone().try_into().map_err(|_| RDFError::ExpectedObjectError {
                    term: focus.to_string(),
                })?;
                Ok(object)
            },
            None => Err(RDFError::NoFocusNodeError),
        }
    }
}

/// A parser that extracts the focus node as its native term type.
#[derive(Debug, Clone)]
pub struct TermParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> TermParser<RDF> {
    pub fn new() -> Self {
        TermParser { _marker: PhantomData }
    }
}

impl<RDF> Default for TermParser<RDF> {
    fn default() -> Self {
        Self::new()
    }
}

impl<RDF> RDFNodeParse<RDF> for TermParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::Term;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match rdf.get_focus() {
            Some(focus) => Ok(focus.clone()),
            None => Err(RDFError::NoFocusNodeError),
        }
    }
}

/// A parser that extracts the focus node as an IRI.
#[derive(Debug, Clone)]
pub struct IriParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> IriParser<RDF> {
    pub fn new() -> Self {
        IriParser { _marker: PhantomData }
    }
}

impl<RDF> Default for IriParser<RDF> {
    fn default() -> Self {
        Self::new()
    }
}

impl<RDF> RDFNodeParse<RDF> for IriParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::IRI;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match rdf.get_focus() {
            Some(focus) => {
                let iri: RDF::IRI = RDF::term_as_iri(focus).map_err(|_| RDFError::ExpectedIRIError {
                    term: focus.to_string(),
                })?;
                Ok(iri)
            },
            None => Err(RDFError::NoFocusNodeError),
        }
    }
}

/// A parser that extracts the focus node as a literal.
#[derive(Debug, Clone)]
pub struct LiteralParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> Default for LiteralParser<RDF> {
    fn default() -> Self {
        Self::new()
    }
}

impl<RDF> LiteralParser<RDF> {
    pub fn new() -> Self {
        LiteralParser { _marker: PhantomData }
    }
}

impl<RDF> RDFNodeParse<RDF> for LiteralParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::Literal;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match rdf.get_focus() {
            Some(focus) => {
                let lit: RDF::Literal = RDF::term_as_literal(focus).map_err(|_| RDFError::ExpectedLiteralError {
                    term: focus.to_string(),
                })?;
                Ok(lit)
            },
            None => Err(RDFError::NoFocusNodeError),
        }
    }
}

/// A parser that extracts the focus node as a boolean.
#[derive(Debug, Clone)]
pub struct BooleanParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> Default for BooleanParser<RDF> {
    fn default() -> Self {
        Self::new()
    }
}

impl<RDF> BooleanParser<RDF> {
    pub fn new() -> Self {
        BooleanParser { _marker: PhantomData }
    }
}

impl<RDF> RDFNodeParse<RDF> for BooleanParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = bool;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let lit = LiteralParser::new().parse_focused(rdf)?;
        lit.to_bool()
            .ok_or_else(|| RDFError::ExpectedBooleanError { term: lit.to_string() })
    }
}

/// A parser that extracts the focus node as an IRI or Blanknode.
#[derive(Debug, Clone)]
pub struct IriOrBlankNodeParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> IriOrBlankNodeParser<RDF> {
    pub fn new() -> Self {
        IriOrBlankNodeParser { _marker: PhantomData }
    }
}

impl<RDF> Default for IriOrBlankNodeParser<RDF> {
    fn default() -> Self {
        Self::new()
    }
}

impl<RDF> RDFNodeParse<RDF> for IriOrBlankNodeParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = IriOrBlankNode;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match rdf.get_focus() {
            Some(focus) => {
                let subj: RDF::Subject =
                    <RDF::Term as TryInto<RDF::Subject>>::try_into(focus.clone()).map_err(|_| {
                        RDFError::ExpectedIriOrBlankNodeError {
                            term: focus.to_string(),
                            error: "Expected IRI or BlankNode".to_string(),
                        }
                    })?;

                let iri_or_bnode: IriOrBlankNode =
                    subj.clone()
                        .try_into()
                        .map_err(|_| RDFError::SubjectToIriOrBlankNodeError {
                            subject: subj.to_string(),
                        })?;

                Ok(iri_or_bnode)
            },
            None => Err(RDFError::NoFocusNodeError),
        }
    }
}

/// Parser that converts a term to an IriS.
#[derive(Debug, Clone)]
pub struct TermAsIri<RDF>
where
    RDF: FocusRDF,
{
    term: RDF::Term,
    _marker: PhantomData<RDF>,
}

impl<RDF> TermAsIri<RDF>
where
    RDF: FocusRDF,
{
    pub fn new(term: RDF::Term) -> Self {
        TermAsIri {
            term,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for TermAsIri<RDF>
where
    RDF: FocusRDF,
{
    type Output = IriS;

    fn parse_focused(&self, _rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let iri: RDF::IRI =
            <RDF::Term as TryInto<RDF::IRI>>::try_into(self.term.clone()).map_err(|_| RDFError::ExpectedIRIError {
                term: self.term.to_string(),
            })?;
        let iri_string = iri.as_str();
        Ok(iri!(iri_string))
    }
}
