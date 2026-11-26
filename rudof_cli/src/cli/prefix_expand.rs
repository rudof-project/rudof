use prefixmap::PrefixMap;

/// If `token` is a prefixed name (`alias:local`, e.g. `es:E10`) whose `alias`
/// is a known prefix — checked against `default_prefixes` first, then
/// `endpoint_prefixes` if given — returns the expanded IRI as a string.
/// `None` for anything else (an existing URL, a file path, an unregistered
/// alias, `-` for stdin, ...), leaving the token to be parsed as a normal
/// `InputSpec`.
///
/// Shared between the interactive shell (which resolves against the active
/// endpoint's own prefixes) and the top-level CLI (which resolves against
/// the endpoint named by `--endpoint`/`-e`, if any).
pub fn resolve_prefixed_resource(
    token: &str,
    default_prefixes: &PrefixMap,
    endpoint_prefixes: Option<&PrefixMap>,
) -> Option<String> {
    if token == "-" || token.contains("://") {
        return None;
    }
    let (alias, local) = token.split_once(':')?;
    if alias.is_empty() || local.is_empty() {
        return None;
    }

    let pm = if default_prefixes.find(alias).is_some() {
        default_prefixes
    } else {
        endpoint_prefixes.filter(|pm| pm.find(alias).is_some())?
    };

    pm.resolve_prefix_local(alias.to_string(), local.to_string())
        .ok()
        .map(|iri| iri.to_string())
}
