use super::http::{self, HttpClient, Instant, SendError};
use crate::{
    cancellation,
    rdf_core::{
        Any, AsyncRDF, Matcher, NeighsRDF, Rdf,
        query::{QueryRDF, QueryResultFormat, QuerySolution, QuerySolutions, VarName},
    },
    rdf_impl::{EndpointStrategy, OxigraphEndpointError},
};
use colored::*;
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, NamedOrBlankNode as OxSubject,
    Term as OxTerm, Triple as OxTriple,
};
use prefixmap::PrefixMap;
use regex::Regex;
use rudof_iri::IriS;
use serde::{Serialize, ser::SerializeStruct};
use sparesults::{
    QueryResultsFormat, QueryResultsParser, QuerySolution as OxQuerySolution, ReaderQueryResultsParserOutput,
};
use std::collections::HashMap;
use std::{collections::HashSet, fmt::Display, hash::Hash, str::FromStr, sync::Arc};
use tracing::{debug, trace, warn};
use url::Url;

/// Type alias for Result with OxigraphEndpointError.
type Result<A> = std::result::Result<A, OxigraphEndpointError>;

impl FromStr for EndpointStrategy {
    type Err = OxigraphEndpointError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "sparql" => Ok(EndpointStrategy::Sparql),
            "dereference" => Ok(EndpointStrategy::Dereference),
            other => Err(OxigraphEndpointError::UnknownEndpointStrategy {
                name: other.to_string(),
            }),
        }
    }
}

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
    client: HttpClient,

    /// Cache of HTTP clients for CONSTRUCT queries
    construct_clients: Arc<std::sync::RwLock<HashMap<QueryResultFormat, HttpClient>>>,

    /// Proactive rate limiter: the earliest time the next request may be
    /// dispatched (`None` before the first request). Shared across all clones
    /// so concurrent callers coordinate on one endpoint.
    next_request_at: Arc<std::sync::Mutex<Option<Instant>>>,

    /// Per-subject predicate cache: subject → (predicate → objects).
    ///
    /// A predicate key present in the inner map (even with an empty set) means
    /// "already fetched — no SPARQL request needed". A missing key means
    /// "not yet fetched". This lets `outgoing_arcs_from_list` skip predicates
    /// that were already queried, so recurring references to the same entity
    /// (common in recursive ShEx schemas like E10/human) cost one SPARQL
    /// request instead of one per validation pass.
    triple_cache: Arc<std::sync::RwLock<HashMap<OxSubject, HashMap<OxNamedNode, HashSet<OxTerm>>>>>,

    /// Per-object predicate cache for inverse (incoming) arcs: object → (predicate → subjects).
    ///
    /// Mirrors `triple_cache` but for the reverse direction, so `incoming_arcs_from_list`
    /// can issue a predicate-scoped query instead of falling back to the trait default,
    /// which fetches *every* triple pointing at the object with no predicate restriction
    /// — unbounded for heavily-linked nodes on a live endpoint.
    incoming_triple_cache: Arc<std::sync::RwLock<HashMap<OxTerm, HashMap<OxNamedNode, HashSet<OxSubject>>>>>,

    /// Count of requests dispatched to this endpoint, shared across clones.
    /// Used only to emit a periodic `debug!` progress line — at roughly one
    /// request per second (see `rate_limit_ms`), validating a highly-connected
    /// node can silently run for minutes otherwise, with nothing telling the
    /// user it's still working.
    request_count: Arc<std::sync::atomic::AtomicU64>,

    /// Minimum interval (in milliseconds) `enforce_rate_limit` waits between
    /// requests to this endpoint, shared across clones. Starts at
    /// [`INITIAL_RATE_LIMIT_MS`] and is raised by [`http_get_with_retry`]
    /// whenever the server responds `429 Too Many Requests` — Wikidata's
    /// query service buckets clients by (User-Agent, IP) and throttles ones
    /// that don't back off, up to a temporary ban for those that keep
    /// hammering it after a 429. Backing off *before* seeing another 429,
    /// not just honouring `Retry-After` on the request that got one, is what
    /// keeps the rest of a long validation run from repeating the same
    /// throttle-wait-throttle cycle.
    rate_limit_ms: Arc<std::sync::atomic::AtomicU64>,

    /// How this endpoint answers `NeighsRDF` lookups: SPARQL queries (the
    /// default) or dereferencing entity IRIs over plain HTTP. See
    /// [`EndpointStrategy`].
    strategy: EndpointStrategy,

    /// HTTP client used only by [`EndpointStrategy::Dereference`]: sends
    /// `Accept: text/turtle` instead of the SPARQL-results JSON `client` uses.
    dereference_client: HttpClient,

    /// Cache for [`EndpointStrategy::Dereference`]: subject → (predicate → objects),
    /// accumulated from every entity document fetched so far.
    ///
    /// Dereferencing one entity's IRI returns more than that entity's own
    /// triples — for a Wikibase entity, its full statement/qualifier/reference
    /// structure, *and* a label-only stub (`rdf:type`, `rdfs:label`, ...) for
    /// every other entity it references as a value. All of it gets merged in
    /// here under its own subject, since it's free — no extra request needed
    /// to have it available if something else asks. But a stub is not the
    /// referenced entity's own document: it has no idea what that entity's
    /// *other* predicates are, so its presence here does *not* mean "fetched,
    /// no re-fetch needed" the way it does for `triple_cache`'s SPARQL-scoped
    /// entries. `dereferenced_subjects` is what actually tracks that.
    ///
    /// This is also what backs `incoming_arcs_from_list`'s and
    /// `triples_matching`'s dereference-mode behaviour: a linear scan over
    /// whatever has been dereferenced so far. Both are necessarily partial —
    /// dereferencing a node only ever yields its *outgoing* triples, so an
    /// inverse or wildcard lookup can only see links from entities this
    /// endpoint has already visited, never the full picture a SPARQL index
    /// would give.
    dereference_cache: Arc<std::sync::RwLock<HashMap<OxSubject, HashMap<OxNamedNode, HashSet<OxTerm>>>>>,

    /// Subjects that have actually been the *target* of a dereference request
    /// (as opposed to merely showing up as a referenced-value stub in some
    /// other subject's document — see `dereference_cache`'s doc comment).
    /// `outgoing_arcs_from_list_dereference` checks this, not
    /// `dereference_cache`, to decide whether a subject still needs fetching.
    dereferenced_subjects: Arc<std::sync::RwLock<HashSet<OxSubject>>>,
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
        Ok(OxigraphEndpoint {
            endpoint_iri: iri.clone(),
            prefixmap: Arc::new(prefixmap.clone()),
            client: HttpClient::new(SPARQL_RESULTS_JSON)?,
            construct_clients: Arc::new(std::sync::RwLock::new(HashMap::new())),
            next_request_at: Arc::new(std::sync::Mutex::new(None)),
            triple_cache: Arc::new(std::sync::RwLock::new(HashMap::new())),
            incoming_triple_cache: Arc::new(std::sync::RwLock::new(HashMap::new())),
            request_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            rate_limit_ms: Arc::new(std::sync::atomic::AtomicU64::new(INITIAL_RATE_LIMIT_MS)),
            strategy: EndpointStrategy::default(),
            // `Accept: text/turtle`: Wikibase entity IRIs content-negotiate a
            // full RDF description of the entity from this.
            dereference_client: HttpClient::new("text/turtle")?,
            dereference_cache: Arc::new(std::sync::RwLock::new(HashMap::new())),
            dereferenced_subjects: Arc::new(std::sync::RwLock::new(HashSet::new())),
        })
    }

    /// Selects how this endpoint answers lookups — SPARQL queries or HTTP
    /// dereferencing. See [`EndpointStrategy`].
    pub fn with_strategy(mut self, strategy: EndpointStrategy) -> OxigraphEndpoint {
        self.strategy = strategy;
        self
    }

    /// Returns the strategy this endpoint currently uses to answer lookups.
    pub fn strategy(&self) -> EndpointStrategy {
        self.strategy
    }

    /// Waits until at least [`Self::rate_limit_ms`]'s current value has elapsed
    /// since the previous request.
    ///
    /// Each caller reserves the next free slot in `next_request_at` before
    /// sleeping until it, which serialises callers: each one waits its turn, so
    /// bursts cannot exceed the current rate regardless of how many async tasks
    /// are running against the same endpoint.
    async fn enforce_rate_limit(&self) -> Result<()> {
        let min_interval =
            std::time::Duration::from_millis(self.rate_limit_ms.load(std::sync::atomic::Ordering::Relaxed));
        let wait = {
            let mut next = self.next_request_at.lock().unwrap();
            let now = Instant::now();
            let slot = match *next {
                Some(next) if next > now => next,
                _ => now,
            };
            *next = Some(slot + min_interval);
            slot.saturating_duration_since(now)
        };
        if !wait.is_zero() {
            trace!(endpoint = %self.endpoint_iri, wait_ms = wait.as_millis(), "rate-limit: waiting before next request");
            http::sleep(wait).await?;
        }

        // At ~1 req/s this can run for minutes against a highly-connected node
        // (e.g. a Wikidata item with thousands of statements) with nothing on
        // screen in the meantime. Surface periodic progress at `debug` level —
        // quiet by default, opt in with `RUST_LOG=debug` or (inside the shell,
        // without restarting) `config set logging.level debug`.
        const PROGRESS_EVERY: u64 = 10;
        let count = self.request_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        if count == 1 || count.is_multiple_of(PROGRESS_EVERY) {
            debug!(endpoint = %self.endpoint_iri, requests_sent = count, "Querying remote SPARQL endpoint...");
        }
        Ok(())
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
        if self.strategy == EndpointStrategy::Dereference {
            return Err(OxigraphEndpointError::UnsupportedForDereferenceStrategy { operation: "SELECT" });
        }
        if cancellation::is_cancelled() {
            return Err(OxigraphEndpointError::Cancelled);
        }
        self.enforce_rate_limit().await?;
        let solutions =
            make_sparql_query_select_async(query, &self.client, &self.endpoint_iri, &self.rate_limit_ms).await?;

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
        if self.strategy == EndpointStrategy::Dereference {
            return Err(OxigraphEndpointError::UnsupportedForDereferenceStrategy { operation: "CONSTRUCT" });
        }
        if cancellation::is_cancelled() {
            return Err(OxigraphEndpointError::Cancelled);
        }
        self.enforce_rate_limit().await?;
        let client = self.get_construct_client(format)?;
        make_sparql_query_construct_async(query, &client, &self.endpoint_iri, format, &self.rate_limit_ms).await
    }

    /// Retrieves or creates an HTTP client for the given `[[QueryResultFormat]]`
    fn get_construct_client(&self, format: &QueryResultFormat) -> Result<HttpClient> {
        if let Some(client) = self.construct_clients.read().unwrap().get(format) {
            return Ok(client.clone());
        }
        let client = HttpClient::new(format.mime_type())?;
        let mut map = self.construct_clients.write().unwrap();
        Ok(map.entry(format.clone()).or_insert(client).clone())
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
        if self.strategy == EndpointStrategy::Dereference {
            return Err(OxigraphEndpointError::UnsupportedForDereferenceStrategy { operation: "ASK" });
        }
        if cancellation::is_cancelled() {
            return Err(OxigraphEndpointError::Cancelled);
        }
        self.enforce_rate_limit().await?;
        make_sparql_query_ask_async(query, &self.client, &self.endpoint_iri, &self.rate_limit_ms).await
    }

    /// Dereferences `subject`'s IRI over HTTP (`Accept: text/turtle`, following
    /// redirects), parses the response as Turtle,
    /// and merges every triple it contains into `dereference_cache` (not just
    /// ones about `subject` — a Wikibase entity's document nests its
    /// statement/qualifier/reference structure, plus label-only stubs for
    /// every entity it references, under the same response, so this is what
    /// lets later lookups for those nested/referenced subjects reuse
    /// whatever's already at hand instead of starting from nothing).
    ///
    /// Always marks `subject` in `dereferenced_subjects` before returning,
    /// including when there was nothing to fetch or nothing usable came
    /// back: a blank node has no IRI to dereference, and plenty of subjects
    /// that show up during validation (statement/reference node IRIs not
    /// reachable as a standalone document) simply have no representation of
    /// their own. Both are real, final answers — "this subject has no
    /// outgoing triples reachable this way" — not "not tried yet", so a
    /// later call for the same subject must not retry the request.
    ///
    /// Also marks every *other* subject the response carried real data for —
    /// not just a [`STUB_ONLY_PREDICATES`]-shaped referenced-value stub — as
    /// dereferenced too. A Wikibase entity's statement/qualifier/reference
    /// nodes come back complete in the same response (see above), so a
    /// statement node reached this way later needs no HTTP request of its
    /// own: it would just redirect back to this same entity document.
    /// Without this, validating a well-connected node re-fetched that whole
    /// (potentially multi-thousand-triple) document once per statement.
    async fn dereference_subject(&self, subject: &OxSubject) -> Result<()> {
        let mark_done = |triples: Vec<OxTriple>| {
            let touched_subjects: HashSet<OxSubject> = triples.iter().map(|t| t.subject.clone()).collect();

            let mut cache = self.dereference_cache.write().unwrap();
            cache.entry(subject.clone()).or_default();
            for triple in triples {
                cache
                    .entry(triple.subject)
                    .or_default()
                    .entry(triple.predicate)
                    .or_default()
                    .insert(triple.object);
            }

            let mut dereferenced = self.dereferenced_subjects.write().unwrap();
            dereferenced.insert(subject.clone());
            for touched in touched_subjects {
                if touched == *subject {
                    continue;
                }
                let has_real_data = cache
                    .get(&touched)
                    .is_some_and(|preds| preds.keys().any(|p| !STUB_ONLY_PREDICATES.contains(&p.as_str())));
                if has_real_data {
                    dereferenced.insert(touched);
                }
            }
        };

        let OxSubject::NamedNode(iri) = subject else {
            mark_done(Vec::new());
            return Ok(());
        };

        if cancellation::is_cancelled() {
            return Err(OxigraphEndpointError::Cancelled);
        }
        self.enforce_rate_limit().await?;

        let url = Url::parse(iri.as_str())?;
        let body = match http_get_with_retry(&self.dereference_client, &url, &self.rate_limit_ms).await {
            Ok(body) => body,
            Err(OxigraphEndpointError::Cancelled) => return Err(OxigraphEndpointError::Cancelled),
            Err(err) => {
                debug!(subject = %subject, error = %err, "dereference: no usable representation, treating as empty");
                mark_done(Vec::new());
                return Ok(());
            },
        };

        let triples = match parse_turtle_lenient(&body, iri.as_str()) {
            Ok(triples) => triples,
            Err(error) => {
                warn!(subject = %subject, %error, "dereference: response did not parse as Turtle, treating as empty");
                mark_done(Vec::new());
                return Ok(());
            },
        };

        trace!(subject = %subject, triples = triples.len(), "dereference: merging fetched triples into cache");
        mark_done(triples);
        Ok(())
    }

    /// `EndpointStrategy::Dereference` implementation of `outgoing_arcs_from_list`:
    /// dereference `subject` if it isn't already in `dereferenced_subjects` (a
    /// no-op if it is — one fetch is always enough for a subject actually
    /// dereferenced, as opposed to one merely mentioned as a referenced-value
    /// stub — see `dereference_cache`'s doc comment), then answer `preds`
    /// straight from `dereference_cache`.
    fn outgoing_arcs_from_list_dereference(
        &self,
        subject: &OxSubject,
        preds: &[OxNamedNode],
    ) -> Result<(HashMap<OxNamedNode, HashSet<OxTerm>>, Vec<OxNamedNode>)> {
        let already_fetched = self.dereferenced_subjects.read().unwrap().contains(subject);
        if !already_fetched {
            let this = self.clone();
            let subject = subject.clone();
            http::block_on(async move { this.dereference_subject(&subject).await })?;
        }

        let cache = self.dereference_cache.read().unwrap();
        let mut results: HashMap<OxNamedNode, HashSet<OxTerm>> = HashMap::new();
        if let Some(subject_data) = cache.get(subject) {
            for pred in preds {
                if let Some(objects) = subject_data.get(pred) {
                    results.entry(pred.clone()).or_default().extend(objects.iter().cloned());
                }
            }
        }
        // Remainder predicates aren't tracked for the dereference strategy either
        // (mirrors the SPARQL branch): closed-shape validation isn't supported
        // against live/remote backends.
        Ok((results, Vec::new()))
    }

    /// `EndpointStrategy::Dereference` implementation of `incoming_arcs_from_list`:
    /// a best-effort scan over `dereference_cache` for subjects whose fetched
    /// outgoing triples happen to point at `object` via one of `preds`. Necessarily
    /// partial — see `dereference_cache`'s doc comment — but it's the only answer
    /// available without a SPARQL index, and it only gets more complete as
    /// validation dereferences more of the graph.
    fn incoming_arcs_from_list_dereference(
        &self,
        object: &OxTerm,
        preds: &[OxNamedNode],
    ) -> HashMap<OxNamedNode, HashSet<OxSubject>> {
        debug!(
            object = %object,
            "incoming_arcs_from_list: dereference strategy only sees links from entities already \
             dereferenced this session — results may be incomplete"
        );
        let mut results: HashMap<OxNamedNode, HashSet<OxSubject>> = HashMap::new();
        let cache = self.dereference_cache.read().unwrap();
        for (subject, subject_data) in cache.iter() {
            for pred in preds {
                if subject_data.get(pred).is_some_and(|objects| objects.contains(object)) {
                    results.entry(pred.clone()).or_default().insert(subject.clone());
                }
            }
        }
        results
    }

    /// `EndpointStrategy::Dereference` implementation of `triples_matching`:
    /// a linear scan over `dereference_cache`, same partiality caveat as
    /// `incoming_arcs_from_list_dereference`.
    fn triples_matching_dereference(&self) -> Vec<OxTriple> {
        let cache = self.dereference_cache.read().unwrap();
        cache
            .iter()
            .flat_map(|(subject, subject_data)| {
                subject_data.iter().flat_map(move |(pred, objects)| {
                    objects
                        .iter()
                        .map(move |o| OxTriple::new(subject.clone(), pred.clone(), o.clone()))
                })
            })
            .collect()
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
            OxigraphEndpoint::new(&iri_s, &PrefixMap::new())
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
        let solutions =
            make_sparql_query_select_async(&query, &self.client, &self.endpoint_iri, &self.rate_limit_ms).await?;

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
        let solutions =
            make_sparql_query_select_async(&query, &self.client, &self.endpoint_iri, &self.rate_limit_ms).await?;

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
        let solutions =
            make_sparql_query_select_async(&query, &self.client, &self.endpoint_iri, &self.rate_limit_ms).await?;

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

// The synchronous lookups wait for the HTTP requests with `http::block_on`:
// natively on a shared tokio runtime, on wasm by suspending with JSPI.
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
        if self.strategy == EndpointStrategy::Dereference {
            // Best-effort: filter whatever's in `dereference_cache` so far — see
            // `triples_matching_dereference`'s doc comment for the partiality caveat.
            let matched: Vec<OxTriple> = self
                .triples_matching_dereference()
                .into_iter()
                .filter(|t| {
                    subject.value().is_none_or(|v| *v == t.subject)
                        && predicate.value().is_none_or(|v| *v == t.predicate)
                        && object.value().is_none_or(|v| *v == t.object)
                })
                .collect();
            return Ok(matched.into_iter());
        }

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
        let triples: Vec<OxTriple> = solutions
            .into_iter()
            .filter_map(move |solution| {
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
            })
            .collect();

        Ok(triples.into_iter())
    }

    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: &[Self::IRI],
    ) -> Result<(HashMap<Self::IRI, HashSet<Self::Term>>, Vec<Self::IRI>)> {
        if preds.is_empty() {
            return Ok((HashMap::new(), Vec::new()));
        }
        if self.strategy == EndpointStrategy::Dereference {
            return self.outgoing_arcs_from_list_dereference(subject, preds);
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
        //
        // One UNION branch per predicate, each a triple pattern with both the
        // subject and the predicate bound (`BIND` recovers ?p for the caller).
        // A single `{subject} ?p ?o FILTER(?p IN (...))` pattern instead looks
        // like it should be equivalent, but on a live SPARQL endpoint it isn't:
        // with ?p a variable, the engine has to scan every triple with this
        // subject (a full SPO range scan) and evaluate the FILTER per row,
        // whereas a fully-bound (subject, predicate) pattern per UNION branch
        // is a direct index lookup. For a highly-connected node (e.g. a
        // Wikidata statement with many qualifiers) this is the difference
        // between one query per predicate's worth of index seeks and a scan
        // over everything that node has.
        let union_clauses = uncached
            .iter()
            .map(|p| format!("{{ {subject} {p} ?o BIND({p} AS ?p) }}"))
            .collect::<Vec<_>>()
            .join(" UNION ");
        let query = format!("SELECT ?p ?o WHERE {{ {union_clauses} }}");

        trace!(
            subject = %subject,
            cached = preds.len() - uncached.len(),
            fetching = uncached.len(),
            %query,
            "outgoing_arcs_from_list UNION query"
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

    /// Fetches incoming (inverse) arcs for the requested predicates only.
    ///
    /// Overrides the `NeighsRDF` default, which fetches *every* triple pointing at
    /// `object` (`SELECT ?s ?p WHERE { ?s ?p <object> }`, no predicate filter, no
    /// `LIMIT`) and filters client-side. For a heavily-linked node on a live SPARQL
    /// endpoint (e.g. a well-known Wikidata entity), that default can pull in a huge
    /// or effectively unbounded result set even when the shape only constrains a
    /// couple of inverse predicates. Mirrors `outgoing_arcs_from_list`: scope the
    /// query to the requested predicates (one UNION branch per predicate, each
    /// fully bound) and cache results per object so repeated references cost
    /// one SPARQL request.
    fn incoming_arcs_from_list(
        &self,
        object: &Self::Term,
        preds: &[Self::IRI],
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Subject>>> {
        if preds.is_empty() {
            return Ok(HashMap::new());
        }
        if self.strategy == EndpointStrategy::Dereference {
            return Ok(self.incoming_arcs_from_list_dereference(object, preds));
        }

        // --- Cache read pass ---
        let mut results: HashMap<OxNamedNode, HashSet<OxSubject>> = HashMap::new();
        let mut uncached: Vec<&OxNamedNode> = Vec::new();
        {
            let cache = self.incoming_triple_cache.read().unwrap();
            if let Some(object_data) = cache.get(object) {
                for pred in preds {
                    if let Some(subjects) = object_data.get(pred) {
                        results
                            .entry(pred.clone())
                            .or_default()
                            .extend(subjects.iter().cloned());
                    } else {
                        uncached.push(pred);
                    }
                }
            } else {
                uncached.extend(preds.iter());
            }
        }

        if uncached.is_empty() {
            trace!(object = %object, "incoming_arcs_from_list: all {} preds from cache", preds.len());
            return Ok(results);
        }

        // --- SPARQL fetch for uncached predicates ---
        // See `outgoing_arcs_from_list` for why this is a UNION of fully-bound
        // (predicate, object) patterns rather than `?p ?o FILTER(?p IN (...))`.
        let union_clauses = uncached
            .iter()
            .map(|p| format!("{{ ?s {p} {object} BIND({p} AS ?p) }}"))
            .collect::<Vec<_>>()
            .join(" UNION ");
        let query = format!("SELECT ?s ?p WHERE {{ {union_clauses} }}");

        trace!(
            object = %object,
            cached = preds.len() - uncached.len(),
            fetching = uncached.len(),
            %query,
            "incoming_arcs_from_list UNION query"
        );

        let solutions = self.query_select(&query)?;

        // --- Cache write pass ---
        let mut cache = self.incoming_triple_cache.write().unwrap();
        let object_entry = cache.entry(object.clone()).or_default();
        for pred in &uncached {
            object_entry.entry((*pred).clone()).or_default();
        }
        for solution in solutions.into_iter() {
            let Some(p_term) = solution.find_solution("p") else {
                continue;
            };
            let p: OxNamedNode = match p_term.clone().try_into() {
                Ok(n) => n,
                Err(_) => continue,
            };
            let Some(s_term) = solution.find_solution("s") else {
                continue;
            };
            let s: OxSubject = match s_term.clone().try_into() {
                Ok(s) => s,
                Err(_) => continue,
            };
            object_entry.entry(p.clone()).or_default().insert(s.clone());
            results.entry(p).or_default().insert(s);
        }

        Ok(results)
    }
}

// These sync methods bridge to the async implementations with
// `http::block_on`: natively through a shared tokio runtime, on wasm by
// suspending with JSPI.
impl QueryRDF for OxigraphEndpoint {
    /// Executes a SPARQL CONSTRUCT query synchronously.
    ///
    /// This is a blocking wrapper around `query_construct_async`.
    fn query_construct(&self, query: &str, format: &QueryResultFormat) -> Result<String> {
        let (this, query, format) = (self.clone(), query.to_string(), format.clone());
        http::block_on(async move { this.query_construct_async(&query, &format).await })
    }

    /// Executes a SPARQL SELECT query synchronously.
    ///
    /// This is a blocking wrapper around `query_select_async`.
    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>> {
        let (this, query) = (self.clone(), query.to_string());
        http::block_on(async move { this.query_select_async(&query).await })
    }

    /// Executes a SPARQL ASK query synchronously.
    ///
    /// This is a blocking wrapper around `query_ask_async`.
    fn query_ask(&self, query: &str) -> Result<bool> {
        let (this, query) = (self.clone(), query.to_string());
        http::block_on(async move { this.query_ask_async(&query).await })
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

/// `Accept` header of SELECT and ASK queries.
const SPARQL_RESULTS_JSON: &str = "application/sparql-results+json";

/// Predicates Wikibase's RDF exporter attaches to a referenced-but-not-fetched
/// entity's label-only "stub" (its `rdf:type` plus label/description
/// variants — e.g. dereferencing Wikidata's Q80 includes `wd:Q84 a
/// wikibase:Item ; rdfs:label "London"@en ; ...` for every place, person,
/// etc. it references as a value, without Q84's own substantive properties).
/// A subject whose known predicate set is a non-empty subset of exactly
/// these is almost certainly just such a stub, not that entity's real
/// document, and [`OxigraphEndpoint::dereference_subject`] uses that to
/// decide which *other* subjects a fetched document can mark as fully
/// dereferenced (its nested statement/qualifier/reference nodes) versus
/// which still need a request of their own (entities it merely references).
const STUB_ONLY_PREDICATES: &[&str] = &[
    "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
    "http://www.w3.org/2000/01/rdf-schema#label",
    "http://www.w3.org/2004/02/skos/core#prefLabel",
    "http://schema.org/name",
    "http://schema.org/description",
];

/// Parses `body` as Turtle with `base` as the base IRI, skipping any
/// individual triple that fails to parse rather than aborting the whole
/// document. Appropriate for [`EndpointStrategy::Dereference`]: `body` is a
/// page we don't control, and getting the triples that *do* parse is more
/// useful than discarding all of them over one bad one.
fn parse_turtle_lenient(body: &str, base: &str) -> std::result::Result<Vec<OxTriple>, String> {
    let parser = oxttl::TurtleParser::new()
        .lenient()
        .with_base_iri(base)
        .map_err(|e| e.to_string())?;
    Ok(parser
        .for_slice(body.as_bytes())
        .filter_map(|result| result.ok())
        .collect())
}

/// Starting value for `OxigraphEndpoint::rate_limit_ms` — the minimum gap enforced
/// between requests before the server has ever throttled us. Wikidata doesn't publish
/// an exact requests-per-second budget, so this is a conservative guess (< 1 req/s);
/// `http_get_with_retry` raises it adaptively if the server disagrees via a 429.
const INITIAL_RATE_LIMIT_MS: u64 = 1100;

/// Ceiling for the adaptive growth of `OxigraphEndpoint::rate_limit_ms`. Bounds how
/// slow repeated 429s can drive the proactive rate limiter, so a pathological run
/// degrades to "one request every 10 s" rather than effectively hanging.
const MAX_RATE_LIMIT_MS: u64 = 10_000;

/// Returns whether `status` is worth retrying: 429 (Too Many Requests) or one of the
/// transient upstream-gateway errors (502/503/504) that public SPARQL endpoints like
/// Wikidata's — fronted by a load balancer in front of a backend that can be briefly
/// overloaded — routinely return for an otherwise well-formed query.
fn is_retryable_status(status: u16) -> bool {
    matches!(
        status,
        TOO_MANY_REQUESTS | BAD_GATEWAY | SERVICE_UNAVAILABLE | GATEWAY_TIMEOUT
    )
}

const TOO_MANY_REQUESTS: u16 = 429;
const BAD_GATEWAY: u16 = 502;
const SERVICE_UNAVAILABLE: u16 = 503;
const GATEWAY_TIMEOUT: u16 = 504;

/// How much of an error response's body [`OxigraphEndpointError::HTTPStatusError`] keeps.
const MAX_ERROR_BODY_CHARS: usize = 1000;

fn http_status_error(url: &Url, status: u16, body: &str) -> OxigraphEndpointError {
    let body = body.trim();
    let mut body = match body.char_indices().nth(MAX_ERROR_BODY_CHARS) {
        Some((end, _)) => format!("{}…", &body[..end]),
        None => body.to_string(),
    };
    if body.is_empty() {
        body.push_str("(empty body)");
    }
    OxigraphEndpointError::HTTPStatusError {
        url: url.to_string(),
        status,
        body,
    }
}

/// Upper bound honoured on a server-supplied `Retry-After` value. Wikidata's own
/// throttling docs describe waits on the order of seconds to low minutes for an
/// ordinary 429 (a client that keeps ignoring 429s escalates to a 24 h *ban*, but
/// that comes back as 403, not a 429 with an enormous `Retry-After`) — this just
/// guards against a malformed or unusually large header value stalling silently
/// for longer than that. `http::sleep` still lets Ctrl-C cut any wait short (natively).
const MAX_HONOURED_RETRY_AFTER_SECS: u64 = 300;

/// Sends an HTTP GET request (a SPARQL query, or an entity dereference under
/// [`EndpointStrategy::Dereference`]) with `client`, retrying on 429 (Too Many
/// Requests), on transient upstream-gateway errors (502/503/504), and on
/// transport-level failures (connection timeout, reset) such as those from a
/// backend stalling under load.
///
/// Wikidata and other public endpoints enforce rate limits and are occasionally
/// overloaded. When a retryable status is received this function waits for the
/// duration indicated by the `Retry-After` header (falling back to exponential backoff
/// starting at 1 s) and retries up to `MAX_RETRIES` times before propagating the error.
///
/// A `429` specifically also raises `rate_limit_ms` (see its doc comment) so that
/// requests made *after* this one — not just the retry of this one — back off too;
/// per Wikidata's throttling policy, a client that doesn't ease up after a 429 risks
/// escalating to a much longer ban.
async fn http_get_with_retry(
    client: &HttpClient,
    url: &Url,
    rate_limit_ms: &std::sync::atomic::AtomicU64,
) -> Result<String> {
    // The proactive rate limiter in `enforce_rate_limit` already prevents most 429s.
    // These retries handle the rare cases where bursts still slip through, plus
    // one-off 502/503/504 blips (or a stalled connection) from the endpoint's own backend.
    const MAX_RETRIES: u32 = 3;
    debug!(url = %url, "HTTP request");
    for retry in 0..=MAX_RETRIES {
        if cancellation::is_cancelled() {
            return Err(OxigraphEndpointError::Cancelled);
        }
        trace!(url = %url, retry, "HTTP GET attempt");
        let response = match client.get(url).await {
            Ok(response) => response,
            // No HTTP response at all (timed out, connection reset/refused): the
            // request never reached `is_retryable_status` below, so it needs its
            // own retry path with the same exponential backoff.
            Err(SendError::Transient(err)) if retry < MAX_RETRIES => {
                let delay_secs = (1u64 << retry).min(5);
                warn!(url = %url, error = %err, delay_secs, retry, "HTTP request failed: retrying after delay");
                http::sleep(std::time::Duration::from_secs(delay_secs)).await?;
                continue;
            },
            Err(SendError::Transient(err) | SendError::Fatal(err)) => return Err(err),
        };
        let status = response.status;
        trace!(url = %url, status, "HTTP response");

        if is_retryable_status(status) {
            if status == TOO_MANY_REQUESTS {
                // The server just told us to slow down; make every future request
                // through this endpoint wait longer too, not just this retry.
                let current = rate_limit_ms.load(std::sync::atomic::Ordering::Relaxed);
                let raised = (current.saturating_mul(3) / 2).min(MAX_RATE_LIMIT_MS).max(current);
                if raised > current
                    && rate_limit_ms
                        .compare_exchange(
                            current,
                            raised,
                            std::sync::atomic::Ordering::Relaxed,
                            std::sync::atomic::Ordering::Relaxed,
                        )
                        .is_ok()
                {
                    debug!(endpoint = %url, from_ms = current, to_ms = raised, "429 from server: raising proactive rate limit");
                }
            }

            if retry == MAX_RETRIES {
                warn!(url = %url, status, "HTTP request failed: max retries reached, giving up");
                return Err(http_status_error(url, status, &response.body));
            }
            // Honour Retry-After in full when present — Wikidata's throttling policy
            // is explicit that clients should wait out the full duration it names,
            // not a client-side guess, on pain of escalating to a much longer ban.
            // Only the exponential-backoff *fallback* (used when the header is
            // absent, e.g. for 502/503/504 blips that aren't rate-limit responses)
            // is our own guess, so that one stays capped at 5 s.
            let delay_secs = match response
                .retry_after
                .as_deref()
                .and_then(|s| s.trim().parse::<u64>().ok())
            {
                Some(secs) => secs.min(MAX_HONOURED_RETRY_AFTER_SECS),
                None => (1u64 << retry).min(5),
            };
            warn!(url = %url, status, delay_secs, retry, "HTTP request failed: retrying after delay");
            http::sleep(std::time::Duration::from_secs(delay_secs)).await?;
            continue;
        }

        if !(200..300).contains(&status) {
            return Err(http_status_error(url, status, &response.body));
        }
        return Ok(response.body);
    }
    unreachable!()
}

async fn make_sparql_query_select_async(
    query_str: &str,
    client: &HttpClient,
    endpoint_iri: &IriS,
    rate_limit_ms: &std::sync::atomic::AtomicU64,
) -> Result<Vec<OxQuerySolution>> {
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query_str)])?;
    let body = http_get_with_retry(client, &url, rate_limit_ms).await?;
    parse_sparql_json_results(&body)
}

async fn make_sparql_query_construct_async(
    query: &str,
    client: &HttpClient,
    endpoint_iri: &IriS,
    _format: &QueryResultFormat,
    rate_limit_ms: &std::sync::atomic::AtomicU64,
) -> Result<String> {
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;
    http_get_with_retry(client, &url, rate_limit_ms).await
}

async fn make_sparql_query_ask_async(
    query: &str,
    client: &HttpClient,
    endpoint_iri: &IriS,
    rate_limit_ms: &std::sync::atomic::AtomicU64,
) -> Result<bool> {
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;
    let body = http_get_with_retry(client, &url, rate_limit_ms).await?;
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
    let sanitized = sanitize_sparql_json_body(body);

    match json_parser.for_reader(sanitized.as_bytes())? {
        ReaderQueryResultsParserOutput::Boolean(b) => Ok(b),
        _ => Err(OxigraphEndpointError::ParsingBody {
            body: format!("Expected boolean ASK result, got: {}", body),
        }),
    }
}

/// Strips non-standard top-level members from a SPARQL JSON results body.
///
/// The SPARQL 1.1 Query Results JSON Format only defines `head`, `results`
/// and `boolean` at the top level, but some endpoints (e.g. QLever) append
/// extra vendor metadata such as `"meta":{"query-time-ms":...}` after
/// `results`. `sparesults`'s streaming reader is strict and fails with
/// "Unexpected JSON after the end of the bindings array" when it encounters
/// such trailing members, even though the response is otherwise valid JSON.
/// Re-serializing with only the spec-defined keys keeps `sparesults` happy
/// while tolerating endpoint-specific extensions.
///
/// If `body` is not valid JSON, it is returned unchanged so the underlying
/// parser can report the original error.
fn sanitize_sparql_json_body(body: &str) -> std::borrow::Cow<'_, str> {
    let Ok(serde_json::Value::Object(mut map)) = serde_json::from_str::<serde_json::Value>(body) else {
        return std::borrow::Cow::Borrowed(body);
    };

    let known_keys = ["head", "results", "boolean"];
    if map.keys().all(|k| known_keys.contains(&k.as_str())) {
        return std::borrow::Cow::Borrowed(body);
    }

    map.retain(|k, _| known_keys.contains(&k.as_str()));
    std::borrow::Cow::Owned(serde_json::Value::Object(map).to_string())
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
    let sanitized = sanitize_sparql_json_body(body);

    if let ReaderQueryResultsParserOutput::Solutions(solutions) = json_parser.for_reader(sanitized.as_bytes())? {
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
