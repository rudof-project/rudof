use crate::Rudof;
use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;

/// Which directive syntax to emit a default prefix declaration in.
pub(crate) enum PrefixDirective {
    /// Turtle/TriG/N3's `@prefix alias: <iri> .`
    Turtle,
    /// SPARQL/ShExC's `PREFIX alias: <iri>` (no trailing period).
    Sparql,
}

static TURTLE_PREFIX_DECL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^\s*@prefix\s+([^\s:]*)\s*:").unwrap());
static SPARQL_PREFIX_DECL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?mi)^\s*PREFIX\s+([^\s:]*)\s*:").unwrap());

/// Builds prefix-declaration lines, in `directive`'s syntax, for every alias
/// in `rudof`'s default prefixes that `source` doesn't already declare
/// itself -- ready to prepend to `source` before parsing it, so a prefixed
/// name the document doesn't declare can still resolve, without ever
/// overriding a declaration the document does have.
///
/// Returns an empty string if there are no default prefixes to add, so
/// callers can skip the prepend (and the buffering it requires) entirely.
pub(crate) fn default_prefix_header(rudof: &Rudof, source: &str, directive: PrefixDirective) -> String {
    let Some(defaults) = rudof.prefixes.as_ref() else {
        return String::new();
    };
    if defaults.is_empty() {
        return String::new();
    }

    let declared_regex = match directive {
        PrefixDirective::Turtle => &*TURTLE_PREFIX_DECL,
        PrefixDirective::Sparql => &*SPARQL_PREFIX_DECL,
    };
    let declared: HashSet<&str> = declared_regex
        .captures_iter(source)
        .map(|c| c.get(1).map_or("", |m| m.as_str()))
        .collect();

    let mut header = String::new();
    for (alias, iri) in defaults.iter() {
        if declared.contains(alias.as_str()) {
            continue;
        }
        match directive {
            PrefixDirective::Turtle => header.push_str(&format!("@prefix {alias}: <{iri}> .\n")),
            PrefixDirective::Sparql => header.push_str(&format!("PREFIX {alias}: <{iri}>\n")),
        }
    }
    header
}
