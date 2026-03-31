use crate::ast::ASTComponent;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{SingleStringPropertyParser, StringsPropertyParser};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;

pub(crate) fn pattern<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    SingleStringPropertyParser::new(ShaclVocab::sh_flags())
        .optional()
        .then(move |maybe_flags| {
            StringsPropertyParser::new(ShaclVocab::sh_pattern()).flat_map(move |strs| match strs.len() {
                0 => Ok(Vec::new()),
                1 => {
                    let pattern = strs.first().unwrap().clone();
                    let flags = maybe_flags.clone();
                    Ok(vec![ASTComponent::Pattern { pattern, flags }])
                },
                _ => todo!(), // Error...
            })
        })
}
