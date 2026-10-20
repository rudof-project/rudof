use serde::{Deserialize, Serialize};

/// How hard rudof may use a SPARQL endpoint: the spacing between requests,
/// how many may be in flight at once, and how many nodes a single query may
/// ask about. Every field is optional: an unset one falls back to the next
/// level (see [`EndpointLimits::or`]) and, at the end, to the endpoint
/// client's own default.
///
/// It appears flattened in `[rdf]` (defaults for every endpoint) and in each
/// `[rdf.endpoints.<name>]` table / endpoint TOML file.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointLimits {
    /// Minimum interval, in milliseconds, between two requests. Default: 1100
    /// for a remote host, 0 for a loopback one. A `429 Too Many Requests`
    /// response still raises it adaptively, whatever its configured value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_request_interval_ms: Option<u64>,

    /// Maximum number of requests in flight at once, when rudof has several
    /// independent ones to make (prefetching the nodes of a shapemap).
    /// Requests stay spaced by `min_request_interval_ms`. Default: 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_requests: Option<usize>,

    /// Maximum number of nodes one SPARQL query fetches the arcs of, when
    /// prefetching the nodes of a shapemap. `1` disables batching. Default: 20.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_nodes_per_query: Option<usize>,
}

impl EndpointLimits {
    pub fn with_min_request_interval_ms(mut self, ms: Option<u64>) -> Self {
        self.min_request_interval_ms = ms;
        self
    }

    pub fn with_max_concurrent_requests(mut self, n: Option<usize>) -> Self {
        self.max_concurrent_requests = n;
        self
    }

    pub fn with_max_nodes_per_query(mut self, n: Option<usize>) -> Self {
        self.max_nodes_per_query = n;
        self
    }

    /// Each field of `self`, or of `fallback` where `self` leaves it unset.
    pub fn or(self, fallback: EndpointLimits) -> EndpointLimits {
        EndpointLimits {
            min_request_interval_ms: self.min_request_interval_ms.or(fallback.min_request_interval_ms),
            max_concurrent_requests: self.max_concurrent_requests.or(fallback.max_concurrent_requests),
            max_nodes_per_query: self.max_nodes_per_query.or(fallback.max_nodes_per_query),
        }
    }

    /// Whether no field is set.
    pub fn is_empty(&self) -> bool {
        *self == EndpointLimits::default()
    }
}
