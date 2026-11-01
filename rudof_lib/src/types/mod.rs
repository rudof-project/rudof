mod bindings;
mod data;
mod data_diff;
mod node_neighborhood;
mod query;
mod shex_statistics;
mod staging;
mod staging_guard;
mod triple;

pub use bindings::Bindings;
pub(crate) use data::Data;
pub use data_diff::{DataDiff, describe_relation};
pub use node_neighborhood::{NeighborArc, NodeNeighborhood};
pub use query::{QueryResult, QueryResultKind};
pub use rudof_rdf::rdf_core::ArcDirection;
pub use rudof_rdf::rdf_core::term::{IriOrBlankNode, Object};
pub use rudof_rdf::rdf_impl::{DEFAULT_EXACT_SEARCH_LIMIT, DiffOptions, GraphDiff, GraphRelation, SubgraphMode};
pub(crate) use shex_statistics::ShExStatistics;
pub(crate) use staging::{StagingFrame, UndoEntry};
pub use staging_guard::StagingGuard;
pub use triple::{Triple, Triples};

#[cfg(feature = "pgschema")]
pub use pgschema::result_association::ResultAssociation as PgSchemaResultAssociation;
#[cfg(feature = "pgschema")]
pub use pgschema::validation_result::ValidationResult as PgSchemaValidationResult;
pub use shacl::types::Severity as ShaclSeverity;
pub use shacl::validator::report::{
    ValidationReport as ShaclValidationReport, ValidationResult as ShaclValidationResult,
};
pub use shex_ast::ir::shape_label::ShapeLabel as ShExShapeLabel;
pub use shex_ast::node::Node as ShExNode;
pub use shex_ast::shapemap::{ResultShapeMap, ValidationStatus as ShExValidationStatus};
