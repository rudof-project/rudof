//! `SERVICE` in a query against the in-memory graph: it reaches the endpoint it
//! names only with the `http-client` feature, and is refused without it.

use crate::rdf_core::RDFFormat;
use crate::rdf_core::query::QueryRDF;
use crate::rdf_impl::{OxigraphInMemory, ReaderMode};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

const RESULTS: &str = r#"{"head":{"vars":["o"]},"results":{"bindings":[{"o":{"type":"literal","value":"remote"}}]}}"#;

/// A SPARQL endpoint answering every request with `RESULTS`. Returns its IRI
/// and how many requests it has received.
fn serve() -> (String, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let requests = Arc::new(AtomicUsize::new(0));
    {
        let requests = Arc::clone(&requests);
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                requests.fetch_add(1, Ordering::SeqCst);
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut line = String::new();
                while reader.read_line(&mut line).unwrap_or(0) > 0 && line != "\r\n" {
                    line.clear();
                }
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/sparql-results+json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{RESULTS}",
                    RESULTS.len()
                );
                let _ = (&stream).write_all(response.as_bytes());
            }
        });
    }
    (format!("http://{addr}/sparql"), requests)
}

fn query_service(endpoint: &str) -> Result<Vec<String>, String> {
    let mut graph = OxigraphInMemory::from_str(
        "<http://example.org/x> <http://example.org/p> 1 .",
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .unwrap();
    graph.ensure_store().unwrap();
    let solutions = graph
        .query_select(&format!("SELECT ?o WHERE {{ SERVICE <{endpoint}> {{ ?s ?p ?o }} }}"))
        .map_err(|e| e.to_string())?;
    Ok(solutions
        .iter()
        .filter_map(|solution| solution.find_solution("o").map(|o| o.to_string()))
        .collect())
}

#[cfg(feature = "http-client")]
#[test]
fn service_reaches_the_endpoint_with_the_http_client() {
    let (endpoint, requests) = serve();
    let values = query_service(&endpoint).unwrap();
    assert_eq!(values, vec!["\"remote\"".to_string()]);
    assert!(requests.load(Ordering::SeqCst) > 0);
}

#[cfg(not(feature = "http-client"))]
#[test]
fn service_is_refused_without_the_http_client() {
    let (endpoint, requests) = serve();
    assert!(query_service(&endpoint).is_err(), "SERVICE must be refused");
    assert_eq!(requests.load(Ordering::SeqCst), 0, "no request may be made");
}
