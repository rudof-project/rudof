use crate::ast::ASTComponent;
use crate::rdf::error::ShaclParserError;
use crate::types::NodeKind;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ValuesPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::Iri;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{FocusRDF, Rdf};

pub(crate) fn node_kind<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    ValuesPropertyParser::new(ShaclVocab::sh_node_kind().clone())
        .flat_map(|ns| {
            let nks: Vec<_> = ns
                .into_iter()
                .flat_map(|term| {
                    let nk = term_to_node_kind::<RDF>(term)?;
                    Ok::<ASTComponent, ShaclParserError>(ASTComponent::NodeKind(nk))
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
        ShaclVocab::SH_LITERAL => Ok(NodeKind::Lit),
        ShaclVocab::SH_BLANK_NODE => Ok(NodeKind::BNode),
        ShaclVocab::SH_BLANK_NODE_OR_IRI => Ok(NodeKind::BNodeOrIri),
        ShaclVocab::SH_BLANK_NODE_OR_LITERAL => Ok(NodeKind::BNodeOrLit),
        ShaclVocab::SH_IRI_OR_LITERAL => Ok(NodeKind::IriOrLit),
        _ => Err(ShaclParserError::UnknownNodeKind {
            term: term_name
        }),
    }
}