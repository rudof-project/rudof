use crate::rdf_to_shacl::parsers::targets::targets;
use crate::rdf_to_shacl::parsers::{components, path, property, reifier_shape, severity};
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
    FocusParser, HasTypeParser, ObjectParser, SetFocusParser, SuccessParser,
};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use shacl_ast::property_shape::PropertyShape;

pub(crate) fn property_shape<RDF: FocusRDF + 'static>() -> impl RDFNodeParse<RDF, Output = PropertyShape<RDF>> {
    FocusParser::new().then(move |focus: RDF::Term| {
        HasTypeParser::new(ShaclVocab::sh_property_shape().clone())
            .optional()
            .with(
                ObjectParser::new()
                    .and(path())
                    .then(move |(id, path)| SuccessParser::new(PropertyShape::new(id, path))),
            )
            .then(move |ps| SetFocusParser::new(focus.clone()).with(SuccessParser::new(ps)))
            .then(|ps| {
                severity()
                    .optional()
                    .flat_map(move |sev| Ok(ps.clone().with_severity(sev)))
            })
            .then(|ps| reifier_shape().flat_map(move |r_shape| Ok(ps.clone().with_reifier_shape(r_shape))))
            .then(|ps| targets().flat_map(move |ts| Ok(ps.clone().with_targets(ts))))
            .then(|ps| {
                property()
                    .flat_map(move |prop_shapes| Ok(ps.clone().with_property_shapes(prop_shapes)))
                    .then(move |ps_with_props| property_shape_components(ps_with_props))
            })
    })
}

fn property_shape_components<RDF: FocusRDF>(
    ps: PropertyShape<RDF>,
) -> impl RDFNodeParse<RDF, Output = PropertyShape<RDF>> {
    components().flat_map(move |cs| Ok(ps.clone().with_components(cs)))
}
