use rudof_rdf::rdf_core::Rdf;
use shacl::ast::ASTSchema;
use shacl::validator::report::ValidationReport;

pub(crate) struct TestInstance<RDF: Rdf> {
    pub data: RDF,
    pub shapes: ASTSchema,
    pub report: ValidationReport,
}

impl<RDF: Rdf> TestInstance<RDF> {
    pub fn new(data: RDF, shapes: ASTSchema, report: ValidationReport) -> Self {
        Self { data, shapes, report }
    }
}