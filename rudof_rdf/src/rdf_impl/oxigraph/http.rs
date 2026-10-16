//! Platform layer of [`OxigraphEndpoint`](super::OxigraphEndpoint): HTTP GET
//! requests, timers, and the bridge from the synchronous RDF traits
//! (`NeighsRDF`, `QueryRDF`) to the asynchronous requests.
//!
//! The protocol logic (rate limiting, retries, parsing) lives in `endpoint.rs`
//! and is shared by every platform. Only what is below differs:
//!
//! * Natively, requests go through `reqwest`, timers through `tokio`, and
//!   [`block_on`] runs the request on a shared `tokio` runtime.
//! * On wasm (with the `jspi` feature), requests go through JavaScript's
//!   `fetch`, timers through `setTimeout`, and [`block_on`] suspends the
//!   WebAssembly stack with JSPI until the request settles, without blocking
//!   the JavaScript event loop. See [`crate::jspi`].

use super::OxigraphEndpointError;
use std::time::Duration;

/// Per-request timeout applied to every HTTP request. Without this, a stalled
/// connection or a slow response from a remote endpoint (e.g. a heavily-linked node
/// pulling in a huge result set) blocks the caller indefinitely, since neither the
/// HTTP clients nor the [`block_on`] bridge impose a deadline on their own.
pub(crate) const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// User-Agent sent with every HTTP request the native endpoint client makes
/// (SPARQL or dereferencing), e.g. `rudof/0.3.11 (https://github.com/rudof-project/rudof)`.
///
/// Public endpoints like Wikidata's bucket clients by (User-Agent, IP) for
/// throttling purposes, and per Wikimedia's User-Agent policy
/// (<https://foundation.wikimedia.org/wiki/Policy:Wikimedia_Foundation_User-Agent_Policy>)
/// a generic or missing User-Agent gets routed into a more restrictive tier
/// meant to deter anonymous scraping — a plain `"rudof"` (the previous
/// value here) qualifies as generic. Identifying the tool and a way to
/// reach its maintainers, as this does, is what the policy asks for and
/// avoids that penalty.
#[cfg(not(target_family = "wasm"))]
const RUDOF_USER_AGENT: &str = concat!(
    "rudof/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/rudof-project/rudof)"
);

/// An HTTP response, with its body already read.
#[derive(Debug)]
pub(crate) struct HttpResponse {
    pub(crate) status: u16,
    /// The value of the `Retry-After` header, if any.
    pub(crate) retry_after: Option<String>,
    pub(crate) body: String,
}

/// Why an HTTP request got no response.
#[derive(Debug)]
pub(crate) enum SendError {
    /// Worth retrying: a timeout, or a connection that could not be made or
    /// was reset (or, on wasm, any rejected `fetch`, which does not tell
    /// these apart from CORS rejections).
    Transient(OxigraphEndpointError),
    /// Not worth retrying, e.g. cancellation.
    Fatal(OxigraphEndpointError),
}

#[cfg(not(target_family = "wasm"))]
pub(crate) use native::{HttpClient, Instant, block_on, sleep};
#[cfg(target_family = "wasm")]
pub(crate) use wasm::{HttpClient, Instant, block_on, sleep};

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::{HttpResponse, REQUEST_TIMEOUT, SendError};
    use crate::{cancellation, rdf_impl::OxigraphEndpointError};
    use reqwest::header::{ACCEPT, HeaderMap, HeaderValue, USER_AGENT};
    use std::{future::Future, sync::Arc, time::Duration};
    use url::Url;

    pub(crate) use std::time::Instant;

    /// Shared tokio runtime used by the blocking SPARQL methods.
    static SPARQL_RUNTIME: once_cell::sync::Lazy<tokio::runtime::Runtime> = once_cell::sync::Lazy::new(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("rudof-sparql")
            .build()
            .expect("failed to build shared tokio runtime for SPARQL queries")
    });

    /// Runs `future` to completion on the shared runtime, blocking the
    /// current thread.
    pub(crate) fn block_on<T>(
        future: impl Future<Output = Result<T, OxigraphEndpointError>> + 'static,
    ) -> Result<T, OxigraphEndpointError> {
        SPARQL_RUNTIME.block_on(future)
    }

    /// Sleeps for `duration`, unless cancellation is requested first (e.g.
    /// Ctrl-C in the shell), in which case this returns early with
    /// [`OxigraphEndpointError::Cancelled`] instead of waiting out the delay.
    pub(crate) async fn sleep(duration: Duration) -> Result<(), OxigraphEndpointError> {
        tokio::select! {
            biased;
            _ = cancellation::cancelled() => Err(OxigraphEndpointError::Cancelled),
            _ = tokio::time::sleep(duration) => Ok(()),
        }
    }

    /// An HTTP client that sends `Accept: <accept>` and rudof's User-Agent.
    #[derive(Debug, Clone)]
    pub(crate) struct HttpClient {
        client: Arc<reqwest::Client>,
    }

    impl HttpClient {
        pub(crate) fn new(accept: &'static str) -> Result<HttpClient, OxigraphEndpointError> {
            let mut headers = HeaderMap::new();
            headers.insert(ACCEPT, HeaderValue::from_static(accept));
            headers.insert(USER_AGENT, HeaderValue::from_static(super::RUDOF_USER_AGENT));
            let client = reqwest::Client::builder().default_headers(headers).build()?;
            Ok(HttpClient {
                client: Arc::new(client),
            })
        }

        pub(crate) async fn get(&self, url: &Url) -> Result<HttpResponse, SendError> {
            // Raced against `cancellation::cancelled()` so Ctrl-C in the shell (see
            // `rudof_cli`'s signal handler) aborts an in-flight request instead of
            // waiting out the full `REQUEST_TIMEOUT`.
            let sent = tokio::select! {
                biased;
                _ = cancellation::cancelled() => return Err(SendError::Fatal(OxigraphEndpointError::Cancelled)),
                sent = self.client.get(url.as_str()).timeout(REQUEST_TIMEOUT).send() => sent,
            };
            let response = match sent {
                Ok(response) => response,
                Err(err) if err.is_timeout() || err.is_connect() => return Err(SendError::Transient(err.into())),
                Err(err) => return Err(SendError::Fatal(err.into())),
            };
            let status = response.status().as_u16();
            let retry_after = response
                .headers()
                .get("Retry-After")
                .and_then(|v| v.to_str().ok())
                .map(str::to_string);
            let body = response.text().await.map_err(|err| SendError::Fatal(err.into()))?;
            Ok(HttpResponse {
                status,
                retry_after,
                body,
            })
        }
    }
}

#[cfg(target_family = "wasm")]
mod wasm {
    use super::{HttpResponse, SendError};
    use crate::{cancellation, jspi, rdf_impl::OxigraphEndpointError};
    use std::time::Duration;
    use url::Url;

    pub(crate) use crate::jspi::{Instant, block_on};

    /// Sleeps for `duration`, failing with [`OxigraphEndpointError::Cancelled`]
    /// if cancellation was requested.
    pub(crate) async fn sleep(duration: Duration) -> Result<(), OxigraphEndpointError> {
        jspi::sleep(duration).await;
        if cancellation::is_cancelled() {
            return Err(OxigraphEndpointError::Cancelled);
        }
        Ok(())
    }

    /// An HTTP client that sends `Accept: <accept>` with `fetch`. Browsers
    /// don't let pages set the User-Agent; a custom `fetch` (see
    /// [`jspi::set_fetch`]) can add other headers, such as the
    /// `Api-User-Agent` Wikimedia asks browser clients to send.
    #[derive(Debug, Clone)]
    pub(crate) struct HttpClient {
        accept: &'static str,
    }

    impl HttpClient {
        pub(crate) fn new(accept: &'static str) -> Result<HttpClient, OxigraphEndpointError> {
            Ok(HttpClient { accept })
        }

        pub(crate) async fn get(&self, url: &Url) -> Result<HttpResponse, SendError> {
            jspi::fetch(url.as_str(), self.accept).await
        }
    }
}
