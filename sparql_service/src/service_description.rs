//! A set whose elements can be repeated. The set tracks how many times each element appears
//!

use std::{
    collections::HashSet,
    fmt::Display,
    io::{self},
    path::Path,
};

use iri_s::IriS;
use itertools::Itertools;
use srdf::{RDFFormat, ReaderMode, SRDFGraph};

use crate::{
    Dataset, Feature, GraphCollection, ServiceDescriptionError, ServiceDescriptionParser,
    SparqlResultFormat, SupportedLanguage,
};

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct ServiceDescription {
    endpoint: Option<IriS>,
    default_dataset: Option<Dataset>,
    supported_language: HashSet<SupportedLanguage>,
    feature: HashSet<Feature>,
    result_format: HashSet<SparqlResultFormat>,
    available_graphs: Vec<GraphCollection>,
}

impl ServiceDescription {
    pub fn new() -> ServiceDescription {
        ServiceDescription {
            endpoint: None,
            default_dataset: None,
            supported_language: HashSet::new(),
            feature: HashSet::new(),
            result_format: HashSet::new(),
            available_graphs: Vec::new(),
        }
    }

    pub fn with_endpoint(mut self, endpoint: Option<IriS>) -> Self {
        self.endpoint = endpoint;
        self
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

    pub fn from_reader<R: io::Read>(
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

    pub fn with_default_dataset(mut self, default_dataset: Option<Dataset>) -> Self {
        self.default_dataset = default_dataset;
        self
    }

    pub fn with_available_graphs(mut self, available_graphs: Vec<GraphCollection>) -> Self {
        self.available_graphs = available_graphs;
        self
    }

    pub fn serialize<W: io::Write>(
        &self,
        format: &crate::ServiceDescriptionFormat,
        writer: &mut W,
    ) -> io::Result<()> {
        match format {
            crate::ServiceDescriptionFormat::Internal => {
                writer.write_all(self.to_string().as_bytes())
            }
        }
    }
}

impl Display for ServiceDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Service")?;
        if let Some(endpoint) = &self.endpoint {
            writeln!(f, " endpoint: {}", endpoint.as_str())?;
        } else {
            writeln!(f, " endpoint: None")?;
        }
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
        if let Some(default_ds) = &self.default_dataset {
            writeln!(f, "  default_dataset: {}", default_ds)?;
        } else {
            writeln!(f, "  default_dataset: None")?;
        }
        writeln!(
            f,
            "  availableGraphs: {}",
            self.available_graphs
                .iter()
                .map(|a| a.to_string())
                .join(", ")
        )?;
        Ok(())
    }
}
