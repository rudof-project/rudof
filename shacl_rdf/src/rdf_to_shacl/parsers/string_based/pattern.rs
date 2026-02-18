use rdf::rdf_core::FocusRDF;
use rdf::rdf_core::parser::rdf_node_parser::constructors::{SingleStringPropertyParser, StringsPropertyParser};
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn pattern<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    SingleStringPropertyParser::new(ShaclVocab::sh_flags().clone())
        .optional()
        .then(move |maybe_flags| {
            StringsPropertyParser::new(ShaclVocab::sh_pattern().clone()).flat_map(move |strs| match strs.len() {
                0 => Ok(Vec::new()),
                1 => {
                    let pattern = strs.first().unwrap().clone();
                    let flags = maybe_flags.clone();
                    Ok(vec![Component::Pattern { pattern, flags }])
                },
                _ => todo!(), // Error ...
            })
        })
}
