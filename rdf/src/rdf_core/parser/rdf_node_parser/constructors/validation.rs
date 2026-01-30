use crate::rdf_core::{
    FocusRDF, RDFError, parser::rdf_node_parser::{RDFNodeParse, constructors::TypeParser}, term::Iri, vocab::rdf_nil
};
use iri_s::IriS;
use std::marker::PhantomData;

/// Parser that validates the focus node against a custom predicate.
#[derive(Clone)]
pub struct SatisfyParser<RDF, P> {
    predicate: P,
    condition_name: String,
    _marker: PhantomData<RDF>,
}

impl<RDF, P> SatisfyParser<RDF, P> 
where 
    RDF: FocusRDF,
    P: Fn(&RDF::Term) -> bool,
{
    pub fn new(predicate: P, condition_name: impl Into<String>) -> Self {
        Self {
            predicate,
            condition_name: condition_name.into(),
            _marker: PhantomData,
        }
    }
}

impl<RDF, P> RDFNodeParse<RDF> for SatisfyParser<RDF, P>
where
    RDF: FocusRDF,
    P: Fn(&RDF::Term) -> bool,
{
    type Output = ();

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = rdf.get_focus()
            .ok_or(RDFError::NoFocusNodeError)?;
            
        if (self.predicate)(&focus) {
            Ok(())
        } else {
            Err(RDFError::NodeDoesntSatisfyConditionError {
                condition_name: self.condition_name.clone(),
                node: focus.to_string(),
            })
        }
    }
}

/// Parser that validates the focus node is a specific IRI.
#[derive(Debug, Clone)]
pub struct IsIriParser<RDF> {
    expected: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> IsIriParser<RDF> {
    pub fn new(expected: IriS) -> Self {
        Self {
            expected,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for IsIriParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = ();

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = rdf.get_focus()
            .ok_or(RDFError::NoFocusNodeError)?;

        let iri: RDF::IRI = match <RDF::Term as TryInto<RDF::IRI>>::try_into(focus.clone()) {
            Ok(iri) => iri,
            Err(_) => return Err(RDFError::ExpectedIRIError {
                term: focus.to_string(),
            }),
        };

        if iri.as_str() == self.expected.as_str() {
            Ok(())
        } else {
            Err(RDFError::NodeDoesntSatisfyConditionError {
                condition_name: format!("Is {}", self.expected),
                node: focus.to_string(),
            })
        }
    }
}

/// Parser that validates the current focus node is `rdf:nil`.
/// 
/// `rdf:nil` represents the empty list in RDF. This parser succeeds with `()`
/// if the focus node is `rdf:nil`, or fails otherwise.
#[derive(Debug, Clone)]
pub struct NilParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl <RDF> NilParser<RDF> {
    pub fn new() -> Self {
        NilParser {
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for NilParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = ();

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = rdf.get_focus()
            .ok_or(RDFError::NoFocusNodeError)?;
        
        let is_nil = match TryInto::<RDF::IRI>::try_into(focus.clone()) {
            Ok(iri) => iri.as_str() == rdf_nil().as_str(),
            Err(_) => false,
        };
            
        if is_nil {
            Ok(())
        } else {
            Err(RDFError::ExpectedNilError {
                term: focus.to_string(),
            })
        }
    }
}

/// Parser that validates the focus node has a specific `rdf:type`.
#[derive(Debug, Clone)]
pub struct HasTypeParser<RDF> {
    expected: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> HasTypeParser<RDF> {
    pub fn new(expected: IriS) -> Self {
        Self {
            expected,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for HasTypeParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = ();

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let actual_type = TypeParser::<RDF>::new().parse_focused(rdf)?;
        let expected_term: RDF::Term = self.expected.clone().into();
        
        if actual_type == expected_term {
            Ok(())
        } else {
            Err(RDFError::TypeMismatchError {
                expected: self.expected.to_string(),
                actual: actual_type.to_string(),
            })
        }
    }
}

/// Parser that always succeeds with a predefined value.
/// 
/// Useful for introducing constants into parser chains or providing
/// default values without querying the RDF graph.
#[derive(Debug, Clone)]
pub struct SuccessParser<A> {
    value: A,
}

impl<A> SuccessParser<A> {
    pub fn new(value: A) -> Self {
        Self { value }
    }
}

impl<RDF, A> RDFNodeParse<RDF> for SuccessParser<A>
where
    RDF: FocusRDF,
    A: Clone,
{
    type Output = A;

    fn parse_focused(&self, _rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        Ok(self.value.clone())
    }
}

/// Parser that always fails with a specific error message.
/// 
/// Useful for representing unrecoverable errors or enforcing 
/// invariants in parser composition.
#[derive(Debug, Clone)]
pub struct FailureParser<A> {
    msg: String,
    _marker: PhantomData<A>,
}

impl<A> FailureParser<A> {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            _marker: PhantomData,
        }
    }
}

impl<RDF, A> RDFNodeParse<RDF> for FailureParser<A>
where
    RDF: FocusRDF,
{
    type Output = A;

    fn parse_focused(&self, _rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        Err(RDFError::ParseFailError {
            msg: self.msg.clone(),
        })
    }
}

/// Parser that validates a value against a predicate.
#[derive(Debug, Clone)]
pub struct CondParser<A, P> {
    value: A,
    predicate: P,
    fail_msg: String,
}

impl<A, P> CondParser<A, P>
where
    P: Fn(&A) -> bool,
{
    pub fn new(value: A, predicate: P, fail_msg: impl Into<String>) -> Self {
        Self {
            value,
            predicate,
            fail_msg: fail_msg.into(),
        }
    }
}

impl<RDF, A, P> RDFNodeParse<RDF> for CondParser<A, P>
where
    RDF: FocusRDF,
    P: Fn(&A) -> bool,
{
    type Output = ();

    fn parse_focused(&self, _rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        if (self.predicate)(&self.value) {
            Ok(())
        } else {
            Err(RDFError::FailedConditionalError {
                msg: self.fail_msg.clone(),
            })
        }
    }
}
