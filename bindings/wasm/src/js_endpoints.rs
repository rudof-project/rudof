//! JavaScript API for remote SPARQL endpoints (the `endpoints` feature).
//!
//! The validators and the SPARQL engine are synchronous, while HTTP requests
//! from wasm can only be made with JavaScript's asynchronous `fetch`. The
//! methods here bridge the two with JSPI (WebAssembly JavaScript Promise
//! Integration): each returns a `Promise`, and while it waits for an HTTP
//! response the WebAssembly stack is suspended, so the JavaScript event loop
//! (and the page) keeps running.
//!
//! ```js
//! const rudof = new Rudof();
//! rudof.useEndpoint("https://query.wikidata.org/sparql");
//! rudof.readShex(schema);
//! rudof.readShapemap("wd:Q80@<Human>");
//! const report = await rudof.validateShexAsync();
//! ```
//!
//! The synchronous methods (`validateShex`, `runQuery`, ...) still work with
//! data loaded with `readData`, but throw when they would need to wait for an
//! endpoint. While an asynchronous call is pending, the `Rudof` object is in
//! use: calling another of its methods throws until the `Promise` settles.
//!
//! Requires a runtime with JSPI and exnref (e.g. Chrome 137+, Node.js 25+):
//! a module built with this feature does not load elsewhere. Endpoints must
//! also allow cross-origin requests (CORS) when used from a browser.

// `#[wasm_bindgen(jspi)]` is marked deprecated while JSPI support in
// wasm-bindgen is experimental.
#![allow(deprecated)]

use crate::js::{JsRudof, to_js};
use rudof_rdf::jspi::run_suspendable;
use rudof_rdf::rdf_core::EndpointLimits;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_class = Rudof)]
impl JsRudof {
    /// Uses a remote SPARQL endpoint as the RDF data, replacing the current
    /// data. `endpoint` is the URL of the endpoint (e.g.
    /// `https://query.wikidata.org/sparql`) or the name of one of the known
    /// endpoints (see `listEndpoints`). `strategy` selects how nodes are looked
    /// up: `sparql` (the default) queries the endpoint, `dereference` fetches
    /// each node's IRI as Turtle. `options` overrides the configured request
    /// limits:
    ///
    /// - `minRequestIntervalMs`: minimum milliseconds between two requests
    ///   (default 1100 for remote hosts, 0 for `localhost`). `0` disables the
    ///   proactive rate limiter, though a `429 Too Many Requests` response still
    ///   makes rudof back off.
    /// - `maxConcurrentRequests`: requests in flight at once when several
    ///   independent ones are needed, e.g. to fetch the nodes of a shapemap
    ///   before validating them (default 1).
    /// - `maxNodesPerQuery`: nodes fetched by one query when fetching the nodes
    ///   of a shapemap (default 20; `1` disables batching).
    ///
    /// No request is made here: the data is queried by the asynchronous
    /// methods (`validateShexAsync`, `validateShaclAsync`, `runQueryAsync`,
    /// `nodeInfoAsync`).
    #[wasm_bindgen(js_name = useEndpoint)]
    pub fn use_endpoint(
        &mut self,
        endpoint: &str,
        strategy: Option<String>,
        #[wasm_bindgen(
            unchecked_param_type = "{ minRequestIntervalMs?: number; maxConcurrentRequests?: number; maxNodesPerQuery?: number }"
        )]
        options: JsValue,
    ) -> Result<(), JsValue> {
        let options: EndpointOptions = if options.is_undefined() || options.is_null() {
            EndpointOptions::default()
        } else {
            serde_wasm_bindgen::from_value(options).map_err(|e| {
                JsValue::from(crate::error::Error::new(
                    "DataError",
                    format!("Invalid endpoint options: {e}"),
                ))
            })?
        };
        Ok(self
            .session
            .use_endpoint(endpoint, strategy.as_deref(), options.into())?)
    }

    /// Number of HTTP requests sent so far to the endpoint selected with
    /// `useEndpoint` (retries included; 0 when the data is not an endpoint).
    #[wasm_bindgen(js_name = requestCount)]
    pub fn request_count(&self) -> f64 {
        self.session.endpoint_request_count() as f64
    }

    /// Like `validateShex`, but can validate data from a remote SPARQL
    /// endpoint (see `useEndpoint`).
    #[wasm_bindgen(jspi, js_name = validateShexAsync, unchecked_return_type = "ShExValidationReport")]
    pub fn validate_shex_async(&mut self) -> Result<JsValue, JsValue> {
        run_suspendable(|| to_js(&self.session.validate_shex()?))
    }

    /// Like `validateShacl`, but can validate data from a remote SPARQL
    /// endpoint (see `useEndpoint`). `fetchStrategy` selects how its data is
    /// fetched: `on-demand` (the default) fetches each focus node's data when
    /// the validation needs it; `by-shape` fetches it shape by shape before
    /// validating, many nodes per query, which takes far fewer requests when
    /// there are many focus nodes.
    #[wasm_bindgen(jspi, js_name = validateShaclAsync, unchecked_return_type = "ShaclValidationReport")]
    pub fn validate_shacl_async(
        &mut self,
        mode: Option<String>,
        #[wasm_bindgen(js_name = "fetchStrategy")] fetch_strategy: Option<String>,
    ) -> Result<JsValue, JsValue> {
        run_suspendable(|| {
            to_js(
                &self
                    .session
                    .validate_shacl(mode.as_deref(), fetch_strategy.as_deref())?,
            )
        })
    }

    /// Like `runQuery`, but can query a remote SPARQL endpoint (see
    /// `useEndpoint`).
    #[wasm_bindgen(jspi, js_name = runQueryAsync, unchecked_return_type = "QueryResults")]
    pub fn run_query_async(&mut self) -> Result<JsValue, JsValue> {
        run_suspendable(|| to_js(&self.session.run_query()?))
    }

    /// Like `nodeInfo`, but can describe nodes of a remote SPARQL endpoint
    /// (see `useEndpoint`).
    #[wasm_bindgen(jspi, js_name = nodeInfoAsync)]
    pub fn node_info_async(
        &mut self,
        #[wasm_bindgen(js_name = "nodeSelector")] node_selector: &str,
        predicates: Option<Vec<String>>,
        mode: Option<String>,
        #[wasm_bindgen(js_name = "showColors")] show_colors: Option<bool>,
        depth: Option<usize>,
    ) -> Result<String, JsValue> {
        run_suspendable(|| {
            Ok(self.session.node_info(
                node_selector,
                predicates.as_deref(),
                mode.as_deref(),
                show_colors,
                depth,
            )?)
        })
    }
}

/// Sets the function used for the HTTP requests to SPARQL endpoints, with
/// the signature of `fetch`, or goes back to the global `fetch` with `null`.
///
/// It is called as `fetch(url, { headers, signal })` and only the `status`,
/// `headers.get()` and `text()` of the response it resolves to are used. It
/// can add headers (e.g. the `Api-User-Agent` that Wikimedia asks browser
/// clients to send), route requests through a proxy (e.g. for endpoints that
/// don't allow cross-origin requests), cache, or mock responses in tests.
#[wasm_bindgen(js_name = setFetch)]
pub fn set_fetch(
    #[wasm_bindgen(
        unchecked_param_type = "((url: string, init: { headers: Record<string, string>; signal?: AbortSignal }) => Promise<Response>) | null | undefined"
    )]
    fetch: Option<js_sys::Function>,
) {
    rudof_rdf::jspi::set_fetch(fetch);
}

/// The `options` of `useEndpoint`.
#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct EndpointOptions {
    min_request_interval_ms: Option<u64>,
    max_concurrent_requests: Option<usize>,
    max_nodes_per_query: Option<usize>,
}

impl From<EndpointOptions> for EndpointLimits {
    fn from(options: EndpointOptions) -> Self {
        EndpointLimits::default()
            .with_min_request_interval_ms(options.min_request_interval_ms)
            .with_max_concurrent_requests(options.max_concurrent_requests)
            .with_max_nodes_per_query(options.max_nodes_per_query)
    }
}
