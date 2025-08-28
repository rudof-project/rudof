//! A set whose elements can be repeated. The set tracks how many times each element appears
//!

use std::{collections::HashSet, fmt::Display, io::BufRead, path::Path};

use iri_s::IriS;
use itertools::Itertools;
use srdf::{RDFFormat, ReaderMode, SRDFGraph};

use crate::{ServiceDescriptionError, ServiceDescriptionParser};

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct ServiceDescription {
    endpoint: IriS,
    default_dataset: Dataset,
    supported_language: HashSet<SupportedLanguage>,
    feature: HashSet<Feature>,
    result_format: HashSet<SparqlResultFormat>,
}

#[derive(Clone, PartialEq, Eq, Default, Debug, Hash)]
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

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum SparqlResultFormat {
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

impl Display for SparqlResultFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SparqlResultFormat::XML => write!(f, "XML"),
            SparqlResultFormat::Turtle => write!(f, "Turtle"),
            SparqlResultFormat::TSV => write!(f, "TSV"),
            SparqlResultFormat::RdfXml => write!(f, "RDF/XML"),
            SparqlResultFormat::JSON => write!(f, "JSON"),
            SparqlResultFormat::NTriples => write!(f, "N-TRIPLES"),
            SparqlResultFormat::CSV => write!(f, "CSV"),
            SparqlResultFormat::JsonLD => write!(f, "JSON_LD"),
            SparqlResultFormat::Other(iri) => write!(f, "ResultFormat({iri})",),
        }
    }
}

/// Features defined in: https://www.w3.org/TR/sparql11-service-description/#sd-Feature
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
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
            supported_language: HashSet::new(),
            feature: HashSet::new(),
            result_format: HashSet::new(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(
        path: P,
        format: &RDFFormat,
        base: Option<&str>,
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
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<ServiceDescription, ServiceDescriptionError> {
        let rdf = SRDFGraph::from_reader(read, format, base, reader_mode)?;
        let mut parser = ServiceDescriptionParser::new(rdf);
        let service = parser.parse()?;
        Ok(service)
    }

    pub fn add_supported_languages<I: IntoIterator<Item = SupportedLanguage>>(
        &mut self,
        supported_languages: I,
    ) {
        self.supported_language.extend(supported_languages);
    }

    pub fn add_features<I: IntoIterator<Item = Feature>>(&mut self, features: I) {
        self.feature.extend(features);
    }

    pub fn add_result_formats<I: IntoIterator<Item = SparqlResultFormat>>(
        &mut self,
        result_formats: I,
    ) {
        self.result_format.extend(result_formats);
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
