use rdf::rdf_core::FocusRDF;
use rdf::rdf_core::parser::rdf_node_parser::constructors::{SingleBoolPropertyParser, ValuesPropertyParser};
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::ShaclVocab;
use shacl_ast::reifier_info::ReifierInfo;

pub(crate) fn reifier_shape<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Option<ReifierInfo>> {
    ValuesPropertyParser::new(ShaclVocab::sh_reifier_shape().clone()).then(move |vs| {
        SingleBoolPropertyParser::new(ShaclVocab::sh_reification_required().clone())
            .optional()
            .map(move |requires_reifier| {
                let reifier_shape = vs.iter().filter_map(|v| RDF::term_as_object(v).ok()).collect();
                if vs.is_empty() {
                    None
                } else {
                    Some(ReifierInfo::new(requires_reifier.unwrap_or(false), reifier_shape))
                }
            })
    })
}
