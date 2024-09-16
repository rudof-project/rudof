//! A set whose elements can be repeated. The set tracks how many times each element appears
//!

use oxiri::Iri;
use std::{fmt::Display, io::BufRead, path::Path};

use iri_s::IriS;
use srdf::{RDFFormat, ReaderMode, SRDFGraph};

use crate::{ServiceDescriptionError, ServiceDescriptionParser};

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct ServiceDescription {
    endpoint: IriS,
    default_dataset: DatasetDescription,
    supported_language: Vec<SupportedLanguage>,
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub enum SupportedLanguage {
    SPARQL10Query,

    #[default]
    SPARQL11Query,

    SPARQL11Update,
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct DatasetDescription {
    default_graph: GraphDescription,
    named_graphs: Vec<NamedGraphDescription>,
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct GraphDescription {
    class_partition: Vec<ClassPartition>,
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct NamedGraphDescription {}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct ClassPartition {
    class: IriS,
    property_partition: PropertyPartition,
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct PropertyPartition {
    property: IriS,
    class_partition: Vec<ClassPartition>,
    datatype_partition: Option<DatatypePartition>,
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct DatatypePartition {
    datatype: IriS,
}

impl ServiceDescription {
    pub fn new(endpoint: IriS) -> ServiceDescription {
        ServiceDescription {
            endpoint: endpoint.clone(),
            default_dataset: DatasetDescription::default(),
            supported_language: Vec::new(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(
        path: P,
        format: &RDFFormat,
        base: Option<Iri<String>>,
        reader_mode: &ReaderMode,
    ) -> Result<ServiceDescription, ServiceDescriptionError> {
        let rdf = SRDFGraph::from_path(path, format, base, reader_mode)?;
        let mut parser = ServiceDescriptionParser::new(rdf);
        let service = parser.parse()?;
        Ok(service)
    }

    pub fn from_reader<R: BufRead>(
        read: R,
        format: &RDFFormat,
        base: Option<Iri<String>>,
        reader_mode: &ReaderMode,
    ) -> Result<ServiceDescription, ServiceDescriptionError> {
        let rdf = SRDFGraph::from_reader(read, format, base, reader_mode)?;
        let mut parser = ServiceDescriptionParser::new(rdf);
        let service = parser.parse()?;
        Ok(service)
    }
}

impl Display for ServiceDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Service")?;
        writeln!(f, "  endpoint: {}", self.endpoint.as_str())?;
        Ok(())
    }
}
