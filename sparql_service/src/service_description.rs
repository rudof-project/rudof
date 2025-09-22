//! A set whose elements can be repeated. The set tracks how many times each element appears
//!
use crate::{
    Dataset, Feature, GraphCollection, ServiceDescriptionError, ServiceDescriptionFormat,
    ServiceDescriptionParser, SparqlResultFormat, SupportedLanguage,
};
use iri_s::IriS;
use itertools::Itertools;
use mie::Mie;
use serde::{Deserialize, Serialize};
use srdf::{RDFFormat, ReaderMode, SRDFGraph};
use std::{
    collections::HashSet,
    fmt::Display,
    io::{self},
    path::Path,
};

/// Describes the service description of a SPARQL endpoint
/// The description is parsed from RDF according to the
/// [Service Description](https://www.w3.org/TR/sparql11-service-description/) spec
/// and the [VoID voacabulary](https://www.w3.org/TR/void/).
#[derive(Clone, PartialEq, Eq, Default, Debug, Serialize, Deserialize)]
pub struct ServiceDescription {
    /// Title of service description
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<IriS>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_dataset: Option<Dataset>,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    supported_language: HashSet<SupportedLanguage>,

    /// Features supported by the endpoint
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    feature: HashSet<Feature>,

    /// Result formats supported by the endpoint
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    result_format: HashSet<SparqlResultFormat>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    available_graphs: Vec<GraphCollection>,
}

impl ServiceDescription {
    pub fn new() -> ServiceDescription {
        ServiceDescription {
            title: None,
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

    pub fn add_title(&mut self, title: Option<&str>) {
        self.title = title.map(|t| t.to_string());
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    pub fn endpoint(&self) -> &Option<IriS> {
        &self.endpoint
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

    pub fn service2mie(&self) -> Mie {
        let mut mie = Mie::default();
        let endpoint = self.endpoint.as_ref().map(|e| e.as_str());
        mie.add_endpoint(endpoint);

        if let Some(title) = &self.title {
            mie.add_title(title);
        }

        for _graph in self.available_graphs.iter() {
            // let graph_name = graph.graph_name().as_ref().map(|g| g.as_str());
            // mie.add_graph(graphs.service2mie());
        }
        mie
    }

    pub fn serialize<W: io::Write>(
        &self,
        format: &crate::ServiceDescriptionFormat,
        writer: &mut W,
    ) -> io::Result<()> {
        match format {
            ServiceDescriptionFormat::Internal => writer.write_all(self.to_string().as_bytes()),
            ServiceDescriptionFormat::Mie => {
                let mie = self.service2mie();
                let mie_str = serde_json::to_string(&mie).map_err(|e| {
                    io::Error::other(format!("Error converting ServiceDescription to MIE: {e}"))
                })?;
                writer.write_all(mie_str.as_bytes())
            }
            ServiceDescriptionFormat::Json => {
                let json = serde_json::to_string_pretty(self).map_err(|e| {
                    io::Error::other(format!("Error converting ServiceDescription to JSON: {e}"))
                })?;
                writer.write_all(json.as_bytes())
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
