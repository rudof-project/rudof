use std::fmt::Debug;

use oxigraph::store::Store;

/// Generic abstraction that represents RDF Data which can be  behind SPARQL
/// endpoints or an in-memory graph or both. The triples in RdfData are taken as
/// the union of the triples of the endpoints and the in-memory graph.
#[derive(Clone)]
pub struct RdfData {
    /// Current focus node used when parsing
    focus: Option<OxTerm>,
    /// List of SPARQL endpoints
    endpoints: Vec<SRDFSparql>,
    /// In-memory graph
    graph: Option<GenericGraph>,
    /// In-memory Store used to access the graph using SPARQL queries
    store: Option<Store>,
}

impl RdfData {
    pub fn default() -> RdfData {
        RdfData {
            endpoints: Vec::new(),
            graph: None,
            store: None,
            focus: None,
        }
    }

    /// Creates an RdfData from an in-memory RDF Graph
    pub fn from_graph(graph: GenericGraph) -> Result<RdfData, RdfDataError> {
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
    pub fn graph(&self) -> Option<&GenericGraph> {
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
                let mut graph = GenericGraph::new();
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
}

impl Rdf for RdfData {}

impl FocusRDF for RdfData {
    fn set_focus(&mut self, focus: &Self::Term) {
        self.focus = Some(focus.clone())
    }

    fn get_focus(&self) -> &Option<Self::Term> {
        &self.focus
    }
}

impl MutableRdf for RdfData {
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

    fn add_triple(
        &mut self,
        _subj: &Self::Subject,
        _pred: &Self::IRI,
        _obj: &Self::Term,
    ) -> Result<(), Self::Err> {
        todo!()
    }

    fn remove_triple(
        &mut self,
        _subj: &Self::Subject,
        _pred: &Self::IRI,
        _obj: &Self::Term,
    ) -> Result<(), Self::Err> {
        todo!()
    }

    fn add_type(&mut self, _node: &srdf::RDFNode, _type_: Self::Term) -> Result<(), Self::Err> {
        todo!()
    }

    fn serialize<W: std::io::Write>(
        &self,
        format: RDFFormat,
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
}

impl QuerySRDF for RdfData {
    fn query_select(&self, query_str: &str) -> Result<QuerySolutions<RdfData>, RdfDataError>
    where
        Self: Sized,
    {
        let mut sols: QuerySolutions<RdfData> = QuerySolutions::empty();
        let query = Query::parse(query_str, None)?;
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

impl Default for RdfData {
    fn default() -> Self {
        Self::new()
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
    QuerySolution::new(Rc::new(variables), values)
}
