use std::fmt::Display;

/// How an `OxigraphEndpoint` answers `NeighsRDF` lookups (outgoing/incoming
/// arcs, `triples_matching`) and `QueryRDF` requests.
///
/// Wikibase instances (Wikidata, MaRDI, ...) publish every entity as Linked
/// Data: `http://www.wikidata.org/entity/Q80` is itself dereferenceable —
/// `GET` it with `Accept: text/turtle` (following redirects) and back comes
/// that entity's full RDF description. [`EndpointStrategy::Dereference`]
/// exploits that as an alternative to SPARQL: one HTTP request per entity,
/// served by the wiki's own (typically CDN-cached) web frontend rather than
/// the separate SPARQL query service — which sidesteps that service's
/// throttling and reliability characteristics entirely, at the cost of only
/// ever seeing *outgoing* arcs (see `OxigraphEndpoint::dereference_cache`).
///
/// Defined on every target, although SPARQL endpoints are not available on
/// `wasm`, so that APIs taking a strategy compile there too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EndpointStrategy {
    /// Answer lookups with SPARQL queries against `endpoint_iri`. Default.
    #[default]
    Sparql,
    /// Answer lookups by dereferencing entity IRIs directly over HTTP.
    Dereference,
}

impl Display for EndpointStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndpointStrategy::Sparql => write!(f, "sparql"),
            EndpointStrategy::Dereference => write!(f, "dereference"),
        }
    }
}
