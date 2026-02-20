use super::super::parse_components_for_iri;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ListParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::literal::Lang;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn language_in<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    parse_components_for_iri(
        ShaclVocab::sh_language_in().clone(),
        ListParser::new().flat_map(cnv_language_in_list::<RDF>),
    )
}

fn cnv_language_in_list<R: FocusRDF>(terms: Vec<R::Term>) -> Result<Component, RDFError> {
    let langs: Vec<Lang> = terms.iter().flat_map(R::term_as_lang).collect();
    Ok(Component::LanguageIn(langs))
}
