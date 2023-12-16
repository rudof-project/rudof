use iri_s::IriS;

use super::rdf_parser_error::RDFParseError;
use crate::{Object, Vocab, RDF_NIL, RDF_TYPE, SRDF};
use std::{collections::VecDeque, error::Error, fmt::Display, marker::PhantomData};

/// The following code is an attempt to define parser combinators where the input is an RDF graph instead of a sequence of characters
/// Some parts of this code are inspired by [Combine]()
pub trait RDFParse<RDF: SRDF> {
    /// The type which is returned if the parser is successful.    
    type Output;

    fn parse(&mut self, rdf: RDF) -> Result<Self::Output, RDF::Err>;
}

pub trait RDFNodeParse<RDF: SRDF> {
    type Output;

    fn parse(&mut self, node: &RDF::Subject, rdf: &RDF) -> Result<Self::Output, RDFParseError> {
        self.parse_impl(node, rdf).into()
    }

    fn parse_impl(
        &mut self,
        node: &RDF::Subject,
        rdf: &RDF,
    ) -> ParseResult<Self::Output, RDFParseError>;
}

#[derive(Copy, Clone)]
pub struct Map<P, F>(P, F);
impl<RDF, A, B, P, F> RDFNodeParse<RDF> for Map<P, F>
where
    RDF: SRDF,
    P: RDFNodeParse<RDF, Output = A>,
    F: FnMut(A) -> B,
{
    type Output = B;

    fn parse_impl(
        &mut self,
        node: &RDF::Subject,
        rdf: &RDF,
    ) -> ParseResult<Self::Output, RDFParseError> {
        match self.0.parse_impl(node, rdf) {
            ParseResult::CommitOk(a) => ParseResult::CommitOk((self.1)(a)),
            ParseResult::PeekOk(a) => ParseResult::PeekOk((self.1)(a)),
            ParseResult::CommitErr(e) => ParseResult::CommitErr(e),
            ParseResult::PeekErr(e) => ParseResult::PeekErr(e),
        }
    }
}

pub fn map<RDF, P, F, B>(p: P, f: F) -> Map<P, F>
where
    RDF: SRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> B,
{
    Map(p, f)
}

pub fn parse_rdf_nil<RDF>() -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: SRDF,
{
    satisfy(
        |node: &RDF::Subject| match RDF::subject_as_iri(node) {
            Some(iri) => {
                let iri_s = RDF::iri2iri_s(&iri);
                iri_s.as_str() == RDF_NIL
            }
            None => false,
        },
        "rdf_nil",
    )
}

pub fn satisfy<RDF, P>(predicate: P, predicate_name: &str) -> Satisfy<RDF, P>
where
    RDF: SRDF,
    P: FnMut(&RDF::Subject) -> bool,
{
    Satisfy {
        predicate,
        predicate_name: predicate_name.to_string(),
        _marker: PhantomData,
    }
}

#[derive(Clone)]
pub struct Satisfy<RDF, P> {
    predicate: P,
    predicate_name: String,
    _marker: PhantomData<RDF>,
}

impl<RDF, P> RDFNodeParse<RDF> for Satisfy<RDF, P>
where
    RDF: SRDF,
    P: FnMut(&RDF::Subject) -> bool,
{
    type Output = ();

    fn parse_impl(&mut self, node: &RDF::Subject, rdf: &RDF) -> ParseResult<(), RDFParseError> {
        if (self.predicate)(node) {
            ParseResult::CommitOk(())
        } else {
            ParseResult::PeekErr(RDFParseError::NodeDoesntSatisfyCondition {
                condition_name: self.predicate_name.clone(),
                node: format!("{node}"),
            })
        }
    }
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum ParseResult<T, E> {
    /// The parser has succeeded and has committed to this parse.
    /// If a parser after this fails, other parser alternatives will not be attempted (`CommitErr` will be returned)
    CommitOk(T),
    /// The parser has succeeded and has not committed to this parse.
    /// If a parser after this fails, other parser alternatives will be attempted (`PeekErr` will be returned)
    PeekOk(T),
    /// The parser failed other parse alternatives will not be attempted.
    CommitErr(E),
    /// The parser failed but other parse alternatives may be attempted.
    PeekErr(E),
}

impl<T, E> Into<Result<T, E>> for ParseResult<T, E> {
    fn into(self) -> Result<T, E> {
        match self {
            ParseResult::CommitOk(t) => Ok(t),
            ParseResult::PeekOk(t) => Ok(t),
            ParseResult::CommitErr(e) => Err(e),
            ParseResult::PeekErr(e) => Err(e),
        }
    }
}

pub struct RDFParser<RDF>
where
    RDF: SRDF,
{
    rdf: RDF,
}

impl<RDF> RDFParser<RDF>
where
    RDF: SRDF,
{
    pub fn new(rdf: RDF) -> RDFParser<RDF> {
        RDFParser { rdf }
    }

    pub fn iri_unchecked(str: &str) -> RDF::IRI {
        RDF::iri_s2iri(&IriS::new_unchecked(str))
    }

    pub fn term_iri_unchecked(str: &str) -> RDF::Term {
        RDF::iri_as_term(Self::iri_unchecked(str))
    }

    #[inline]
    fn rdf_type() -> RDF::IRI {
        RDF::iri_s2iri(&Vocab::rdf_type())
    }

    #[inline]
    fn rdf_first() -> RDF::IRI {
        RDF::iri_s2iri(&Vocab::rdf_first())
    }

    #[inline]
    fn rdf_rest() -> RDF::IRI {
        RDF::iri_s2iri(&Vocab::rdf_rest())
    }

    #[inline]
    fn rdf_nil() -> RDF::IRI {
        RDF::iri_s2iri(&Vocab::rdf_nil())
    }

    pub fn instances_of(
        &self,
        object: &RDF::Term,
    ) -> Result<impl Iterator<Item = RDF::Subject>, RDFParseError> {
        let values = self
            .rdf
            .subjects_with_predicate_object(&Self::rdf_type(), &object)
            .map_err(|e| RDFParseError::SRDFError { err: e.to_string() })?;
        Ok(values.into_iter())
    }

    pub fn instance_of(&self, object: &RDF::Term) -> Result<RDF::Subject, RDFParseError> {
        let mut values = self.instances_of(&object)?;
        if let Some(value1) = values.next() {
            if let Some(value2) = values.next() {
                Err(RDFParseError::MoreThanOneInstanceOf {
                    object: format!("{object}"),
                    value1: format!("{value1}"),
                    value2: format!("{value2}"),
                })
            } else {
                // Only one value
                Ok(value1)
            }
        } else {
            Err(RDFParseError::NoInstancesOf {
                object: format!("{object}"),
            })
        }
    }

    pub fn predicate_values(
        &self,
        node: &RDF::Subject,
        pred: &RDF::IRI,
    ) -> Result<impl Iterator<Item = RDF::Term>, RDFParseError>
    where
        RDF: SRDF,
    {
        let values = self
            .rdf
            .get_objects_for_subject_predicate(&node, &pred)
            .map_err(|e| RDFParseError::SRDFError {
                err: format!("{e}"),
            })?;
        Ok(values.into_iter())
    }

    pub fn predicate_value(
        &self,
        node: &RDF::Subject,
        pred: &RDF::IRI,
    ) -> Result<RDF::Term, RDFParseError>
    where
        RDF: SRDF,
    {
        let mut values = self.predicate_values(&node, &pred)?;
        if let Some(value1) = values.next() {
            if let Some(value2) = values.next() {
                Err(RDFParseError::MoreThanOneValuePredicate {
                    node: format!("{node}"),
                    pred: format!("{pred}"),
                    value1: format!("{value1:?}"),
                    value2: format!("{value2:?}"),
                })
            } else {
                Ok(value1)
            }
        } else {
            /* Debug in case no value found */
            println!("Not found value for property {pred}");
            let preds = self.rdf.get_predicates_for_subject(&node);
            for pred in preds.iter() {
                println!("Other predicates: {pred:?}");
            }
            /* end debug */

            Err(RDFParseError::NoValuesPredicate {
                node: format!("{node}"),
                pred: format!("{pred}"),
            })
        }
    }

    pub fn get_rdf_type(&self, node: &RDF::Subject) -> Result<RDF::Term, RDFParseError> {
        let value = self.predicate_value(node, &Self::rdf_type())?;
        Ok(value)
    }

    pub fn term_as_iri(term: &RDF::Term) -> Result<IriS, RDFParseError> {
        let obj = RDF::term_as_object(term);
        match obj {
            Object::Iri { iri } => Ok(iri),
            Object::BlankNode(bnode) => Err(RDFParseError::ExpectedIRIFoundBNode { bnode }),
            Object::Literal(lit) => Err(RDFParseError::ExpectedIRIFoundLiteral { lit }),
        }
    }

    pub fn term_as_subject(term: &RDF::Term) -> Result<RDF::Subject, RDFParseError> {
        match RDF::term_as_subject(&term) {
            None => Err(RDFParseError::ExpectedSubject {
                node: format!("{term}"),
            }),
            Some(subj) => Ok(subj),
        }
    }

    pub fn parse_list_for_predicate(
        &self,
        node: &RDF::Subject,
        pred: &RDF::IRI,
    ) -> Result<Vec<RDF::Term>, RDFParseError> {
        let list_node = self.predicate_value(&node, &pred)?;
        let list_node_subj = Self::term_as_subject(&list_node)?;
        let values = self.parse_list(&list_node_subj, vec![list_node])?;
        Ok(values)
    }

    fn parse_list(
        &self,
        list_node: &RDF::Subject,
        mut visited: Vec<RDF::Term>,
    ) -> Result<Vec<RDF::Term>, RDFParseError> {
        if Self::node_is_rdf_nil(&list_node) {
            Ok(Vec::new())
        } else {
            let value = self.predicate_value(&list_node, &Self::rdf_first())?;
            let rest = self.predicate_value(&list_node, &Self::rdf_rest())?;
            if visited.contains(&&rest) {
                Err(RDFParseError::RecursiveRDFList {
                    node: format!("{rest}"),
                })
            } else {
                visited.push(rest.clone());
                let rest_subj = Self::term_as_subject(&rest)?;
                let mut rest = Vec::new();
                rest.push(value);
                rest.extend(self.parse_list(&rest_subj, visited)?);
                Ok(rest)
            }
        }
    }

    fn node_is_rdf_nil(node: &RDF::Subject) -> bool {
        if let Some(iri) = RDF::subject_as_iri(&node) {
            iri == Self::rdf_nil()
        } else {
            false
        }
    }
}
