use rdf::rdf_core::Rdf;
use shacl_ast::ShaclSchema;
use shacl_validation::validation_report::report::ValidationReport;

pub struct ShaclTest<R: Rdf> {
    pub data: R,
    pub shapes: ShaclSchema<R>,
    pub report: ValidationReport,
}

impl<R: Rdf> ShaclTest<R> {
    pub fn new(data: R, shapes: ShaclSchema<R>, report: ValidationReport) -> Self {
        ShaclTest { data, shapes, report }
    }
}
