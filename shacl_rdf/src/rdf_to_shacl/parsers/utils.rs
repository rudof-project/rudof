use iri_s::IriS;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::term::{Iri, Object, Term};
use rudof_rdf::rdf_core::{FocusRDF, RDFError, Rdf};
use shacl_ast::component::Component;
use shacl_ast::value::Value;

pub(super) fn parse_components_for_iri<RDF, P>(
    iri: IriS,
    component_parser: P,
) -> impl RDFNodeParse<RDF, Output = Vec<Component<RDF>>>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = Component<RDF>>,
{
    component_parser.map_property(iri)
}

pub(super) fn terms_as_nodes<RDF: Rdf>(terms: Vec<RDF::Term>) -> Result<Vec<Object>, RDFError> {
    terms
        .into_iter()
        .map(|t| {
            let term_name = t.to_string();
            RDF::term_as_object(&t).map_err(|_| RDFError::FailedTermToRDFNodeError { term: term_name })
        })
        .collect()
}

pub(super) fn term_to_value<RDF>(term: &RDF::Term, msg: &str) -> Result<Value, RDFError>
where
    RDF: Rdf,
{
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
        let slit: ConcreteLiteral = literal.clone().try_into().map_err(|_e| RDFError::LiteralAsSLiteral {
            literal: literal.to_string(),
        })?;
        Ok(Value::Literal(slit))
    } else {
        println!("Unexpected code in term_to_value: {term}: {msg}");
        todo!()
    }
}
