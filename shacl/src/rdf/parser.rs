use crate::ast::ASTShape;
use rudof_rdf::rdf_core::parser::RDFParse;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::HashMap;

pub(crate) struct ShaclParser<RDF: FocusRDF> {
    rdf_parser: RDFParse<RDF>,
    shapes: HashMap<Object, ASTShape>,
}

impl<RDF: FocusRDF> ShaclParser<RDF> {
    pub fn new(rdf: RDF) -> Self<> {
        Self {
            rdf_parser: RDFParse::new(rdf),
            shapes: HashMap::new(),
        }
    }
}