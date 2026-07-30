use crate::{
    rdf_core::{
        Any, AsyncRDF, Matcher, NeighsRDF, Rdf,
        query::{QueryRDF, QueryResultFormat, QuerySolution, QuerySolutions, VarName},
    },
    rdf_impl::OxigraphEndpointError,
};
use colored::*;
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, NamedOrBlankNode as OxSubject,
    Term as OxTerm, Triple as OxTriple,
};
use prefixmap::PrefixMap;
use regex::Regex;
use reqwest::header::{ACCEPT, HeaderMap, HeaderValue, USER_AGENT};
use rudof_iri::IriS;
use serde::{Serialize, ser::SerializeStruct};
use sparesults::{
    QueryResultsFormat, QueryResultsParser, QuerySolution as OxQuerySolution, ReaderQueryResultsParserOutput,
};
use std::collections::HashMap;
use std::{collections::HashSet, fmt::Display, hash::Hash, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use tracing::{debug, trace, warn};
use url::Url;

/// Type alias for Result with OxigraphEndpointError.
type Result<A> = std::result::Result<A, OxigraphEndpointError>;

/// A SPARQL endpoint client that implements the SRDF interface.
///
/// This struct provides methods for querying SPARQL endpoints with support for
/// different query types (SELECT, CONSTRUCT, ASK) and result formats.
///
/// # Performance
///
/// - Uses `Arc` for shared data (clients, prefix map) to enable cheap cloning
/// - Pre-allocates collections when size is known
/// - Caches HTTP clients with appropriate headers for each format
#[derive(Debug, Clone)]
pub struct OxigraphEndpoint {
    /// The IRI of the SPARQL endpoint.
    endpoint_iri: IriS,

    /// Prefix map for qualifying IRIs.
    prefixmap: Arc<PrefixMap>,

    /// HTTP client configured for SELECT queries (expects JSON results).
    client: Arc<reqwest::Client>,

    /// Cache of HTTP clients for CONSTRUCT queries
    construct_clients: Arc<RwLock<HashMap<QueryResultFormat, Arc<reqwest::Client>>>>,

    /// Proactive rate limiter: records when the last request was dispatched.
    /// Shared across all clones so concurrent callers coordinate on one endpoint.
    last_request_at: Arc<tokio::sync::Mutex<std::time::Instant>>,

    /// Per-subject predicate cache: subject → (predicate → objects).
    ///
    /// A predicate key present in the inner map (even with an empty set) means
    /// "already fetched — no SPARQL request needed". A missing key means
    /// "not yet fetched". This lets `outgoing_arcs_from_list` skip predicates
    /// that were already queried, so recurring references to the same entity
    /// (common in recursive ShEx schemas like E10/human) cost one SPARQL
    /// request instead of one per validation pass.
    triple_cache: Arc<std::sync::RwLock<HashMap<OxSubject, HashMap<OxNamedNode, HashSet<OxTerm>>>>>,
}

impl PartialEq for OxigraphEndpoint {
    /// Two endpoints are equal if they have the same IRI.
    ///
    /// Note: This compares only the endpoint IRI, not the prefix maps or clients.
    fn eq(&self, other: &Self) -> bool {
        self.endpoint_iri == other.endpoint_iri
    }
}

impl Hash for OxigraphEndpoint {
    /// Hash based on the endpoint IRI.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.endpoint_iri.hash(state);
    }
}

impl Eq for OxigraphEndpoint {}

impl Serialize for OxigraphEndpoint {
    /// Serialize only the endpoint IRI and prefix map.
    ///
    /// HTTP clients are not serialized as they cannot be meaningfully serialized.
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("SRDFSparql", 2)?;
        state.serialize_field("endpoint_iri", &self.endpoint_iri)?;
        state.serialize_field("prefixmap", self.prefixmap.as_ref())?;
        state.end()
    }
}

impl OxigraphEndpoint {
    /// Creates a new SPARQL endpoint with the given IRI and prefix map.
    ///
    /// This initializes HTTP clients with appropriate headers for each result format.
    ///
    /// # Arguments
    ///
    /// * `iri` - The IRI of the SPARQL endpoint
    /// * `prefixmap` - The prefix map for qualifying IRIs
    ///
    /// # Errors
    ///
    /// Returns an error if HTTP client creation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use rudof_rdf::rdf_impl::OxigraphEndpoint;
    /// use rudof_iri::IriS;
    /// use prefixmap::PrefixMap;
    ///
    /// let iri = IriS::new_unchecked("https://dbpedia.org/sparql");
    /// let prefixmap = PrefixMap::new();
    /// let endpoint = OxigraphEndpoint::new(&iri, &prefixmap);
    /// ```
    pub fn new(iri: &IriS, prefixmap: &PrefixMap) -> Result<OxigraphEndpoint> {
        let client = Arc::new(sparql_client()?);
        // Initialise to 1.1 s in the past so the first request fires immediately.
        let initial = std::time::Instant::now()
            .checked_sub(std::time::Duration::from_millis(1100))
            .unwrap_or_else(std::time::Instant::now);
        Ok(OxigraphEndpoint {
            endpoint_iri: iri.clone(),
            prefixmap: Arc::new(prefixmap.clone()),
            client,
            construct_clients: Arc::new(RwLock::new(HashMap::new())),
            last_request_at: Arc::new(tokio::sync::Mutex::new(initial)),
            triple_cache: Arc::new(std::sync::RwLock::new(HashMap::new())),
        })
    }

    /// Waits until at least 1.1 s have elapsed since the previous request, then
    /// stamps `last_request_at` with the current time.
    ///
    /// Holding the `tokio::sync::Mutex` across the `sleep` serialises callers:
    /// each one waits its turn, so bursts cannot exceed ~1 req/s regardless of
    /// how many async tasks are running against the same endpoint.
    async fn enforce_rate_limit(&self) {
        const MIN_INTERVAL: std::time::Duration = std::time::Duration::from_millis(1100);
        let mut last = self.last_request_at.lock().await;
        let elapsed = last.elapsed();
        if elapsed < MIN_INTERVAL {
            let wait = MIN_INTERVAL - elapsed;
            trace!(endpoint = %self.endpoint_iri, wait_ms = wait.as_millis(), "rate-limit: waiting before next request");
            tokio::time::sleep(wait).await;
        }
        *last = std::time::Instant::now();
    }

    /// Returns a reference to the endpoint IRI.
    pub fn iri(&self) -> &IriS {
        &self.endpoint_iri
    }

    /// Returns a reference to the prefix map.
    pub fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }

    /// Creates a SPARQL endpoint for Wikidata.
    ///
    /// This is a convenience method that creates an endpoint configured for
    /// the Wikidata Query Service at `https://query.wikidata.org/sparql`.
    ///
    /// # Errors
    ///
    /// Returns an error if HTTP client creation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use rudof_rdf::rdf_impl::OxigraphEndpoint;
    ///
    /// let wikidata = OxigraphEndpoint::wikidata();
    /// ```
    pub fn wikidata() -> Result<OxigraphEndpoint> {
        OxigraphEndpoint::new(
            &IriS::new_unchecked("https://query.wikidata.org/sparql"),
            &PrefixMap::wikidata(),
        )
    }

    /// Replaces the prefix map with a new one.
    ///
    /// This consumes self and returns a new endpoint with the updated prefix map.
    ///
    /// # Arguments
    ///
    /// * `pm` - The new prefix map
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rudof_rdf::rdf_impl::OxigraphEndpoint;
    /// use prefixmap::PrefixMap;
    ///
    /// let endpoint = OxigraphEndpoint::wikidata();
    /// let custom_prefixmap = PrefixMap::new();
    /// let endpoint = endpoint.unwrap().with_prefixmap(custom_prefixmap);
    /// ```
    pub fn with_prefixmap(mut self, pm: PrefixMap) -> OxigraphEndpoint {
        self.prefixmap = Arc::new(pm);
        self
    }

    /// Formats a blank node with color for display.
    ///
    /// This is an internal helper that applies green coloring to blank nodes.
    #[inline]
    fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        bn.to_string().green().to_string()
    }

    /// Formats a literal with color for display.
    ///
    /// This is a public helper that applies red coloring to literals.
    #[inline]
    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        lit.to_string().red().to_string()
    }

    /// Executes a SPARQL SELECT query asynchronously.
    ///
    /// This method works on both WASM and native platforms.
    ///
    /// # Arguments
    ///
    /// * `query` - The SPARQL SELECT query string
    ///
    /// # Returns
    ///
    /// Returns `QuerySolutions` containing the results of the query.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The response cannot be parsed as JSON
    /// - The JSON cannot be parsed as SPARQL results
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rudof_rdf::rdf_impl::OxigraphEndpoint;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let endpoint = OxigraphEndpoint::wikidata()?;
    ///     let query = "SELECT ?item WHERE { ?item wdt:P31 wd:Q5 } LIMIT 10";
    ///
    ///     let results = endpoint.query_select_async(query).await?;
    ///
    ///     // Assert that we got some solutions
    ///     assert!(results.count() > 0, "Expected at least one result");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_select_async(&self, query: &str) -> Result<QuerySolutions<Self>> {
        self.enforce_rate_limit().await;
        let solutions = make_sparql_query_select_async(query, &self.client, &self.endpoint_iri).await?;

        // Pre-allocate with known capacity for better performance
        let mut qs = Vec::with_capacity(solutions.len());
        for solution in &solutions {
            qs.push(cnv_query_solution(solution));
        }

        Ok(QuerySolutions::new(qs, (*self.prefixmap).clone()))
    }

    /// Executes a SPARQL CONSTRUCT query asynchronously.
    ///
    /// This method works on both WASM and native platforms.
    ///
    /// # Arguments
    ///
    /// * `query` - The SPARQL CONSTRUCT query string
    /// * `format` - The desired result format (Turtle, RDF/XML, or JSON-LD)
    ///
    /// # Returns
    ///
    /// Returns the query results as a string in the requested format.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The format is not supported
    /// - The HTTP request fails
    /// - The response cannot be read as text
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rudof_rdf::rdf_impl::OxigraphEndpoint;
    /// use rudof_rdf::rdf_core::query::QueryResultFormat;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let endpoint = OxigraphEndpoint::wikidata()?;
    ///     let query = "CONSTRUCT { ?s ?p ?o } WHERE { ?s ?p ?o } LIMIT 10";
    ///
    ///     let turtle = endpoint
    ///         .query_construct_async(query, &QueryResultFormat::Turtle)
    ///         .await?;
    ///
    ///     // Assert that we got some RDF content
    ///     assert!(!turtle.trim().is_empty(), "Expected non-empty Turtle output");
    ///
    ///     // Very lightweight sanity check for Turtle syntax
    ///     assert!(
    ///         turtle.contains('.') || turtle.contains("@prefix"),
    ///         "Output does not look like valid Turtle"
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_construct_async(&self, query: &str, format: &QueryResultFormat) -> Result<String> {
        self.enforce_rate_limit().await;
        let client = self.get_construct_client(format).await?;
        make_sparql_query_construct_async(query, &client, &self.endpoint_iri, format).await
    }

    /// Retrieves or creates an HTTP client for the given `[[QueryResultFormat]]`
    async fn get_construct_client(&self, format: &QueryResultFormat) -> Result<Arc<reqwest::Client>> {
        {
            let map = self.construct_clients.read().await;
            if let Some(client) = map.get(format) {
                return Ok(client.clone());
            }
        }
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static(format.mime_type()));
        headers.insert(USER_AGENT, HeaderValue::from_static("rudof"));

        let client = reqwest::Client::builder().default_headers(headers).build()?;
        let client = Arc::new(client);

        let mut map = self.construct_clients.write().await;
        Ok(map.entry(format.clone()).or_insert_with(|| client.clone()).clone())
    }

    /// Executes a SPARQL ASK query asynchronously.
    ///
    /// This method works on both WASM and native platforms.
    ///
    /// # Arguments
    ///
    /// * `query` - The SPARQL ASK query string
    ///
    /// # Returns
    ///
    /// Returns `true` if the query pattern matches, `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The response cannot be parsed
    /// - The response is not a valid boolean value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rudof_rdf::rdf_impl::OxigraphEndpoint;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let endpoint = OxigraphEndpoint::wikidata()?;
    ///
    ///     // This should always be true: Wikidata has triples
    ///     let query = "ASK { ?s ?p ?o }";
    ///     let exists = endpoint.query_ask_async(query).await?;
    ///
    ///     assert!(exists, "Expected ASK query to return true");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_ask_async(&self, query: &str) -> Result<bool> {
        self.enforce_rate_limit().await;
        make_sparql_query_ask_async(query, &self.client, &self.endpoint_iri).await
    }
}

impl FromStr for OxigraphEndpoint {
    type Err = OxigraphEndpointError;

    /// Parses a SPARQL endpoint from a string.
    ///
    /// Supports two formats:
    /// - IRI in angle brackets: `<https://example.org/sparql>`
    /// - Predefined endpoint name: `wikidata`
    ///
    /// # Performance
    ///
    /// Uses a cached regex (via `once_cell::Lazy`) to avoid recompiling
    /// the pattern on each call.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use once_cell::sync::Lazy;
        // Regex is compiled once and cached for all future calls
        static RE_IRI: Lazy<Regex> = Lazy::new(|| Regex::new(r"<(.*)>").unwrap());

        if let Some(iri_str) = RE_IRI.captures(s) {
            // Parse IRI from angle brackets
            let iri_s = IriS::from_str(&iri_str[1])?;
            let client = Arc::new(sparql_client()?);
            let initial = std::time::Instant::now()
                .checked_sub(std::time::Duration::from_millis(1100))
                .unwrap_or_else(std::time::Instant::now);
            Ok(OxigraphEndpoint {
                endpoint_iri: iri_s,
                prefixmap: Arc::new(PrefixMap::new()),
                client,
                construct_clients: Arc::new(RwLock::new(HashMap::new())),
                last_request_at: Arc::new(tokio::sync::Mutex::new(initial)),
                triple_cache: Arc::new(std::sync::RwLock::new(HashMap::new())),
            })
        } else {
            // Try to match predefined endpoint names
            match s.to_lowercase().as_str() {
                "wikidata" => OxigraphEndpoint::wikidata(),
                name => Err(OxigraphEndpointError::UnknownEndpointName { name: name.to_string() }),
            }
        }
    }
}

impl Rdf for OxigraphEndpoint {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Triple = OxTriple;
    type Err = OxigraphEndpointError;

    /// Resolves a prefix and local name to a full IRI.
    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> std::result::Result<IriS, prefixmap::PrefixMapError> {
        self.prefixmap.resolve_prefix_local(prefix, local)
    }

    /// Qualifies an IRI using the prefix map.
    ///
    /// Converts full IRIs to their prefixed form (e.g., `rdf:type`).
    fn qualify_iri(&self, node: &OxNamedNode) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap();
        self.prefixmap.qualify(&iri)
    }

    /// Qualifies a subject (named node or blank node) for display.
    fn qualify_subject(&self, subj: &OxSubject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_iri(n),
        }
    }

    /// Qualifies a term (IRI, blank node, or literal) for display.
    fn qualify_term(&self, term: &OxTerm) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
            OxTerm::Triple(_) => unimplemented!("Triple terms not yet supported"),
        }
    }

    /// Returns the prefix map for this endpoint.
    fn prefixmap(&self) -> Option<PrefixMap> {
        Some((*self.prefixmap).clone())
    }
}

impl AsyncRDF for OxigraphEndpoint {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = OxigraphEndpointError;

    /// Retrieves all predicates for a given subject.
    ///
    /// Executes a SPARQL query: `SELECT ?pred WHERE { <subject> ?pred ?obj }`
    ///
    /// # Performance
    ///
    /// Pre-allocates the HashSet with capacity based on the number of results.
    async fn get_predicates_subject(&self, subject: &OxSubject) -> Result<HashSet<OxNamedNode>> {
        let query = format!(r#"select ?pred where {{ {subject} ?pred ?obj . }}"#);
        let solutions = make_sparql_query_select_async(&query, &self.client, &self.endpoint_iri).await?;

        let mut results = HashSet::with_capacity(solutions.len());
        for solution in solutions {
            let n = get_iri_solution(&solution, "pred")?;
            results.insert(n);
        }
        Ok(results)
    }

    /// Retrieves all objects for a given subject-predicate pair.
    ///
    /// Executes a SPARQL query: `SELECT ?obj WHERE { <subject> <pred> ?obj }`
    async fn get_objects_for_subject_predicate(
        &self,
        subject: &OxSubject,
        pred: &OxNamedNode,
    ) -> Result<HashSet<OxTerm>> {
        let query = format!(r#"select ?obj where {{ {subject} {pred} ?obj . }}"#);
        let solutions = make_sparql_query_select_async(&query, &self.client, &self.endpoint_iri).await?;

        let mut results = HashSet::with_capacity(solutions.len());
        for solution in solutions {
            if let Some(obj) = solution.get("obj") {
                results.insert(obj.clone());
            }
        }
        Ok(results)
    }

    /// Retrieves all subjects for a given predicate-object pair.
    ///
    /// Executes a SPARQL query: `SELECT ?subj WHERE { ?subj <pred> <object> }`
    async fn get_subjects_for_object_predicate(
        &self,
        object: &OxTerm,
        pred: &OxNamedNode,
    ) -> Result<HashSet<OxSubject>> {
        let query = format!(r#"select ?subj where {{ ?subj {pred} {object} . }}"#);
        let solutions = make_sparql_query_select_async(&query, &self.client, &self.endpoint_iri).await?;

        let mut results = HashSet::with_capacity(solutions.len());
        for solution in solutions {
            if let Some(OxTerm::NamedNode(n)) = solution.get("subj") {
                results.insert(OxSubject::NamedNode(n.clone()));
            } else if let Some(OxTerm::BlankNode(bn)) = solution.get("subj") {
                results.insert(OxSubject::BlankNode(bn.clone()));
            }
        }
        Ok(results)
    }
}

// NeighsRDF is only available on non-WASM platforms because it requires
// synchronous iteration, which is not possible in WASM environments
#[cfg(not(target_family = "wasm"))]
impl NeighsRDF for OxigraphEndpoint {
    /// Returns an iterator over all triples in the endpoint.
    ///
    /// This is equivalent to `SELECT * WHERE { ?s ?p ?o }`.
    ///
    /// Note: This can be very expensive for large endpoints.
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>> {
        self.triples_matching(&Any, &Any, &Any)
    }

    /// Returns an iterator over triples matching the given pattern.
    ///
    /// # Arguments
    ///
    /// * `subject` - Subject matcher (use `Any` to match all)
    /// * `predicate` - Predicate matcher (use `Any` to match all)
    /// * `object` - Object matcher (use `Any` to match all)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rudof_rdf::rdf_impl::OxigraphEndpoint;
    /// use rudof_rdf::rdf_core::{Any, NeighsRDF};
    /// use oxrdf::NamedNode;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let endpoint = OxigraphEndpoint::wikidata()?;
    ///
    ///     let predicate = NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?;
    ///     let mut triples = endpoint.triples_matching(&Any, &predicate, &Any)?;
    ///
    ///     // Take the first triple, if any
    ///     let first = triples.next();
    ///
    ///     assert!(first.is_some(), "Expected at least one triple from the endpoint");
    ///
    ///     Ok(())
    /// }
    /// ```
    fn triples_matching<S, P, O>(
        &self,
        subject: &S,
        predicate: &P,
        object: &O,
    ) -> Result<impl Iterator<Item = Self::Triple> + '_>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        // Build SPARQL query from matchers, only projecting wildcard positions
        let s_str = subject
            .value()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "?s".to_string());
        let p_str = predicate
            .value()
            .map(|p| p.to_string())
            .unwrap_or_else(|| "?p".to_string());
        let o_str = object
            .value()
            .map(|o| o.to_string())
            .unwrap_or_else(|| "?o".to_string());

        let mut select_vars = Vec::new();
        if subject.value().is_none() {
            select_vars.push("?s");
        }
        if predicate.value().is_none() {
            select_vars.push("?p");
        }
        if object.value().is_none() {
            select_vars.push("?o");
        }
        // SELECT * is valid when all positions are bound (returns one empty row if the triple exists)
        let select_clause = if select_vars.is_empty() {
            "*".to_string()
        } else {
            select_vars.join(" ")
        };

        let query = format!("SELECT {} WHERE {{ {} {} {} }}", select_clause, s_str, p_str, o_str);

        let solutions = self.query_select(&query)?;

        // Clone matcher values for use in the closure
        let subject_val = subject.value().cloned();
        let predicate_val = predicate.value().cloned();
        let object_val = object.value().cloned();

        // Build iterator that converts query solutions to triples, using named variable lookup
        let triples = solutions.into_iter().filter_map(move |solution| {
            let subject_res: Self::Subject = match &subject_val {
                Some(s) => s.clone(),
                None => solution.find_solution("s").and_then(|s| s.clone().try_into().ok())?,
            };
            let predicate_res: Self::IRI = match &predicate_val {
                Some(p) => p.clone(),
                None => solution
                    .find_solution("p")
                    .and_then(|pred| pred.clone().try_into().ok())?,
            };
            let object_res = match &object_val {
                Some(o) => o.clone(),
                None => solution.find_solution("o")?.clone(),
            };
            Some(OxTriple::new(subject_res, predicate_res, object_res))
        });

        Ok(triples)
    }

    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: &[Self::IRI],
    ) -> Result<(HashMap<Self::IRI, HashSet<Self::Term>>, Vec<Self::IRI>)> {
        if preds.is_empty() {
            return Ok((HashMap::new(), Vec::new()));
        }

        // --- Cache read pass ---
        // Separate the requested predicates into those already cached and those
        // that still need a SPARQL request.
        let mut results: HashMap<OxNamedNode, HashSet<OxTerm>> = HashMap::new();
        let mut uncached: Vec<&OxNamedNode> = Vec::new();
        {
            let cache = self.triple_cache.read().unwrap();
            if let Some(subject_data) = cache.get(subject) {
                for pred in preds {
                    if let Some(objects) = subject_data.get(pred) {
                        // Predicate is cached (even if the object set is empty).
                        results.entry(pred.clone()).or_default().extend(objects.iter().cloned());
                    } else {
                        uncached.push(pred);
                    }
                }
            } else {
                uncached.extend(preds.iter());
            }
        }

        if uncached.is_empty() {
            trace!(subject = %subject, "outgoing_arcs_from_list: all {} preds from cache", preds.len());
            return Ok((results, Vec::new()));
        }

        // --- SPARQL fetch for uncached predicates ---
        let filter_in = uncached.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");
        let query = format!(
            "SELECT ?p ?o WHERE {{ {} ?p ?o FILTER(?p IN ({})) }}",
            subject, filter_in
        );

        trace!(
            subject = %subject,
            cached = preds.len() - uncached.len(),
            fetching = uncached.len(),
            %query,
            "outgoing_arcs_from_list FILTER query"
        );

        let solutions = self.query_select(&query)?;

        // --- Cache write pass ---
        // Write back all fetched predicates, including those with no results
        // (so we don't re-query them on the next call).
        let mut cache = self.triple_cache.write().unwrap();
        let subject_entry = cache.entry(subject.clone()).or_default();
        // Pre-insert all fetched preds with empty sets to mark them as "queried".
        for pred in &uncached {
            subject_entry.entry((*pred).clone()).or_default();
        }
        // Fill in actual values.
        for solution in solutions.into_iter() {
            let Some(p_term) = solution.find_solution("p") else {
                continue;
            };
            let p: OxNamedNode = match p_term.clone().try_into() {
                Ok(n) => n,
                Err(_) => continue,
            };
            let Some(o) = solution.find_solution("o").cloned() else {
                continue;
            };
            subject_entry.entry(p.clone()).or_default().insert(o.clone());
            results.entry(p).or_default().insert(o);
        }

        // Remainder predicates (those not in `preds`) are not fetched.
        // Closed-shape validation against live endpoints is not supported.
        Ok((results, Vec::new()))
    }
}

// Shared tokio runtime used by the blocking SPARQL methods.
#[cfg(not(target_family = "wasm"))]
static SPARQL_RUNTIME: once_cell::sync::Lazy<tokio::runtime::Runtime> = once_cell::sync::Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("rudof-sparql")
        .build()
        .expect("failed to build shared tokio runtime for SPARQL queries")
});

// QueryRDF is only available on non-WASM platforms.
// On native platforms, these sync methods bridge to the async implementations
// through a shared tokio runtime (see `SPARQL_RUNTIME`).
#[cfg(not(target_family = "wasm"))]
impl QueryRDF for OxigraphEndpoint {
    /// Executes a SPARQL CONSTRUCT query synchronously.
    ///
    /// This is a blocking wrapper around `query_construct_async`.
    fn query_construct(&self, query: &str, format: &QueryResultFormat) -> Result<String> {
        SPARQL_RUNTIME.block_on(self.query_construct_async(query, format))
    }

    /// Executes a SPARQL SELECT query synchronously.
    ///
    /// This is a blocking wrapper around `query_select_async`.
    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>> {
        SPARQL_RUNTIME.block_on(self.query_select_async(query))
    }

    /// Executes a SPARQL ASK query synchronously.
    ///
    /// This is a blocking wrapper around `query_ask_async`.
    fn query_ask(&self, query: &str) -> Result<bool> {
        SPARQL_RUNTIME.block_on(self.query_ask_async(query))
    }
}

/// Converts an oxrdf QuerySolution to our QuerySolution type.
///
/// # Performance
///
/// Uses iterators with `collect()` for efficient conversion.
#[inline]
fn cnv_query_solution(qs: &OxQuerySolution) -> QuerySolution<OxigraphEndpoint> {
    let vars: Vec<_> = qs.variables().iter().map(|v| VarName::new(v.as_str())).collect();
    let vals: Vec<_> = qs.values().to_vec();
    QuerySolution::new(vars, vals)
}

/// Creates an HTTP client configured for SPARQL SELECT queries.
///
/// Sets the Accept header to `application/sparql-results+json` and
/// includes a custom User-Agent.
///
/// # Errors
///
/// Returns an error if the client builder fails (e.g., TLS initialization fails).
fn sparql_client() -> Result<reqwest::Client> {
    use reqwest::header::{self, ACCEPT, USER_AGENT};

    let mut headers = header::HeaderMap::new();
    headers.insert(
        ACCEPT,
        header::HeaderValue::from_static("application/sparql-results+json"),
    );
    headers.insert(USER_AGENT, header::HeaderValue::from_static("rudof"));

    let client = reqwest::Client::builder().default_headers(headers).build()?;
    Ok(client)
}

/// Sends an HTTP GET request to a SPARQL endpoint, retrying on 429 (Too Many Requests).
///
/// Wikidata and other public endpoints enforce rate limits. When a 429 is received this
/// function waits for the duration indicated by the `Retry-After` header (falling back to
/// exponential backoff starting at 1 s) and retries up to `MAX_RETRIES` times before
/// propagating the error.
async fn sparql_get_with_retry(client: &reqwest::Client, url: &Url) -> Result<String> {
    // The proactive rate limiter in `enforce_rate_limit` already prevents most 429s.
    // These retries handle the rare cases where bursts still slip through.
    const MAX_RETRIES: u32 = 3;
    debug!(url = %url, "SPARQL request");
    for retry in 0..=MAX_RETRIES {
        trace!(url = %url, retry, "SPARQL GET attempt");
        let response = client.get(url.as_str()).send().await?;
        let status = response.status();
        trace!(url = %url, status = %status, "SPARQL response");

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            if retry == MAX_RETRIES {
                warn!(url = %url, "SPARQL 429: max retries reached, giving up");
                return Err(response.error_for_status().unwrap_err().into());
            }
            // Honour Retry-After if present; cap at 5 s to avoid hanging indefinitely.
            let delay_secs = response
                .headers()
                .get("Retry-After")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(1)
                .min(5);
            warn!(url = %url, delay_secs, retry, "SPARQL 429: retrying after delay");
            tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
            continue;
        }

        return response.error_for_status()?.text().await.map_err(Into::into);
    }
    unreachable!()
}

async fn make_sparql_query_select_async(
    query_str: &str,
    client: &reqwest::Client,
    endpoint_iri: &IriS,
) -> Result<Vec<OxQuerySolution>> {
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query_str)])?;
    let body = sparql_get_with_retry(client, &url).await?;
    parse_sparql_json_results(&body)
}

async fn make_sparql_query_construct_async(
    query: &str,
    client: &reqwest::Client,
    endpoint_iri: &IriS,
    _format: &QueryResultFormat,
) -> Result<String> {
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;
    sparql_get_with_retry(client, &url).await
}

async fn make_sparql_query_ask_async(query: &str, client: &reqwest::Client, endpoint_iri: &IriS) -> Result<bool> {
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;
    let body = sparql_get_with_retry(client, &url).await?;
    parse_sparql_ask_results(&body)
}

/// Parses SPARQL ASK query JSON results.
///
/// ASK queries return JSON in the format: `{"head": {}, "boolean": true}`
///
/// # Arguments
///
/// * `body` - The JSON response body as a string
///
/// # Returns
///
/// The boolean value from the response.
///
/// # Errors
///
/// Returns an error if:
/// - The JSON cannot be parsed
/// - The response is not a boolean result
fn parse_sparql_ask_results(body: &str) -> Result<bool> {
    let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);

    match json_parser.for_reader(body.as_bytes())? {
        ReaderQueryResultsParserOutput::Boolean(b) => Ok(b),
        _ => Err(OxigraphEndpointError::ParsingBody {
            body: format!("Expected boolean ASK result, got: {}", body),
        }),
    }
}

/// Parses SPARQL JSON results into a vector of query solutions.
///
/// # Arguments
///
/// * `body` - The JSON response body as a string
///
/// # Returns
///
/// A vector of query solutions.
///
/// # Errors
///
/// Returns an error if:
/// - The JSON cannot be parsed
/// - The JSON is not a valid SPARQL results format
/// - Individual solutions cannot be parsed
fn parse_sparql_json_results(body: &str) -> Result<Vec<OxQuerySolution>> {
    let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);

    if let ReaderQueryResultsParserOutput::Solutions(solutions) = json_parser.for_reader(body.as_bytes())? {
        // Collect all solutions, propagating any parsing errors
        solutions
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| OxigraphEndpointError::ParsingBody {
                body: format!("Error parsing solution: {}", e),
            })
    } else {
        Err(OxigraphEndpointError::ParsingBody { body: body.to_string() })
    }
}

/// Utility struct for displaying SPARQL variable lists.
///
/// This is used for debugging and error messages.
#[derive(Debug)]
pub struct SparqlVars {
    /// The list of variable names.
    values: Vec<String>,
}

impl Display for SparqlVars {
    /// Formats the variable list as a comma-separated string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.values.join(", "))
    }
}

/// Extracts a named IRI from a query solution.
///
/// # Arguments
///
/// * `solution` - The query solution
/// * `name` - The variable name to extract
///
/// # Returns
///
/// The named node if found and is an IRI.
///
/// # Errors
///
/// Returns an error if:
/// - The variable is not found in the solution
/// - The value is not a named node (IRI)
fn get_iri_solution(solution: &OxQuerySolution, name: &str) -> Result<OxNamedNode> {
    solution
        .get(name)
        .ok_or_else(|| OxigraphEndpointError::NotFoundInSolution {
            value: name.to_string(),
            solution: format!("{solution:?}"),
        })
        .and_then(|v| match v {
            OxTerm::NamedNode(n) => Ok(n.clone()),
            _ => Err(OxigraphEndpointError::SPARQLSolutionErrorNoIRI { value: v.clone() }),
        })
}
