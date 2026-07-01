use super::RdfDataError;
use colored::*;
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, NamedOrBlankNode as OxSubject,
    Term as OxTerm, Triple as OxTriple,
};
use prefixmap::PrefixMap;
use rudof_iri::IriS;
#[cfg(feature = "qlever")]
use rudof_rdf::rdf_impl::QleverGraphContainer;
use rudof_rdf::{
    rdf_core::{
        BuildRDF, FocusRDF, Matcher, NeighsRDF, RDFFormat, Rdf, RdfDataConfig,
        query::{QueryRDF, QueryResultFormat, QuerySolution, QuerySolutions},
    },
    rdf_impl::{OxigraphEndpoint, OxigraphInMemory, RdfBackend, ReaderMode},
};
use serde::Serialize;
use serde::ser::SerializeStruct;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::str::FromStr;

/// Federation aggregator that pairs a single primary [`RdfBackend`] with an
/// optional set of secondary SPARQL endpoints.
///
/// Triple and SPARQL queries are answered by the primary backend and unioned
/// with the results of every endpoint registered in `use_endpoints`. The
/// `endpoints` map is the full catalog (typically populated from
/// [`RdfDataConfig`]); `use_endpoints` is the subset actually queried.
#[derive(Clone)]
pub struct RdfData {
    /// Single backend that owns the data. Trait dispatch goes through this.
    primary: RdfBackend,

    /// Catalog of registered endpoints (e.g. loaded from the TOML config).
    /// Not queried directly — only those moved into `use_endpoints` are.
    endpoints: HashMap<String, OxigraphEndpoint>,

    /// Endpoints actively unioned with the primary on every query.
    use_endpoints: HashMap<String, OxigraphEndpoint>,
}

impl RdfData {
    pub fn new() -> RdfData {
        RdfData {
            primary: RdfBackend::default(),
            endpoints: HashMap::new(),
            use_endpoints: HashMap::new(),
        }
    }

    /// Borrow the primary backend.
    pub fn backend(&self) -> &RdfBackend {
        &self.primary
    }

    /// Mutable borrow of the primary backend.
    pub fn backend_mut(&mut self) -> &mut RdfBackend {
        &mut self.primary
    }

    /// Replace the primary backend.
    pub fn set_backend(&mut self, backend: RdfBackend) {
        self.primary = backend;
    }

    /// Replace the QLever-backed primary. Provided for ergonomic parity with
    /// `set_backend(RdfBackend::Qlever(g))`.
    #[cfg(feature = "qlever")]
    pub fn set_qlever(&mut self, graph: QleverGraphContainer) {
        self.primary = RdfBackend::Qlever(graph);
    }

    pub fn reset(&mut self) {
        self.use_endpoints.clear();
        self.primary = RdfBackend::default();
    }

    pub fn with_rdf_data_config(mut self, rdf_data_config: &RdfDataConfig) -> Result<Self, RdfDataError> {
        if let Some(endpoints) = &rdf_data_config.endpoints {
            for (name, endpoint_description) in endpoints.iter() {
                let sparql_endpoint =
                    OxigraphEndpoint::new(endpoint_description.query_url(), &endpoint_description.prefixmap())
                        .map_err(|e| RdfDataError::SRDFSparqlFromEndpointDescriptionError {
                            name: name.clone(),
                            url: endpoint_description.query_url().to_string(),
                            err: Box::new(e),
                        })?;
                self.add_endpoint(name, sparql_endpoint);
            }
        }
        Ok(self)
    }

    /// Eagerly populate the embedded SPARQL store on the in-memory primary so
    /// subsequent `query_select` / `query_ask` calls return real results.
    /// No-op for SPARQL endpoint and QLever primaries (they answer queries
    /// over their own remote / on-disk store already).
    pub fn check_store(&mut self) -> Result<(), RdfDataError> {
        if let Some(g) = self.primary.as_in_memory_mut() {
            g.ensure_store().map_err(|e| RdfDataError::Backend {
                err: Box::new(rudof_rdf::rdf_impl::RdfBackendError::from(e)),
            })?;
        }
        Ok(())
    }

    /// Creates an RdfData from an in-memory RDF graph.
    pub fn from_graph(graph: OxigraphInMemory) -> Result<RdfData, RdfDataError> {
        Ok(RdfData {
            primary: RdfBackend::InMemory(graph),
            endpoints: HashMap::new(),
            use_endpoints: HashMap::new(),
        })
    }

    /// Cleans the values of endpoints and graph.
    pub fn clean_all(&mut self) {
        self.endpoints.clear();
        self.use_endpoints.clear();
        self.primary = RdfBackend::default();
    }

    /// Returns the PrefixMap of the primary backend (empty if the primary
    /// has no prefixes).
    pub fn graph_prefixmap(&self) -> PrefixMap {
        <RdfBackend as Rdf>::prefixmap(&self.primary).unwrap_or_default()
    }

    /// Resets the primary backend to an empty in-memory graph.
    pub fn clean_graph(&mut self) {
        self.primary = RdfBackend::default();
    }

    pub fn from_str(
        data: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<RdfData, RdfDataError> {
        let mut rdf_data = Self::new();
        rdf_data.merge_from_reader(&mut data.as_bytes(), "String", format, base, reader_mode)?;
        Ok(rdf_data)
    }

    /// Merge the in-memory primary with the graph read from a reader.
    /// Errors if the primary is not an in-memory backend.
    pub fn merge_from_reader<R: io::Read>(
        &mut self,
        read: &mut R,
        source_name: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), RdfDataError> {
        let backend_name = match &self.primary {
            RdfBackend::InMemory(_) => "in-memory",
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => "sparql-endpoint",
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(_) => "qlever",
        };
        let RdfBackend::InMemory(graph) = &mut self.primary else {
            return Err(RdfDataError::NotInMemoryBackend { backend: backend_name });
        };
        graph
            .merge_from_reader(read, source_name, format, base, reader_mode)
            .map_err(|e| RdfDataError::Backend {
                err: Box::new(rudof_rdf::rdf_impl::RdfBackendError::from(e)),
            })
    }

    /// Creates an RdfData from a single endpoint (registered as both catalog
    /// and active federation member; primary is left as an empty in-memory
    /// backend).
    pub fn from_endpoint(name: &str, endpoint: OxigraphEndpoint) -> RdfData {
        RdfData {
            primary: RdfBackend::default(),
            endpoints: HashMap::from([(name.to_string(), endpoint.clone())]),
            use_endpoints: HashMap::from([(name.to_string(), endpoint)]),
        }
    }

    /// Adds an endpoint to the catalog. Not used at query time until
    /// `use_endpoint` selects it.
    pub fn add_endpoint(&mut self, name: &str, endpoint: OxigraphEndpoint) {
        self.endpoints
            .entry(name.to_string())
            .and_modify(|e| *e = endpoint.clone())
            .or_insert(endpoint);
    }

    pub fn use_endpoints(&self) -> &HashMap<String, OxigraphEndpoint> {
        &self.use_endpoints
    }

    pub fn endpoints(&self) -> &HashMap<String, OxigraphEndpoint> {
        &self.endpoints
    }

    pub fn use_endpoint(&mut self, name: &str, endpoint: OxigraphEndpoint) {
        self.use_endpoints.insert(name.to_string(), endpoint);
    }

    pub fn dont_use_endpoint(&mut self, name: &str) {
        self.use_endpoints.remove(name);
    }

    pub fn endpoints_to_use(&self) -> impl Iterator<Item = (&str, &OxigraphEndpoint)> {
        self.use_endpoints
            .iter()
            .map(|(name, endpoint)| (name.as_str(), endpoint))
    }

    pub fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        let str: String = format!("{bn}");
        format!("{}", str.green())
    }

    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        let str = match lit.clone().destruct() {
            (value, None, None, None) => format!("\"{}\"", value),
            (value, Some(dt), None, None) => format!("\"{}\"^^{}", value, self.qualify_iri(&dt)),
            (value, _, Some(lang), None) => format!("\"{}\"@{}", value, lang),
            (value, _, Some(lang), Some(direction)) => {
                format!("\"{}\"@{}{}", value, lang, direction)
            },
            _ => panic!("Unexpected literal structure <{}>", lit),
        };
        format!("{}", str.red())
    }

    pub fn serialize<W: io::Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), RdfDataError> {
        BuildRDF::serialize(&self.primary, format, writer).map_err(|e| RdfDataError::Serializing {
            format: *format,
            error: format!("{e}"),
        })?;
        for (name, e) in self.use_endpoints.iter() {
            writeln!(writer, "Endpoint {}: {}", name, e.iri())?
        }
        Ok(())
    }

    pub fn find_endpoint(&self, name: &str) -> Option<OxigraphEndpoint> {
        self.endpoints.get(name).cloned()
    }

    /// Gets all the triples from the primary backend and the active endpoints.
    /// Can be very expensive on remote backends.
    pub fn all_triples(&self) -> Result<Box<dyn Iterator<Item = OxTriple> + '_>, RdfDataError> {
        let primary_triples: Vec<OxTriple> = self
            .primary
            .triples()
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })?
            .collect();
        let endpoint_triples = self
            .use_endpoints
            .values()
            .flat_map(|e| NeighsRDF::triples(e))
            .flatten();
        Ok(Box::new(primary_triples.into_iter().chain(endpoint_triples)))
    }
}

impl Serialize for RdfData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("RdfData", 2)?;
        state.serialize_field("endpoints", &self.endpoints)?;
        // Only the in-memory backend has a meaningful serde representation
        // today; remote backends serialize as nothing.
        if let RdfBackend::InMemory(g) = &self.primary {
            state.serialize_field("graph", g)?;
        }
        state.end()
    }
}

impl Debug for RdfData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RdfData")
            .field("primary", &self.primary)
            .field("endpoints", &self.endpoints)
            .finish()
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

    fn prefixmap(&self) -> Option<PrefixMap> {
        let primary_pm = <RdfBackend as Rdf>::prefixmap(&self.primary);
        if self.use_endpoints.is_empty() {
            return primary_pm;
        }
        let mut pm = primary_pm.unwrap_or_default();
        for e in self.use_endpoints.values() {
            pm.merge(e.prefixmap().clone());
        }
        Some(pm)
    }

    fn qualify_iri(&self, node: &Self::IRI) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap_or_else(|_| IriS::new_unchecked(node.as_str()));
        // Try the primary's prefixmap, then each active endpoint's.
        if let Some(pm) = <RdfBackend as Rdf>::prefixmap(&self.primary)
            && let Some(q) = pm.qualify_optional(&iri)
        {
            return q;
        }
        for endpoint in self.use_endpoints.values() {
            if let Some(q) = endpoint.prefixmap().qualify_optional(&iri) {
                return q;
            }
        }
        format!("{node}")
    }

    fn qualify_subject(&self, subj: &Self::Subject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_iri(n),
        }
    }

    fn qualify_term(&self, term: &Self::Term) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, prefixmap::error::PrefixMapError> {
        if let Some(pm) = <RdfBackend as Rdf>::prefixmap(&self.primary)
            && let Ok(iri) = pm.resolve_prefix_local(prefix, local)
        {
            return Ok(iri);
        }
        for endpoint in self.use_endpoints.values() {
            if let Ok(iri) = endpoint.prefixmap().resolve_prefix_local(prefix, local) {
                return Ok(iri);
            }
        }
        Err(prefixmap::error::PrefixMapError::PrefixNotFound {
            prefix: prefix.to_string(),
            prefixmap: Box::new(PrefixMap::new()),
        })
    }
}

impl QueryRDF for RdfData {
    fn query_construct(&self, query_str: &str, format: &QueryResultFormat) -> Result<String, RdfDataError>
    where
        Self: Sized,
    {
        let mut out = self
            .primary
            .query_construct(query_str, format)
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })?;
        for (_name, endpoint) in self.endpoints_to_use() {
            let extra = endpoint.query_construct(query_str, format)?;
            out.push_str(&extra);
        }
        Ok(out)
    }

    fn query_select(&self, query_str: &str) -> Result<QuerySolutions<RdfData>, RdfDataError>
    where
        Self: Sized,
    {
        let mut sols: QuerySolutions<RdfData> = QuerySolutions::empty();

        // Primary backend.
        let primary_sols = self
            .primary
            .query_select(query_str)
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })?;
        let primary_pm = <RdfBackend as Rdf>::prefixmap(&self.primary).unwrap_or_default();
        let converted: Vec<QuerySolution<RdfData>> = primary_sols.iter().map(|s| s.convert(|t| t.clone())).collect();
        sols.extend(converted, primary_pm);

        // Federated endpoints.
        for (_, endpoint) in self.endpoints_to_use() {
            let new_sols = endpoint.query_select(query_str)?;
            let new_sols_converted: Vec<QuerySolution<RdfData>> =
                new_sols.iter().map(|s| s.convert(|t| t.clone())).collect();
            sols.extend(new_sols_converted, endpoint.prefixmap().clone());
        }
        Ok(sols)
    }

    fn query_ask(&self, query: &str) -> Result<bool, Self::Err> {
        if self
            .primary
            .query_ask(query)
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })?
        {
            return Ok(true);
        }
        for (_, endpoint) in self.endpoints_to_use() {
            if endpoint.query_ask(query)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl NeighsRDF for RdfData {
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        // Materialize the primary so the iterator is `'static`; otherwise we
        // can't chain with the endpoint iterators (which are also lazy).
        let primary: Vec<OxTriple> = self
            .primary
            .triples()
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })?
            .collect();
        let endpoint_triples: Vec<OxTriple> = self
            .use_endpoints
            .values()
            .flat_map(|e| NeighsRDF::triples(e).map(|i| i.collect::<Vec<_>>()).unwrap_or_default())
            .collect();
        let iter: Box<dyn Iterator<Item = OxTriple> + '_> = Box::new(primary.into_iter().chain(endpoint_triples));
        Ok(iter)
    }

    fn triples_matching<S, P, O>(
        &self,
        subject: &S,
        predicate: &P,
        object: &O,
    ) -> Result<impl Iterator<Item = Self::Triple> + '_, Self::Err>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        let primary: Vec<OxTriple> = self
            .primary
            .triples_matching(subject, predicate, object)
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })?
            .collect();
        let mut endpoint_triples: Vec<OxTriple> = Vec::new();
        for e in self.use_endpoints.values() {
            endpoint_triples.extend(e.triples_matching(subject, predicate, object)?);
        }
        Ok(primary.into_iter().chain(endpoint_triples))
    }

    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: &[Self::IRI],
    ) -> std::result::Result<
        (std::collections::HashMap<Self::IRI, std::collections::HashSet<Self::Term>>, Vec<Self::IRI>),
        Self::Err,
    > {
        if preds.is_empty() {
            return Ok((std::collections::HashMap::new(), Vec::new()));
        }
        // Primary backend (in-memory or endpoint-backed): uses its own FILTER query when available
        let (mut results, _) = self
            .primary
            .outgoing_arcs_from_list(subject, preds)
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })?;
        // Active SPARQL endpoints: each issues a single FILTER query for only the needed predicates
        for endpoint in self.use_endpoints.values() {
            let (ep_results, _) = endpoint.outgoing_arcs_from_list(subject, preds)?;
            for (pred, objects) in ep_results {
                results.entry(pred).or_default().extend(objects);
            }
        }
        // Remainder predicates are not fetched from SPARQL endpoints (expensive).
        // Closed-shape validation against live endpoints is not supported.
        Ok((results, Vec::new()))
    }
}

impl FocusRDF for RdfData {
    fn set_focus(&mut self, focus: &Self::Term) {
        self.primary.set_focus(focus);
    }

    fn get_focus(&self) -> Option<&Self::Term> {
        self.primary.get_focus()
    }
}

impl BuildRDF for RdfData {
    fn empty() -> Self {
        RdfData::new()
    }

    fn add_base(&mut self, base: &Option<IriS>) {
        self.primary.add_base(base);
    }

    fn add_prefix(&mut self, alias: &str, iri: &IriS) {
        self.primary.add_prefix(alias, iri);
    }

    fn set_prefix_map(&mut self, prefix_map: PrefixMap) {
        self.primary.set_prefix_map(prefix_map);
    }

    fn merge_prefixes(&mut self, prefix_map: PrefixMap) {
        self.primary.merge_prefixes(prefix_map);
    }

    fn add_triple<S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>,
    {
        self.primary
            .add_triple(subj, pred, obj)
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })
    }

    fn remove_triple<S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>,
    {
        self.primary
            .remove_triple(subj, pred, obj)
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })
    }

    fn add_type<S, T>(&mut self, node: S, type_: T) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        T: Into<Self::Term>,
    {
        self.primary
            .add_type(node, type_)
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })
    }

    fn serialize<W: std::io::Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), Self::Err> {
        BuildRDF::serialize(&self.primary, format, writer).map_err(|e| RdfDataError::Serializing {
            format: *format,
            error: format!("{e}"),
        })?;
        for (name, endpoint) in &self.endpoints {
            writeln!(writer, "Endpoint {}: {}", name, endpoint.iri())?;
        }
        Ok(())
    }

    fn add_bnode(&mut self) -> Result<Self::BNode, Self::Err> {
        self.primary
            .add_bnode()
            .map_err(|e| RdfDataError::Backend { err: Box::new(e) })
    }
}

#[cfg(test)]
mod tests {
    use rudof_iri::iri;

    use super::*;

    #[test]
    fn test_rdf_data_from_str() {
        let data = "<http://example.org/subject> <http://example.org/predicate> <http://example.org/object> .";
        let rdf_data = RdfData::from_str(data, &RDFFormat::NTriples, None, &ReaderMode::Lax);
        assert!(rdf_data.is_ok());
        let rdf_data = rdf_data.unwrap();
        assert_eq!(rdf_data.backend().as_in_memory().unwrap().triples().unwrap().count(), 1);
    }

    #[test]
    fn test_build_rdf_data() {
        let mut rdf_data = RdfData::new();
        rdf_data.add_prefix("ex", &IriS::from_str("http://example.org/").unwrap());
        rdf_data
            .add_triple(
                iri!("http://example.org/alice"),
                iri!("http://example.org/knows"),
                iri!("http://example.org/bob"),
            )
            .unwrap();
        assert_eq!(rdf_data.backend().as_in_memory().unwrap().triples().unwrap().count(), 1);
    }
}
