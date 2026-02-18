mod targets_class;
mod targets_implicit_class;
mod targets_node;
mod targets_objects_of;
mod targets_subjects_of;

use rdf::rdf_core::FocusRDF;
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::target::Target;

pub(crate) use targets_class::targets_class;
pub(crate) use targets_implicit_class::targets_implicit_class;
pub(crate) use targets_node::targets_node;
pub(crate) use targets_objects_of::targets_objects_of;
pub(crate) use targets_subjects_of::targets_subjects_of;

pub(crate) fn targets<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target<RDF>>> {
    let others: Vec<Box<dyn RDFNodeParse<RDF, Output = Vec<Target<RDF>>>>> = vec![
        Box::new(targets_node()),
        Box::new(targets_implicit_class()),
        Box::new(targets_subjects_of()),
        Box::new(targets_objects_of()),
    ];

    Box::new(targets_class()).combine_many(others)
}
