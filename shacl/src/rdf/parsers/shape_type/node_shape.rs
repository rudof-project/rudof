use crate::ast::ASTNodeShape;
use crate::rdf::parsers::components::components;
use crate::rdf::parsers::{property, severity, targets};
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
    NonEmptyValuesPropertyParser, ObjectParser, SuccessParser,
};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;

pub(crate) fn node_shape<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = ASTNodeShape> {
    NonEmptyValuesPropertyParser::new(ShaclVocab::sh_path().clone())
        .not()
        .with(
            ObjectParser::new()
                .then(move |t| SuccessParser::new(ASTNodeShape::new(t)))
                .then(|ns| {
                    severity()
                        .optional()
                        .flat_map(move |sev| Ok(ns.clone().with_severity(sev)))
                })
                .then(|ns| targets().flat_map(move |ts| Ok(ns.clone().with_targets(ts))))
                .then(|ns| {
                    property()
                        .flat_map(move |ps| Ok(ns.clone().with_property_shapes(ps)))
                        .then(|ns_with_ps| components().flat_map(move |cs| Ok(ns_with_ps.clone().with_components(cs))))
                }),
        )
}
