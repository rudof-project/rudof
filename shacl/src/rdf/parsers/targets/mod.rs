use crate::types::Target;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};

mod class;
mod implicit_class;
mod node;
mod objects_of;
mod subjects_of;

pub(crate) use class::targets_class;
pub(crate) use implicit_class::targets_implicit_class;
pub(crate) use node::targets_node;
pub(crate) use objects_of::targets_objects_of;
pub(crate) use subjects_of::targets_subjects_of;

pub(crate) fn targets<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target>> {
    let others: Vec<Box<dyn RDFNodeParse<RDF, Output = Vec<Target>>>> = vec![
        Box::new(targets_node()),
        Box::new(targets_implicit_class()),
        Box::new(targets_subjects_of()),
        Box::new(targets_objects_of()),
    ];

    Box::new(targets_class()).combine_many(others)
}
