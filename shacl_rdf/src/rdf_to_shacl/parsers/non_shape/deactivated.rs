use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::BoolsPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use shacl_ast::NodeExpr;
use shacl_ast::component::Component;

pub(crate) fn deactivated<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component<RDF>>> {
    BoolsPropertyParser::new(ShaclVocab::sh_deactivated().clone()).map(|ns| {
        ns.into_iter()
            .map(|b| Component::Deactivated(NodeExpr::Literal(ConcreteLiteral::BooleanLiteral(b))))
            .collect::<Vec<_>>()
    })
}
