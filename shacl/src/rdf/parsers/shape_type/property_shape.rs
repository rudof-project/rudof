use crate::ast::ASTPropertyShape;
use crate::rdf::parsers::components::components;
use crate::rdf::parsers::{path, property, reifier_shape, severity, targets};
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
    FocusParser, HasTypeParser, ObjectParser, SetFocusParser, SuccessParser,
};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use crate::rdf::parsers::non_shape::message;

pub(crate) fn property_shape<RDF: FocusRDF + 'static>() -> impl RDFNodeParse<RDF, Output = ASTPropertyShape> {
    FocusParser::new().then(move |focus: RDF::Term| {
        HasTypeParser::new(ShaclVocab::sh_property_shape())
            .optional()
            .with(
                ObjectParser::new()
                    .and(path())
                    .then(move |(id, path)| SuccessParser::new(ASTPropertyShape::new(id, path))),
            )
            .then(move |ps| SetFocusParser::new(focus.clone()).with(SuccessParser::new(ps)))
            .then(|ps| {
                severity()
                    .optional()
                    .flat_map(move |sev| Ok(ps.clone().with_severity(sev)))
            })
            .then(|ps| message()
                .optional()
                .flat_map(move |msg| Ok(ps.clone().with_message(msg))))
            .then(|ps| reifier_shape().flat_map(move |r_shape| Ok(ps.clone().with_reifier_shape(r_shape))))
            .then(|ps| targets().flat_map(move |ts| Ok(ps.clone().with_targets(ts))))
            .then(|ps| {
                property()
                    .flat_map(move |prop_shapes| Ok(ps.clone().with_property_shapes(prop_shapes)))
                    .then(move |ps_with_props| {
                        components().flat_map(move |cs| Ok(ps_with_props.clone().with_components(cs)))
                    })
            })
    })
}
