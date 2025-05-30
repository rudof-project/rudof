use super::RdfDataError;
use colored::*;
use iri_s::IriS;
use oxigraph::sparql::Query as OxQuery;
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, Subject as OxSubject,
    Term as OxTerm, Triple as OxTriple,
};
use oxrdfio::RdfFormat;
use prefixmap::PrefixMap;
use sparesults::QuerySolution as SparQuerySolution;
use srdf::FocusRDF;
use srdf::Query;
use srdf::QuerySolution;
use srdf::QuerySolutions;
use srdf::RDFFormat;
use srdf::Rdf;
use srdf::ReaderMode;
use srdf::SRDFBuilder;
use srdf::SRDFGraph;
use srdf::SRDFSparql;
use srdf::Sparql;
use srdf::VarName;
use srdf::RDF_TYPE_STR;
use std::fmt::Debug;
use std::io;
use std::str::FromStr;

/// Generic abstraction that represents RDF Data which can be  behind SPARQL endpoints or an in-memory graph or both
/// The triples in RdfData are taken as the union of the triples of the endpoints and the in-memory graph
#[derive(Clone)]
pub struct RdfData {
    /// Current focus node used when parsing
    focus: Option<OxTerm>,

    /// List of SPARQL endpoints
    endpoints: Vec<SRDFSparql>,

    /// In-memory graph
    graph: Option<SRDFGraph>,

    /// In-memory Store used to access the graph using SPARQL queries
    store: Option<Store>,
}

impl Debug for RdfData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RdfData")
            .field("endpoints", &self.endpoints)
            .field("graph", &self.graph)
            .finish()
    }
}

impl RdfData {
    pub fn new() -> RdfData {
        RdfData {
            endpoints: Vec::new(),
            graph: None,
            store: None,
            focus: None,
        }
    }

    /// Checks if the Store has been initialized
    ///
    /// By default, the RDF Data Store is not initialized as it is expensive and is only required for SPARQL queries
    pub fn check_store(&mut self) -> Result<(), RdfDataError> {
        if let Some(graph) = &self.graph {
            if self.store.is_none() {
                let store = Store::new()?;
                store.bulk_loader().load_quads(graph.quads())?;
                self.store = Some(store)
            }
        }
        Ok(())
    }

    /// Creates an RdfData from an in-memory RDF Graph
    pub fn from_graph(graph: SRDFGraph) -> Result<RdfData, RdfDataError> {
        let store = Store::new()?;
        store.bulk_loader().load_quads(graph.quads())?;
        Ok(RdfData {
            endpoints: Vec::new(),
            graph: Some(graph),
            store: Some(store),
            focus: None,
        })
    }

    // Cleans the values of endpoints and graph
    pub fn clean_all(&mut self) {
        self.endpoints = Vec::new();
        self.graph = None
    }

    /// Get the in-memory graph
    pub fn graph(&self) -> Option<&SRDFGraph> {
        self.graph.as_ref()
    }

    /// Cleans the in-memory graph
    pub fn clean_graph(&mut self) {
        self.graph = None
    }

    /// Merge the in-memory graph with the graph read from a reader
    pub fn merge_from_reader<R: io::Read>(
        &mut self,
        read: R,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), RdfDataError> {
        match &mut self.graph {
            Some(ref mut graph) => graph
                .merge_from_reader(read, format, base, reader_mode)
                .map_err(|e| RdfDataError::SRDFGraphError { err: e }),
            None => {
                let mut graph = SRDFGraph::new();
                graph
                    .merge_from_reader(read, format, base, reader_mode)
                    .map_err(|e| RdfDataError::SRDFGraphError { err: e })?;
                self.graph = Some(graph);
                Ok(())
            }
        }
    }

    /// Creates an RdfData from an endpoint
    pub fn from_endpoint(endpoint: SRDFSparql) -> RdfData {
        RdfData {
            endpoints: vec![endpoint],
            graph: None,
            store: None,
            focus: None,
        }
    }

    /// Adds a new endpoint to the list of endpoints
    pub fn add_endpoint(&mut self, endpoint: SRDFSparql) {
        // TODO: Ensure that there are no repeated endpoints
        self.endpoints.push(endpoint);
    }

    /// Gets the PrefixMap from the in-memory graph
    pub fn prefixmap_in_memory(&self) -> PrefixMap {
        self.graph
            .as_ref()
            .map(|g| g.prefixmap())
            .unwrap_or_default()
    }

    pub fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        let str: String = format!("{}", bn);
        format!("{}", str.green())
    }

    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        let str: String = format!("{}", lit);
        format!("{}", str.red())
    }

    pub fn serialize<W: io::Write>(
        &self,
        format: &RDFFormat,
        writer: &mut W,
    ) -> Result<(), RdfDataError> {
        if let Some(graph) = &self.graph {
            graph
                .serialize(format, writer)
                .map_err(|e| RdfDataError::Serializing {
                    format: *format,
                    error: format!("{e}"),
                })?
        }
        for e in self.endpoints.iter() {
            writeln!(writer, "Endpoint {}", e.iri())?
        }
        Ok(())
    }
}

impl Default for RdfData {
    fn default() -> Self {
        Self::new()
    }
}

impl Rdf for RdfData {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Triple = OxTriple;
    type Err = RdfDataError;

    fn prefixmap(&self) -> std::option::Option<PrefixMap> {
        self.graph.as_ref().map(|g| g.prefixmap())
    }

    fn qualify_iri(&self, node: &Self::IRI) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap();
        if let Some(graph) = &self.graph {
            graph.prefixmap().qualify(&iri)
        } else {
            for e in self.endpoints.iter() {
                if let Some(qualified) = e.prefixmap().qualify_optional(&iri) {
                    return qualified;
                }
            }
            format!("<{node}>")
        }
    }

    fn qualify_subject(&self, subj: &Self::Subject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_iri(n),
            // #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => unimplemented!(),
        }
    }

    fn qualify_term(&self, term: &Self::Term) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
            // #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn resolve_prefix_local(
        &self,
        prefix: &str,
        local: &str,
    ) -> Result<IriS, prefixmap::PrefixMapError> {
        if let Some(graph) = self.graph() {
            let iri = graph.prefixmap().resolve_prefix_local(prefix, local)?;
            Ok(iri.clone())
        } else {
            for e in self.endpoints.iter() {
                if let Ok(iri) = e.prefixmap().resolve_prefix_local(prefix, local) {
                    return Ok(iri.clone());
                }
            }
            Err(prefixmap::PrefixMapError::PrefixNotFound {
                prefix: prefix.to_string(),
                prefixmap: PrefixMap::new(),
            })
        }
    }
}

impl Sparql for RdfData {
    fn query_select(&self, query_str: &str) -> Result<QuerySolutions<RdfData>, RdfDataError>
    where
        Self: Sized,
    {
        let mut sols: QuerySolutions<RdfData> = QuerySolutions::empty();
        let query = OxQuery::parse(query_str, None)?;
        if let Some(store) = &self.store {
            let new_sol = store.query(query)?;
            let sol = cnv_query_results(new_sol)?;
            sols.extend(sol)
        }
        for endpoint in &self.endpoints {
            let new_sols = endpoint.query_select(query_str)?;
            let new_sols_converted: Vec<QuerySolution<RdfData>> =
                new_sols.iter().map(cnv_sol).collect();
            sols.extend(new_sols_converted)
        }
        Ok(sols)
    }

    fn query_ask(&self, _query: &str) -> Result<bool, Self::Err> {
        todo!()
    }
}

fn cnv_sol(sol: &QuerySolution<SRDFSparql>) -> QuerySolution<RdfData> {
    sol.convert(|t| t.clone())
}

fn cnv_query_results(
    query_results: QueryResults,
) -> Result<Vec<QuerySolution<RdfData>>, RdfDataError> {
    let mut results = Vec::new();
    if let QueryResults::Solutions(solutions) = query_results {
        for solution in solutions {
            let result = cnv_query_solution(solution?);
            results.push(result)
        }
    }
    Ok(results)
}

fn cnv_query_solution(qs: SparQuerySolution) -> QuerySolution<RdfData> {
    let mut variables = Vec::new();
    let mut values = Vec::new();
    for v in qs.variables() {
        let varname = VarName::new(v.as_str());
        variables.push(varname);
    }
    for t in qs.values() {
        let term = t.clone();
        values.push(term)
    }
    QuerySolution::new(variables, values)
}

fn _cnv_rdf_format(rdf_format: RDFFormat) -> RdfFormat {
    match rdf_format {
        RDFFormat::NTriples => RdfFormat::NTriples,
        RDFFormat::Turtle => RdfFormat::Turtle,
        RDFFormat::RDFXML => RdfFormat::RdfXml,
        RDFFormat::TriG => RdfFormat::TriG,
        RDFFormat::N3 => RdfFormat::N3,
        RDFFormat::NQuads => RdfFormat::NQuads,
    }
}

fn _rdf_type() -> OxNamedNode {
    OxNamedNode::new_unchecked(RDF_TYPE_STR)
}

impl Query for RdfData {
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        let endpoints_triples = self.endpoints.iter().flat_map(Query::triples).flatten();
        let graph_triples = self.graph.iter().flat_map(Query::triples).flatten();
        Ok(endpoints_triples.chain(graph_triples))
    }
}

impl FocusRDF for RdfData {
    fn set_focus(&mut self, focus: &Self::Term) {
        self.focus = Some(focus.clone())
    }

    fn get_focus(&self) -> &Option<Self::Term> {
        &self.focus
    }
}

impl SRDFBuilder for RdfData {
    fn empty() -> Self {
        todo!()
    }

    fn add_base(&mut self, _base: &Option<IriS>) -> Result<(), Self::Err> {
        todo!()
    }

    fn add_prefix(&mut self, _alias: &str, _iri: &IriS) -> Result<(), Self::Err> {
        todo!()
    }

    fn add_prefix_map(&mut self, _prefix_map: PrefixMap) -> Result<(), Self::Err> {
        todo!()
    }

    fn add_triple<S, P, O>(&mut self, _subj: S, _pred: P, _obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>,
    {
        todo!()
    }

    fn remove_triple<S, P, O>(&mut self, _subj: S, _pred: P, _obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>,
    {
        todo!()
    }

    fn add_type<S, T>(&mut self, _node: S, _type_: T) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        T: Into<Self::Term>,
    {
        todo!()
    }

    fn serialize<W: std::io::Write>(
        &self,
        format: &RDFFormat,
        writer: &mut W,
    ) -> Result<(), Self::Err> {
        if let Some(graph) = &self.graph {
            graph.serialize(format, writer)?;
            Ok::<(), Self::Err>(())
        } else {
            Ok(())
        }?;
        for endpoint in &self.endpoints {
            writeln!(writer, "Endpoint {}", endpoint.iri())?;
        }
        Ok(())
    }

    fn add_bnode(&mut self) -> Result<Self::BNode, Self::Err> {
        match self.graph {
            Some(ref mut graph) => {
                let bnode = graph.add_bnode()?;
                Ok(bnode)
            }
            None => Err(RdfDataError::BNodeNoGraph),
        }
    }
}
