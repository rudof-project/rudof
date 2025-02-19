use std::fmt::Debug;

use oxigraph::store::Store;
use oxrdf::Term as OxTerm;
use oxrdf::Triple as OxTriple;
use prefixmap::PrefixMap;
use sparesults::QuerySolution as OxQuerySolution;
use srdf::model::matcher::Matcher;
use srdf::model::rdf::FocusRdf;
use srdf::model::rdf::MutableRdf;
use srdf::model::rdf::Object;
use srdf::model::rdf::PrefixMapRdf;
use srdf::model::rdf::Rdf;
use srdf::model::reader::RdfReader;
use srdf::model::reader::ReaderMode;
use srdf::model::sparql::Sparql;
use srdf::model::RdfFormat;
use srdf::oxgraph::GenericGraph;
use srdf::oxgraph::OxGraph;
use srdf::OxSparql;

use super::RdfDataError;

/// Generic abstraction that represents RDF Data which can be  behind SPARQL
/// endpoints or an in-memory graph or both. The triples in RdfData are taken as
/// the union of the triples of the endpoints and the in-memory graph.
#[derive(Clone)]
pub struct RdfData {
    /// Current focus node used when parsing
    focus: Option<OxTerm>,
    /// List of SPARQL endpoints
    endpoints: Vec<OxSparql>,
    /// In-memory graph
    graph: Option<OxGraph>,
    /// In-memory Store used to access the graph using SPARQL queries
    store: Option<Store>,
}

impl RdfData {
    /// Creates an RdfData from an in-memory RDF Graph.
    pub fn from_graph(graph: OxGraph) -> Self {
        Self {
            endpoints: Vec::new(),
            graph: Some(graph),
            store: None,
            focus: None,
        }
    }

    /// Creates an RdfData from an endpoint.
    pub fn from_endpoint(endpoint: OxSparql) -> Self {
        Self {
            endpoints: vec![endpoint],
            graph: None,
            store: None,
            focus: None,
        }
    }

    // Cleans the values of endpoints and graph.
    pub fn clean_all(&mut self) {
        self.clean_graph();
        self.clean_endpoints();
    }

    /// Cleans the in-memory graph.
    pub fn clean_graph(&mut self) {
        self.graph = None
    }

    /// Cleans the list of endpoints.
    pub fn clean_endpoints(&mut self) {
        self.endpoints = Default::default();
    }

    /// Get the in-memory graph
    pub fn graph(&self) -> Option<&OxGraph> {
        self.graph.as_ref()
    }

    /// Adds a new endpoint to the list of endpoints
    pub fn add_endpoint(&mut self, endpoint: OxSparql) {
        // TODO: Ensure that there are no repeated endpoints
        self.endpoints.push(endpoint);
    }

    // pub fn show_blanknode(&self, bn: &OxBlankNode) -> String {
    //     let str: String = format!("{}", bn);
    //     format!("{}", str.green())
    // }

    // pub fn show_literal(&self, lit: &OxLiteral) -> String {
    //     let str: String = format!("{}", lit);
    //     format!("{}", str.red())
    // }
}

impl PrefixMapRdf for RdfData {
    fn prefixmap(&self) -> &PrefixMap {
        self.graph.as_ref().map(|g| g.prefixmap()).unwrap()
    }
}

impl Debug for RdfData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RdfData")
            .field("endpoints", &self.endpoints)
            .field("graph", &self.graph)
            .finish()
    }
}

impl Default for RdfData {
    fn default() -> Self {
        Self {
            endpoints: Default::default(),
            graph: Default::default(),
            store: Default::default(),
            focus: Default::default(),
        }
    }
}

impl Rdf for RdfData {
    type Triple = OxTriple;
    type Error = RdfDataError;

    fn triples_matching(
        &self,
        subject: impl Into<Matcher<Self>>,
        predicate: impl Into<Matcher<Self>>,
        object: impl Into<Matcher<Self>>,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Error> {
        let triples = self
            .endpoints
            .iter()
            .flat_map(|endpoint| endpoint.triples_matching(subject, predicate, object))
            .flatten();

        // let triples = if let Some(graph) = &self.graph {
        //     let graph_triples = graph.triples_matching(subject, predicate, object)?;
        //     triples.chain(graph_triples)
        // } else {
        //     triples
        // };

        Ok(triples)
    }
}

impl MutableRdf for RdfData {
    type MutableRdfError = RdfDataError;

    fn add_triple(&mut self, triple: Self::Triple) -> Result<(), Self::MutableRdfError> {
        todo!()
    }

    fn remove_triple(&mut self, triple: &Self::Triple) -> Result<(), Self::MutableRdfError> {
        todo!()
    }

    fn add_base(
        &mut self,
        base: srdf::model::rdf::Predicate<Self>,
    ) -> Result<(), Self::MutableRdfError> {
        todo!()
    }

    fn add_prefix(
        &mut self,
        alias: &str,
        iri: srdf::model::rdf::Predicate<Self>,
    ) -> Result<(), Self::MutableRdfError> {
        todo!()
    }

    // fn serialize<W: std::io::Write>(
    //     &self,
    //     format: RDFFormat,
    //     writer: &mut W,
    // ) -> Result<(), Self::Err> {
    //     if let Some(graph) = &self.graph {
    //         graph.serialize(format, writer)?;
    //         Ok::<(), Self::Err>(())
    //     } else {
    //         Ok(())
    //     }?;
    //     for endpoint in &self.endpoints {
    //         writeln!(writer, "Endpoint {}", endpoint.iri())?;
    //     }
    //     Ok(())
    // }
}

impl FocusRdf for RdfData {
    fn set_focus(&mut self, focus: Object<Self>) {
        self.focus = Some(focus)
    }

    fn get_focus(&self) -> Option<&Object<Self>> {
        self.focus.as_ref()
    }
}

impl RdfReader for RdfData {
    type ReaderError = RdfDataError;

    /// Merge the in-memory graph with the graph read from a reader
    fn merge_from_reader<R: std::io::Read>(
        &mut self,
        read: R,
        format: RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), Self::ReaderError> {
        match &mut self.graph {
            Some(ref mut graph) => graph
                .merge_from_reader(read, format, base, reader_mode)
                .map_err(|e| RdfDataError::GraphParse { err: e }),
            None => {
                let mut graph = GenericGraph::default();
                graph
                    .merge_from_reader(read, format, base, reader_mode)
                    .map_err(|e| RdfDataError::GraphParse { err: e })?;
                self.graph = Some(graph);
                Ok(())
            }
        }
    }
}

impl Sparql for RdfData {
    type QuerySolution = OxQuerySolution;
    type SparqlError = RdfDataError;

    fn make_sparql_query(
        &self,
        query: String,
    ) -> Result<Vec<Self::QuerySolution>, Self::SparqlError> {
        let mut sols = Vec::new();
        for endpoints in self.endpoints.iter() {
            let endpoint_solutions = endpoints.make_sparql_query(query.clone())?;
            sols.extend(endpoint_solutions);
        }
        // if let Some(store) = self.store { TODO
        //     let store_solutions = store.make_sparql_query(query);
        //     sols.extend(endpoint_solutions);
        // }
        Ok(sols)
    }
}
