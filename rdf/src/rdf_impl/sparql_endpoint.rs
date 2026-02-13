use crate::{
    rdf_core::{
        Any, AsyncRDF, Matcher, NeighsRDF, Rdf,
        query::{QueryRDF, QueryResultFormat, QuerySolution, QuerySolutions, VarName},
    },
    rdf_impl::SparqlEndpointError,
};
use async_trait::async_trait;
use colored::*;
use iri_s::IriS;
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode,
    NamedOrBlankNode as OxSubject, Term as OxTerm, Triple as OxTriple,
};
use prefixmap::PrefixMap;
use regex::Regex;
use serde::{Serialize, ser::SerializeStruct};
use sparesults::{
    QueryResultsFormat, QueryResultsParser, QuerySolution as OxQuerySolution,
    ReaderQueryResultsParserOutput,
};
use std::{collections::HashSet, fmt::Display, hash::Hash, str::FromStr, sync::Arc};
use url::Url;

/// Type alias for Result with SparqlEndpointError.
type Result<A> = std::result::Result<A, SparqlEndpointError>;

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
pub struct SparqlEndpoint {
    /// The IRI of the SPARQL endpoint.
    endpoint_iri: IriS,

    /// Prefix map for qualifying IRIs.
    prefixmap: Arc<PrefixMap>,

    /// HTTP client configured for SELECT queries (expects JSON results).
    client: Arc<reqwest::Client>,

    /// HTTP client configured for CONSTRUCT queries with Turtle format.
    client_construct_turtle: Arc<reqwest::Client>,

    /// HTTP client configured for CONSTRUCT queries with RDF/XML format.
    client_construct_rdfxml: Arc<reqwest::Client>,

    /// HTTP client configured for CONSTRUCT queries with JSON-LD format.
    client_construct_jsonld: Arc<reqwest::Client>,
}

impl PartialEq for SparqlEndpoint {
    /// Two endpoints are equal if they have the same IRI.
    ///
    /// Note: This compares only the endpoint IRI, not the prefix maps or clients.
    fn eq(&self, other: &Self) -> bool {
        self.endpoint_iri == other.endpoint_iri
    }
}

impl Hash for SparqlEndpoint {
    /// Hash based on the endpoint IRI.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.endpoint_iri.hash(state);
    }
}

impl Eq for SparqlEndpoint {}

impl Serialize for SparqlEndpoint {
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

impl SparqlEndpoint {
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
    /// use rdf::rdf_impl::SparqlEndpoint;
    /// use iri_s::IriS;
    /// use prefixmap::PrefixMap;
    ///
    /// let iri = IriS::new_unchecked("https://dbpedia.org/sparql");
    /// let prefixmap = PrefixMap::new();
    /// let endpoint = SparqlEndpoint::new(&iri, &prefixmap);
    /// ```
    pub fn new(iri: &IriS, prefixmap: &PrefixMap) -> Result<SparqlEndpoint> {
        let client = Arc::new(sparql_client()?);
        let client_construct_turtle = Arc::new(sparql_client_construct_turtle()?);
        let client_construct_jsonld = Arc::new(sparql_client_construct_jsonld()?);
        let client_construct_rdfxml = Arc::new(sparql_client_construct_rdfxml()?);

        Ok(SparqlEndpoint {
            endpoint_iri: iri.clone(),
            prefixmap: Arc::new(prefixmap.clone()),
            client,
            client_construct_turtle,
            client_construct_rdfxml,
            client_construct_jsonld,
        })
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
    /// use rdf::rdf_impl::SparqlEndpoint;
    ///
    /// let wikidata = SparqlEndpoint::wikidata();
    /// ```
    pub fn wikidata() -> Result<SparqlEndpoint> {
        SparqlEndpoint::new(
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
    /// ```
    /// use rdf::rdf_impl::SparqlEndpoint;
    /// use prefixmap::PrefixMap;
    ///
    /// let endpoint = SparqlEndpoint::wikidata();
    /// let custom_prefixmap = PrefixMap::new();
    /// let endpoint = endpoint.unwrap().with_prefixmap(custom_prefixmap);
    /// ```
    pub fn with_prefixmap(mut self, pm: PrefixMap) -> SparqlEndpoint {
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
    /// ```
    /// use rdf::rdf_impl::SparqlEndpoint;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let endpoint = SparqlEndpoint::wikidata()?;
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
        tracing::trace!("srdf_sparql: SPARQL SELECT query: {}", query);
        let solutions =
            make_sparql_query_select_async(query, &self.client, &self.endpoint_iri).await?;

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
    /// ```
    /// use rdf::rdf_impl::SparqlEndpoint;
    /// use rdf::rdf_core::query::QueryResultFormat;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let endpoint = SparqlEndpoint::wikidata()?;
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
    pub async fn query_construct_async(
        &self,
        query: &str,
        format: &QueryResultFormat,
    ) -> Result<String> {
        // Select the appropriate client based on the requested format
        let client = match format {
            QueryResultFormat::Turtle => &self.client_construct_turtle,
            QueryResultFormat::RdfXml => &self.client_construct_rdfxml,
            QueryResultFormat::JsonLd => &self.client_construct_jsonld,
            _ => {
                return Err(SparqlEndpointError::UnsupportedConstructFormat {
                    format: format.to_string(),
                });
            }
        };

        make_sparql_query_construct_async(query, client, &self.endpoint_iri, format).await
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
    /// ```
    /// use rdf::rdf_impl::SparqlEndpoint;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let endpoint = SparqlEndpoint::wikidata()?;
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
        make_sparql_query_ask_async(query, &self.client, &self.endpoint_iri).await
    }
}

impl FromStr for SparqlEndpoint {
    type Err = SparqlEndpointError;

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
            let client_construct_turtle = Arc::new(sparql_client_construct_turtle()?);
            let client_construct_rdfxml = Arc::new(sparql_client_construct_rdfxml()?);
            let client_construct_jsonld = Arc::new(sparql_client_construct_jsonld()?);

            Ok(SparqlEndpoint {
                endpoint_iri: iri_s,
                prefixmap: Arc::new(PrefixMap::new()),
                client,
                client_construct_turtle,
                client_construct_rdfxml,
                client_construct_jsonld,
            })
        } else {
            // Try to match predefined endpoint names
            match s.to_lowercase().as_str() {
                "wikidata" => SparqlEndpoint::wikidata(),
                name => Err(SparqlEndpointError::UnknownEndpointName {
                    name: name.to_string(),
                }),
            }
        }
    }
}

impl Rdf for SparqlEndpoint {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Triple = OxTriple;
    type Err = SparqlEndpointError;

    /// Resolves a prefix and local name to a full IRI.
    fn resolve_prefix_local(
        &self,
        prefix: &str,
        local: &str,
    ) -> std::result::Result<IriS, prefixmap::PrefixMapError> {
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

#[async_trait]
impl AsyncRDF for SparqlEndpoint {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = SparqlEndpointError;

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
            make_sparql_query_select_async(&query, &self.client, &self.endpoint_iri).await?;

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
            make_sparql_query_select_async(&query, &self.client, &self.endpoint_iri).await?;

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
            make_sparql_query_select_async(&query, &self.client, &self.endpoint_iri).await?;

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
impl NeighsRDF for SparqlEndpoint {
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
    /// ```
    /// use rdf::rdf_impl::SparqlEndpoint;
    /// use rdf::rdf_core::{Any, NeighsRDF};
    /// use oxrdf::NamedNode;
    ///
    // /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ///     let endpoint = SparqlEndpoint::wikidata()?;
    // ///
    // ///     let predicate = NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?;
    // ///     let mut triples = endpoint.triples_matching(&Any, &predicate, &Any)?;
    // ///
    // ///     // Take the first triple, if any
    // ///     let first = triples.next();
    // ///
    // ///     assert!(first.is_some(), "Expected at least one triple from the endpoint");
    // ///
    // ///     Ok(())
    // /// }
    /// ```
    fn triples_matching<S, P, O>(
        &self,
        subject: &S,
        predicate: &P,
        object: &O,
    ) -> Result<impl Iterator<Item = Self::Triple>>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        // Build SPARQL query from matchers
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

        let query = format!("SELECT ?s ?p ?o WHERE {{ {} {} {} }}", s_str, p_str, o_str);

        let solutions = self.query_select(&query)?;

        // Clone matcher values for use in the closure
        let subject_val = subject.value().cloned();
        let predicate_val = predicate.value().cloned();
        let object_val = object.value().cloned();

        // Build iterator that converts query solutions to triples
        let triples = solutions.into_iter().filter_map(move |solution| {
            // Use matched value if available, otherwise extract from solution
            let subject_res: Self::Subject = match &subject_val {
                Some(s) => s.clone(),
                None => solution
                    .find_solution(0)
                    .and_then(|s| s.clone().try_into().ok())?,
            };

            let predicate_res: Self::IRI = match &predicate_val {
                Some(p) => p.clone(),
                None => solution
                    .find_solution(1)
                    .and_then(|pred| pred.clone().try_into().ok())?,
            };

            let object_res = match &object_val {
                Some(o) => o.clone(),
                None => solution.find_solution(2)?.clone(),
            };

            Some(OxTriple::new(subject_res, predicate_res, object_res))
        });

        Ok(triples)
    }
}

// QueryRDF is only available on non-WASM platforms.
// On native platforms, these sync methods use tokio::runtime to run
// the async implementations.
#[cfg(not(target_family = "wasm"))]
impl QueryRDF for SparqlEndpoint {
    /// Executes a SPARQL CONSTRUCT query synchronously.
    ///
    /// This is a blocking wrapper around `query_construct_async`.
    /// It creates a tokio runtime to run the async method.
    ///
    /// # Performance Note
    ///
    /// Creates a new runtime for each call. For better performance in
    /// async contexts, use `query_construct_async` directly.
    fn query_construct(&self, query: &str, format: &QueryResultFormat) -> Result<String> {
        let runtime =
            tokio::runtime::Runtime::new().map_err(|e| SparqlEndpointError::ParsingBody {
                body: format!("Failed to create runtime: {}", e),
            })?;

        runtime.block_on(self.query_construct_async(query, format))
    }

    /// Executes a SPARQL SELECT query synchronously.
    ///
    /// This is a blocking wrapper around `query_select_async`.
    /// It creates a tokio runtime to run the async method.
    ///
    /// # Performance Note
    ///
    /// Creates a new runtime for each call. For better performance in
    /// async contexts, use `query_select_async` directly.
    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>> {
        let runtime =
            tokio::runtime::Runtime::new().map_err(|e| SparqlEndpointError::ParsingBody {
                body: format!("Failed to create runtime: {}", e),
            })?;

        runtime.block_on(self.query_select_async(query))
    }

    /// Executes a SPARQL ASK query synchronously.
    ///
    /// This is a blocking wrapper around `query_ask_async`.
    /// It creates a tokio runtime to run the async method.
    ///
    /// # Performance Note
    ///
    /// Creates a new runtime for each call. For better performance in
    /// async contexts, use `query_ask_async` directly.
    fn query_ask(&self, query: &str) -> Result<bool> {
        let runtime =
            tokio::runtime::Runtime::new().map_err(|e| SparqlEndpointError::ParsingBody {
                body: format!("Failed to create runtime: {}", e),
            })?;

        runtime.block_on(self.query_ask_async(query))
    }
}

/// Converts an oxrdf QuerySolution to our QuerySolution type.
///
/// # Performance
///
/// Uses iterators with `collect()` for efficient conversion.
#[inline]
fn cnv_query_solution(qs: &OxQuerySolution) -> QuerySolution<SparqlEndpoint> {
    let vars: Vec<_> = qs
        .variables()
        .iter()
        .map(|v| VarName::new(v.as_str()))
        .collect();
    let vals: Vec<_> = qs.values().iter().cloned().collect();
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

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

/// Macro to generate HTTP client creation functions for different CONSTRUCT formats.
///
/// This reduces code duplication by generating similar functions with different
/// Accept headers.
///
/// # Parameters
///
/// * `$accept_header` - The MIME type to set in the Accept header
/// * `$func_name` - The name of the generated function
macro_rules! create_construct_client {
    ($accept_header:expr, $func_name:ident) => {
        /// Creates an HTTP client configured for SPARQL CONSTRUCT queries.
        ///
        /// Sets the Accept header to the specified format and includes a custom User-Agent.
        fn $func_name() -> Result<reqwest::Client> {
            use reqwest::header::{self, ACCEPT, USER_AGENT};

            let mut headers = header::HeaderMap::new();
            headers.insert(ACCEPT, header::HeaderValue::from_static($accept_header));
            headers.insert(USER_AGENT, header::HeaderValue::from_static("rudof"));

            let client = reqwest::Client::builder()
                .default_headers(headers)
                .build()?;
            Ok(client)
        }
    };
}

// Generate client creation functions for each CONSTRUCT format
create_construct_client!("text/turtle", sparql_client_construct_turtle);
create_construct_client!("application/ld+json", sparql_client_construct_jsonld);
create_construct_client!("application/rdf+xml", sparql_client_construct_rdfxml);

/// Executes a SPARQL SELECT query asynchronously.
///
/// This is the core async implementation used by both WASM and native platforms.
///
/// # Arguments
///
/// * `query_str` - The SPARQL query string
/// * `client` - The HTTP client to use
/// * `endpoint_iri` - The endpoint IRI
///
/// # Returns
///
/// A vector of query solutions.
///
/// # Errors
///
/// Returns an error if:
/// - URL construction fails
/// - HTTP request fails
/// - Response cannot be parsed as JSON
/// - JSON cannot be parsed as SPARQL results
async fn make_sparql_query_select_async(
    query_str: &str,
    client: &reqwest::Client,
    endpoint_iri: &IriS,
) -> Result<Vec<OxQuerySolution>> {
    // Build URL with query parameter
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query_str)])?;
    tracing::debug!("Making SPARQL query: {}", url);

    // Execute request and get response body
    let body = client.get(url).send().await?.text().await?;
    parse_sparql_json_results(&body)
}

/// Executes a SPARQL CONSTRUCT query asynchronously.
///
/// # Arguments
///
/// * `query` - The SPARQL CONSTRUCT query string
/// * `client` - The HTTP client configured for the desired format
/// * `endpoint_iri` - The endpoint IRI
/// * `_format` - The query result format (currently unused, determined by client headers)
///
/// # Returns
///
/// The query results as a string in the format determined by the client's Accept header.
async fn make_sparql_query_construct_async(
    query: &str,
    client: &reqwest::Client,
    endpoint_iri: &IriS,
    _format: &QueryResultFormat,
) -> Result<String> {
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;
    tracing::debug!("Making SPARQL CONSTRUCT query: {}", url);

    let response = client.get(url).send().await?;
    tracing::debug!("status: {}", response.status());

    // Log the Content-Type for debugging
    if let Some(ct) = response.headers().get("Content-Type") {
        match ct.to_str() {
            Ok(s) => tracing::debug!("Content-Type: {}", s),
            Err(e) => tracing::debug!("Content-Type: <invalid>: {}", e),
        }
    }

    let body = response.text().await?;
    tracing::debug!("body length: {}", body.len());
    Ok(body)
}

/// Executes a SPARQL ASK query asynchronously.
///
/// ASK queries return a different JSON format than SELECT queries:
/// `{"head": {}, "boolean": true/false}`
///
/// # Arguments
///
/// * `query` - The SPARQL ASK query string
/// * `client` - The HTTP client to use
/// * `endpoint_iri` - The endpoint IRI
///
/// # Returns
///
/// A boolean indicating whether the query pattern matched.
///
/// # Errors
///
/// Returns an error if:
/// - URL construction fails
/// - HTTP request fails
/// - Response cannot be parsed as JSON
/// - JSON does not contain a boolean field
async fn make_sparql_query_ask_async(
    query: &str,
    client: &reqwest::Client,
    endpoint_iri: &IriS,
) -> Result<bool> {
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;

    let body = client.get(url).send().await?.text().await?;
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
        _ => Err(SparqlEndpointError::ParsingBody {
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

    if let ReaderQueryResultsParserOutput::Solutions(solutions) =
        json_parser.for_reader(body.as_bytes())?
    {
        // Collect all solutions, propagating any parsing errors
        solutions
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| SparqlEndpointError::ParsingBody {
                body: format!("Error parsing solution: {}", e),
            })
    } else {
        Err(SparqlEndpointError::ParsingBody {
            body: body.to_string(),
        })
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
        .ok_or_else(|| SparqlEndpointError::NotFoundInSolution {
            value: name.to_string(),
            solution: format!("{solution:?}"),
        })
        .and_then(|v| match v {
            OxTerm::NamedNode(n) => Ok(n.clone()),
            _ => Err(SparqlEndpointError::SPARQLSolutionErrorNoIRI { value: v.clone() }),
        })
}
