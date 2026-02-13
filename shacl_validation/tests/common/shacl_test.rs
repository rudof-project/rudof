use shacl_ast::Schema;
use shacl_validation::validation_report::report::ValidationReport;
use rdf::rdf_core::Rdf; 

pub struct ShaclTest<R: Rdf> {
    pub data: R,
    pub shapes: Schema<R>,
    pub report: ValidationReport,
}

impl<R: Rdf> ShaclTest<R> {
    pub fn new(data: R, shapes: Schema<R>, report: ValidationReport) -> Self {
        ShaclTest {
            data,
            shapes,
            report,
        }
    }
}
