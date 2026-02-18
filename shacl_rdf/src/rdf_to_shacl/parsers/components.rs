use crate::rdf_to_shacl::parsers::{
    and, class, closed, datatype, deactivated, disjoint, equals, has_value, in_component, language_in, less_than,
    less_than_or_equals, max_count, max_exclusive, max_inclusive, max_length, min_count, min_exclusive, min_inclusive,
    min_length, node, node_kind, not, or, pattern, qualified_value_shape, unique_lang, xone,
};
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::component::Component;

pub(crate) fn components<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    let parsers: Vec<Box<dyn RDFNodeParse<RDF, Output = Vec<Component>>>> = vec![
        // Value type
        Box::new(node_kind()),
        Box::new(datatype()),
        // Cardinality
        Box::new(min_count()),
        Box::new(max_count()),
        // Value range
        Box::new(min_inclusive()),
        Box::new(min_exclusive()),
        Box::new(max_inclusive()),
        Box::new(max_exclusive()),
        // String based
        Box::new(min_length()),
        Box::new(max_length()),
        Box::new(pattern()),
        // single_line(), // TODO - SHACL 1.2
        Box::new(language_in()),
        Box::new(unique_lang()),
        // List constraint components - SHACL 1.2
        // member_shape(), // TODO - SHACL 1.2
        // min_list_length(), // TODO - SHACL 1.2
        // max_list_length(), // TODO - SHACL 1.2
        // unique_members(), // TODO - SHACL 1.2
        // Property pair
        Box::new(equals()),
        Box::new(disjoint()),
        Box::new(less_than()),
        Box::new(less_than_or_equals()),
        // Logical constraint components
        Box::new(not()),
        Box::new(and()),
        Box::new(or()),
        Box::new(xone()),
        // Shape based constraint components
        Box::new(node()),
        // property is handled differently
        Box::new(qualified_value_shape()),
        // Other
        Box::new(closed()),
        Box::new(has_value()),
        Box::new(in_component()),
        // SPARQL based constraints and SPARQL based constraint components
        // TODO

        // TODO: deactivated is not a shape component...move this code elsewhere?
        Box::new(deactivated()),
    ];

    Box::new(class()).combine_many(parsers)
}
