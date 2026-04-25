use crate::ast::ASTComponent;
use crate::rdf::parsers::utils::parse_components_for_iri;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ListParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::literal::Lang;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn language_in<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    parse_components_for_iri(
        ShaclVocab::sh_language_in(),
        ListParser::new().flat_map(cnv_language_in_list::<RDF>),
    )
}

fn cnv_language_in_list<RDF: FocusRDF>(terms: Vec<RDF::Term>) -> Result<ASTComponent, RDFError> {
    let langs: Vec<Lang> = terms.iter().flat_map(RDF::term_as_lang).collect();
    Ok(ASTComponent::LanguageIn(langs))
}
