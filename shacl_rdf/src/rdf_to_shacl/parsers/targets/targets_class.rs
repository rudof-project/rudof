use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IrisPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use shacl_ast::target::Target;

pub(crate) fn targets_class<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target<RDF>>> {
    IrisPropertyParser::new(ShaclVocab::sh_target_class().clone()).flat_map(move |ts| {
        let result = ts.into_iter().map(|iri| Target::Class(Object::Iri(iri))).collect();
        Ok(result)
    })
}
