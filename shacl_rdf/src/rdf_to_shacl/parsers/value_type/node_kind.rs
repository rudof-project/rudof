use crate::error::ShaclParserError;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ValuesPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::Iri;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{FocusRDF, Rdf};
use shacl_ast::component::Component;
use shacl_ast::node_kind::NodeKind;

pub(crate) fn node_kind<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    ValuesPropertyParser::new(ShaclVocab::sh_node_kind().clone()).flat_map(|ns| {
        let nks: Vec<_> = ns
            .into_iter()
            .flat_map(|term| {
                let nk = term_to_node_kind::<RDF>(term)?;
                Ok::<Component, ShaclParserError>(Component::NodeKind(nk))
            })
            .collect();
        Ok(nks)
    })
}

fn term_to_node_kind<RDF: Rdf>(term: RDF::Term) -> Result<NodeKind, ShaclParserError> {
    let term_name = term.to_string();
    let result_iri: Result<RDF::IRI, ShaclParserError> =
        <RDF::Term as TryInto<RDF::IRI>>::try_into(term).map_err(|_| ShaclParserError::ExpectedNodeKind {
            term: term_name.to_string(),
        });
    let iri = result_iri?;
    match iri.as_str() {
        ShaclVocab::SH_IRI => Ok(NodeKind::Iri),
        ShaclVocab::SH_LITERAL => Ok(NodeKind::Literal),
        ShaclVocab::SH_BLANK_NODE => Ok(NodeKind::BlankNode),
        ShaclVocab::SH_BLANK_NODE_OR_IRI => Ok(NodeKind::BlankNodeOrIri),
        ShaclVocab::SH_BLANK_NODE_OR_LITERAL => Ok(NodeKind::BlankNodeOrLiteral),
        ShaclVocab::SH_IRI_OR_LITERAL => Ok(NodeKind::IRIOrLiteral),
        _ => Err(ShaclParserError::UnknownNodeKind { term: term_name }),
    }
}
