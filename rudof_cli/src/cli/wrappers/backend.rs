use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

use clap::ValueEnum;
use rudof_lib::formats::BackendSpec;
use rudof_rdf::rdf_impl::EndpointStrategy;

use crate::cli::parser::CommonArgsAll;
use crate::cli_wrapper;

/// Choice of backend.
///
/// Parsed from the `--backend` CLI flag. Accepted forms:
/// - `memory` — in-process `OxigraphInMemory` (default).
/// - `qlever` — local QLever Docker container. Requires the `qlever` feature.
/// - `lbug` — the connected LadybugDB property graph database (see `rudof
///   connect`). Valid on RDF-loading commands too, but not yet functional
///   there: rudof cannot yet read RDF back out of a property graph.
/// - `endpoint=<URL_OR_NAME>` — remote SPARQL endpoint. The value is either a
///   full URL or the name of an endpoint registered in the rudof TOML config.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BackendKindCli {
    #[default]
    Memory,
    Qlever,
    Lbug,
    Endpoint(String),
}

/// Helpers used by every CLI subcommand that loads RDF data, so the
/// `--backend memory|qlever|lbug|endpoint=…` flag means the same thing everywhere.
impl BackendKindCli {
    /// `true` when the user picked the QLever backend.
    pub fn is_qlever(&self) -> bool {
        matches!(self, BackendKindCli::Qlever)
    }

    /// Returns `Some(url_or_name)` when the user picked `endpoint=…`, else `None`.
    pub fn endpoint(&self) -> Option<&str> {
        match self {
            BackendKindCli::Endpoint(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

/// Resolve the effective [`BackendSpec`] for a subcommand.
///
/// The single point of dispatch every subcommand goes through; downstream
/// commands hand the result to `LoadDataBuilder::with_backend`. The
/// `--endpoint` shortcut takes precedence over `--backend`, but clap's
/// `conflicts_with` prevents both from being set at the same time.
pub fn resolve_backend(args: &CommonArgsAll) -> BackendSpec {
    if let Some(url) = args.endpoint.as_deref() {
        return BackendSpec::Endpoint(url.to_string());
    }
    match args.backend.as_ref() {
        Some(b) => b.clone().into(),
        None => BackendSpec::default(),
    }
}

impl From<BackendKindCli> for BackendSpec {
    fn from(b: BackendKindCli) -> Self {
        match b {
            BackendKindCli::Memory => BackendSpec::Memory,
            BackendKindCli::Qlever => BackendSpec::Qlever,
            BackendKindCli::Lbug => BackendSpec::Lbug,
            BackendKindCli::Endpoint(s) => BackendSpec::Endpoint(s),
        }
    }
}

impl Display for BackendKindCli {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BackendKindCli::Memory => write!(f, "memory"),
            BackendKindCli::Qlever => write!(f, "qlever"),
            BackendKindCli::Lbug => write!(f, "lbug"),
            BackendKindCli::Endpoint(url) => write!(f, "endpoint={url}"),
        }
    }
}

/// Parse error for `BackendKindCli` (so it satisfies `std::error::Error` for
/// clap's `value_parser!`).
#[derive(Debug)]
pub struct ParseBackendError(String);

impl Display for ParseBackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ParseBackendError {}

impl FromStr for BackendKindCli {
    type Err = ParseBackendError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let trimmed = s.trim();
        let lowered = trimmed.to_ascii_lowercase();
        match lowered.as_str() {
            "memory" => Ok(BackendKindCli::Memory),
            "qlever" => Ok(BackendKindCli::Qlever),
            "lbug" => Ok(BackendKindCli::Lbug),
            "endpoint" => Err(ParseBackendError(
                "missing endpoint URL or name; use --backend endpoint=<URL_OR_NAME>".to_string(),
            )),
            _ => {
                // Preserve the original (non-lowercased) value for endpoint URLs.
                if let Some(rest) = trimmed
                    .strip_prefix("endpoint=")
                    .or_else(|| trimmed.strip_prefix("endpoint:"))
                {
                    if rest.is_empty() {
                        Err(ParseBackendError(
                            "missing endpoint URL or name after 'endpoint='".to_string(),
                        ))
                    } else {
                        Ok(BackendKindCli::Endpoint(rest.to_string()))
                    }
                } else {
                    Err(ParseBackendError(format!(
                        "invalid backend '{s}'; expected one of: memory, qlever, endpoint=<URL_OR_NAME>"
                    )))
                }
            },
        }
    }
}

// How an `--endpoint`/`--backend endpoint=…` backend answers lookups.
//
// Parsed from the `--strategy` CLI flag. `sparql` (the default) queries the
// endpoint's SPARQL query service; `dereference` instead fetches each entity
// IRI directly over HTTP (`Accept: text/turtle`, following redirects) — the
// approach Wikibase instances like Wikidata and MaRDI support natively,
// since every entity is itself a dereferenceable Linked Data resource. See
// `rudof_rdf::rdf_impl::EndpointStrategy` for the trade-offs.
cli_wrapper!(EndpointStrategyCli, EndpointStrategy, { Sparql, Dereference });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_memory() {
        assert_eq!("memory".parse::<BackendKindCli>().unwrap(), BackendKindCli::Memory);
        assert_eq!("Memory".parse::<BackendKindCli>().unwrap(), BackendKindCli::Memory);
    }

    #[test]
    fn parses_qlever() {
        assert_eq!("qlever".parse::<BackendKindCli>().unwrap(), BackendKindCli::Qlever);
        assert_eq!("QLEVER".parse::<BackendKindCli>().unwrap(), BackendKindCli::Qlever);
    }

    #[test]
    fn parses_lbug() {
        assert_eq!("lbug".parse::<BackendKindCli>().unwrap(), BackendKindCli::Lbug);
        assert_eq!("LBUG".parse::<BackendKindCli>().unwrap(), BackendKindCli::Lbug);
    }

    #[test]
    fn parses_endpoint_url() {
        let parsed: BackendKindCli = "endpoint=https://query.wikidata.org/sparql".parse().unwrap();
        assert_eq!(
            parsed,
            BackendKindCli::Endpoint("https://query.wikidata.org/sparql".to_string())
        );
    }

    #[test]
    fn parses_endpoint_name() {
        let parsed: BackendKindCli = "endpoint=wikidata".parse().unwrap();
        assert_eq!(parsed, BackendKindCli::Endpoint("wikidata".to_string()));
    }

    #[test]
    fn preserves_case_in_endpoint_value() {
        let parsed: BackendKindCli = "endpoint=MyGraph".parse().unwrap();
        assert_eq!(parsed, BackendKindCli::Endpoint("MyGraph".to_string()));
    }

    #[test]
    fn rejects_bare_endpoint() {
        assert!("endpoint".parse::<BackendKindCli>().is_err());
        assert!("endpoint=".parse::<BackendKindCli>().is_err());
    }

    #[test]
    fn rejects_unknown_value() {
        assert!("foo".parse::<BackendKindCli>().is_err());
        assert!("".parse::<BackendKindCli>().is_err());
    }

    #[test]
    fn display_round_trips() {
        let cases = [
            BackendKindCli::Memory,
            BackendKindCli::Qlever,
            BackendKindCli::Lbug,
            BackendKindCli::Endpoint("https://x/sparql".to_string()),
        ];
        for c in cases {
            let s = c.to_string();
            let parsed: BackendKindCli = s.parse().unwrap();
            assert_eq!(parsed, c);
        }
    }

    fn args(backend: Option<BackendKindCli>, endpoint: Option<&str>) -> CommonArgsAll {
        CommonArgsAll {
            config: None,
            output: None,
            force_overwrite: false,
            backend,
            endpoint: endpoint.map(|s| s.to_string()),
        }
    }

    #[test]
    fn resolve_backend_prefers_endpoint_shortcut() {
        let spec = resolve_backend(&args(None, Some("https://query.wikidata.org/sparql")));
        assert_eq!(
            spec,
            BackendSpec::Endpoint("https://query.wikidata.org/sparql".to_string())
        );
    }

    #[test]
    fn resolve_backend_uses_backend_when_endpoint_absent() {
        let spec = resolve_backend(&args(Some(BackendKindCli::Qlever), None));
        assert_eq!(spec, BackendSpec::Qlever);
    }

    #[test]
    fn resolve_backend_defaults_to_memory_when_both_absent() {
        let spec = resolve_backend(&args(None, None));
        assert_eq!(spec, BackendSpec::Memory);
    }
}
