use crate::ast::ASTComponent;
use crate::types::Value;
use prefixmap::IriRef;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
    ApplyRdfParser, ListParser, MIN_PARSE_STEPS, PARSE_STEPS_PER_TRIPLE, ParseBudget,
};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::term::{Iri, Object, Term};
use rudof_rdf::rdf_core::{FocusRDF, RDFError, Rdf};
use std::fmt::Display;
use std::sync::Arc;

pub(crate) fn parse_components_for_iri<RDF, P>(
    iri: IriS,
    component_parser: P,
) -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = ASTComponent>,
{
    component_parser.map_property(iri)
}

pub(crate) fn terms_as_nodes<RDF: Rdf>(terms: Vec<RDF::Term>) -> Result<Vec<Object>, RDFError> {
    terms
        .into_iter()
        .map(|t| {
            let term_name = t.to_string();
            RDF::term_as_object(&t).map_err(|_| RDFError::FailedTermToRDFNodeError { term: term_name })
        })
        .collect()
}

pub(crate) fn term_to_value<RDF: Rdf>(term: &RDF::Term, msg: &str) -> Result<Value, RDFError> {
    if term.is_blank_node() {
        Err(RDFError::ExpectedIriOrBlankNodeError {
            term: term.to_string(),
            error: msg.to_string(),
        })
    } else if let Ok(iri) = RDF::term_as_iri(term) {
        let iri: RDF::IRI = iri;
        let iri_string = iri.as_str();
        let iri_s = IriS::new_unchecked(iri_string);
        Ok(Value::Iri(IriRef::Iri(iri_s)))
    } else if let Ok(literal) = RDF::term_as_literal(term) {
        let literal: RDF::Literal = literal;
        let slit: ConcreteLiteral = literal.clone().try_into().map_err(|_| RDFError::LiteralAsSLiteral {
            literal: literal.to_string(),
        })?;
        Ok(Value::Literal(slit))
    } else {
        Err(RDFError::DefaultError {
            msg: format!("Unexpected term in {msg}: {term}"),
        })
    }
}

/// Takes `n` of the `steps` left, for what parsing `term` has built. If fewer are left it takes them all,
/// so that [`ParseBudget::spend`] counts the graph and runs the parse again.
pub(crate) fn take_steps(term: &impl Display, steps: &mut usize, n: usize) -> Result<(), RDFError> {
    let Some(left) = steps.checked_sub(n) else {
        *steps = 0;
        return Err(RDFError::DefaultError {
            msg: format!(
                "Parsing {term} takes more than {MIN_PARSE_STEPS} steps plus {PARSE_STEPS_PER_TRIPLE} per triple of the graph"
            ),
        });
    };
    *steps = left;
    Ok(())
}

/// Parses the RDF list at the focus node, taking a step from `budget` for each member once the free
/// ones run out, so a list shared by many shapes cannot multiply the work.
pub(crate) fn list_with_budget<RDF: FocusRDF>(
    budget: Arc<ParseBudget>,
) -> impl RDFNodeParse<RDF, Output = Vec<RDF::Term>> {
    ApplyRdfParser::new(move |rdf: &mut RDF| {
        let head = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
        let members = ListParser::new().parse_focused(rdf)?;
        let charged = budget.take_free_members(members.len());
        budget.spend(rdf, |_, steps| take_steps(&head, steps, charged))?;
        Ok(members)
    })
}
