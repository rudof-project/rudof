use crate::ast::ASTNodeShape;
use crate::rdf::parsers::components::components;
use crate::rdf::parsers::non_shape::message;
use crate::rdf::parsers::{property, severity, targets};
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
    NonEmptyValuesPropertyParser, ObjectParser, ParseBudget, SuccessParser,
};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use std::sync::Arc;

pub(crate) fn node_shape<RDF: FocusRDF>(budget: Arc<ParseBudget>) -> impl RDFNodeParse<RDF, Output = ASTNodeShape> {
    NonEmptyValuesPropertyParser::new(ShaclVocab::sh_path()).not().with(
        ObjectParser::new()
            .then(move |t| SuccessParser::new(ASTNodeShape::new(t)))
            .then(|ns| {
                severity()
                    .optional()
                    .flat_map(move |sev| Ok(ns.clone().with_severity(sev)))
            })
            .then(|ns| {
                message()
                    .optional()
                    .flat_map(move |msg| Ok(ns.clone().with_message(msg)))
            })
            .then(|ns| targets().flat_map(move |ts| Ok(ns.clone().with_targets(ts))))
            .then(move |ns| {
                let budget = budget.clone();
                property()
                    .flat_map(move |ps| Ok(ns.clone().with_property_shapes(ps)))
                    .then(move |ns_with_ps| {
                        components(budget.clone()).flat_map(move |cs| Ok(ns_with_ps.clone().with_components(cs)))
                    })
            }),
    )
}
