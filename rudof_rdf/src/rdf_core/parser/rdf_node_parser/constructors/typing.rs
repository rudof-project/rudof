use crate::rdf_core::vocabs::RdfVocab;
use crate::rdf_core::{
    FocusRDF, RDFError,
    parser::rdf_node_parser::{
        RDFNodeParse,
        constructors::{SingleValuePropertyParser, SubjectsWithValuePropertyParser},
    },
    term::Iri,
};
use iri_s::IriS;
use std::{collections::HashMap, marker::PhantomData};

/// Parser that extracts the `rdf:type` value from the focus node.
#[derive(Debug, Clone)]
pub struct TypeParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> Default for TypeParser<RDF> {
    fn default() -> Self {
        Self::new()
    }
}

impl<RDF> TypeParser<RDF> {
    pub fn new() -> Self {
        Self { _marker: PhantomData }
    }
}

impl<RDF> RDFNodeParse<RDF> for TypeParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::Term;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        SingleValuePropertyParser::new(RdfVocab::rdf_type().clone()).parse_focused(rdf)
    }
}

/// Parser that finds and focuses on a single instance of the specified type.
///
/// Searches for nodes with `rdf:type` matching the expected IRI, expecting exactly one match.
/// If found, the focus is moved to that node and it is returned.
#[derive(Debug, Clone)]
pub struct SingleInstanceParser<RDF> {
    expected: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> SingleInstanceParser<RDF> {
    pub fn new(expected: IriS) -> Self {
        Self {
            expected,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for SingleInstanceParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::Subject;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let instances = InstancesParser::new(self.expected.clone()).parse_focused(rdf)?;

        let mut iter = instances.into_iter();
        match iter.next() {
            Some(instance) => {
                if iter.next().is_some() {
                    Err(RDFError::MoreThanOneInstanceError {
                        type_iri: self.expected.to_string(),
                    })
                } else {
                    Ok(instance)
                }
            },
            None => Err(RDFError::FailedInstancesOfError {
                object: self.expected.to_string(),
            }),
        }
    }
}

/// Parser that dispatches to different parsers based on the node's `rdf:type`.
#[derive(Debug, Clone)]
pub struct TypeDispatchParser<P> {
    type_map: HashMap<IriS, P>,
    default: P,
}

impl<P> TypeDispatchParser<P> {
    pub fn new<M>(mapping: M, default: P) -> Self
    where
        M: IntoIterator<Item = (IriS, P)>,
    {
        Self {
            type_map: mapping.into_iter().collect(),
            default,
        }
    }
}

impl<RDF, P, A> RDFNodeParse<RDF> for TypeDispatchParser<P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = A;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let type_term = TypeParser::<RDF>::new().parse_focused(rdf)?;
        let type_iri: RDF::IRI =
            <RDF::Term as TryInto<RDF::IRI>>::try_into(type_term.clone()).map_err(|_| RDFError::ExpectedIRIError {
                term: type_term.to_string(),
            })?;

        let type_str = type_iri.as_str();
        let type_iri_s = iri_s::iri!(type_str);

        match self.type_map.get(&type_iri_s) {
            Some(parser) => parser.parse_focused(rdf),
            None => self.default.parse_focused(rdf),
        }
    }
}

/// Parser that finds all instances of a specific type in the RDF graph.
#[derive(Debug, Clone)]
pub struct InstancesParser<RDF> {
    expected_type: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> InstancesParser<RDF> {
    pub fn new(expected_type: IriS) -> Self {
        Self {
            expected_type,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for InstancesParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<RDF::Subject>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let type_term: RDF::Term = self.expected_type.clone().into();
        let pred: RDF::IRI = RdfVocab::rdf_type().clone().into();

        SubjectsWithValuePropertyParser::new(pred, type_term).parse_focused(rdf)
    }
}
