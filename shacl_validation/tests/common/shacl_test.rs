use rudof_rdf::rdf_core::Rdf;
use shacl::ast::ASTSchema;
use shacl_validation::validation_report::report::ValidationReport;

pub struct ShaclTest<R: Rdf> {
    pub data: R,
    pub shapes: ASTSchema,
    pub report: ValidationReport,
}

impl<R: Rdf> ShaclTest<R> {
    pub fn new(data: R, shapes: ASTSchema, report: ValidationReport) -> Self {
        ShaclTest { data, shapes, report }
    }
}
