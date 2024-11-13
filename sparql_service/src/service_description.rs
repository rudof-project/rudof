//! A set whose elements can be repeated. The set tracks how many times each element appears
//!

use std::{fmt::Display, io::BufRead, path::Path};

use iri_s::IriS;
use itertools::Itertools;
use srdf::{RDFFormat, ReaderMode, GenericGraph};

use crate::{ServiceDescriptionError, ServiceDescriptionParser};

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct ServiceDescription {
    endpoint: IriS,
    default_dataset: Dataset,
    supported_language: Vec<SupportedLanguage>,
    feature: Vec<Feature>,
    result_format: Vec<ResultFormat>,
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub enum SupportedLanguage {
    SPARQL10Query,

    #[default]
    SPARQL11Query,

    SPARQL11Update,
}

impl Display for SupportedLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedLanguage::SPARQL10Query => write!(f, "SPARQL10Query"),
            SupportedLanguage::SPARQL11Query => write!(f, "SPARQL11Query"),
            SupportedLanguage::SPARQL11Update => write!(f, "SPARQL11Update"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ResultFormat {
    XML,
    Turtle,
    TSV,
    RdfXml,
    JSON,
    NTriples,
    CSV,
    JsonLD,
    Other(IriS),
}

impl Display for ResultFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultFormat::XML => write!(f, "XML"),
            ResultFormat::Turtle => write!(f, "Turtle"),
            ResultFormat::TSV => write!(f, "TSV"),
            ResultFormat::RdfXml => write!(f, "RDF/XML"),
            ResultFormat::JSON => write!(f, "JSON"),
            ResultFormat::NTriples => write!(f, "N-TRIPLES"),
            ResultFormat::CSV => write!(f, "CSV"),
            ResultFormat::JsonLD => write!(f, "JSON_LD"),
            ResultFormat::Other(iri) => write!(f, "ResultFormat({iri})",),
        }
    }
}

/// Features defined in: https://www.w3.org/TR/sparql11-service-description/#sd-Feature
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Feature {
    DereferencesURIs,
    UnionDefaultGraph,
    RequiresDataset,
    EmptyGraphs,
    BasicFederatedQuery,
    Other(IriS),
}

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Feature::DereferencesURIs => write!(f, "DereferencesURIs"),
            Feature::UnionDefaultGraph => write!(f, "UnionDefaultGraph"),
            Feature::RequiresDataset => write!(f, "RequiresDataset"),
            Feature::EmptyGraphs => write!(f, "EmptyGraphs"),
            Feature::BasicFederatedQuery => write!(f, "BasicFederatedQuery"),
            Feature::Other(iri) => write!(f, "Feature({iri})"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Dataset {
    term: IriS,
    default_graph: GraphDescription,
    named_graphs: Vec<NamedGraphDescription>,
}

impl Dataset {
    pub fn new(iri: &IriS) -> Dataset {
        Dataset {
            term: iri.clone(),
            default_graph: GraphDescription::default(),
            named_graphs: Vec::new(),
        }
    }
}

impl Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dataset: {}", self.term)
    }
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct GraphDescription {
    triples: u128,
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
            default_dataset: Dataset::default(),
            supported_language: Vec::new(),
            feature: Vec::new(),
            result_format: Vec::new(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(
        path: P,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<ServiceDescription, ServiceDescriptionError> {
        let rdf = GenericGraph::from_path(path, format, base, reader_mode)?;
        let mut parser = ServiceDescriptionParser::new(rdf);
        let service = parser.parse()?;
        Ok(service)
    }

    pub fn from_reader<R: BufRead>(
        read: R,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<ServiceDescription, ServiceDescriptionError> {
        let rdf = GenericGraph::from_reader(read, format, base, reader_mode)?;
        let mut parser = ServiceDescriptionParser::new(rdf);
        let service = parser.parse()?;
        Ok(service)
    }

    pub fn add_supported_language(&mut self, supported_language: &[SupportedLanguage]) {
        supported_language.clone_into(&mut self.supported_language);
    }

    pub fn add_feature(&mut self, feature: &[Feature]) {
        feature.clone_into(&mut self.feature);
    }

    pub fn add_result_format(&mut self, result_format: &[ResultFormat]) {
        result_format.clone_into(&mut self.result_format);
    }

    pub fn add_default_dataset(&mut self, default_dataset: &Dataset) {
        self.default_dataset = default_dataset.clone();
    }
}

impl Display for ServiceDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Service")?;
        writeln!(f, "  endpoint: {}", self.endpoint.as_str())?;
        let sup_lang = self
            .supported_language
            .iter()
            .map(|l| l.to_string())
            .join(", ");
        writeln!(f, "  supportedLanguage: [{sup_lang}]")?;
        let feature = self.feature.iter().map(|l| l.to_string()).join(", ");
        writeln!(f, "  feature: [{feature}]")?;
        let result = self.result_format.iter().map(|l| l.to_string()).join(", ");
        writeln!(f, "  result_format: [{result}]")?;
        writeln!(f, "  default_dataset: {}", self.default_dataset)?;
        Ok(())
    }
}
