use rudof_rdf::rdf_core::SHACLPath;

mod basic_validator;

/// Converts a [`SHACLPath`] to its SPARQL property path [String] representation.
///
/// IRIs are enclosed in angle brackets and path operators follow SPARQL 1.1 property path syntax.
fn path_to_sparql(path: &SHACLPath) -> String {
    match path {
        SHACLPath::Predicate { pred } => format!("<{pred}>"),
        SHACLPath::Alternative { paths } => {
            let parts: Vec<_> = paths.iter().map(path_to_sparql).collect();
            format!("({})", parts.join("|"))
        },
        SHACLPath::Sequence { paths } => {
            let parts: Vec<_> = paths.iter().map(path_to_sparql).collect();
            format!("({})", parts.join("/"))
        },
        SHACLPath::Inverse { path } => format!("^({})", path_to_sparql(path)),
        SHACLPath::ZeroOrMore { path } => format!("({})*", path_to_sparql(path)),
        SHACLPath::OneOrMore { path } => format!("({})+", path_to_sparql(path)),
        SHACLPath::ZeroOrOne { path } => format!("({})?", path_to_sparql(path)),
    }
}

fn inject_values_into_where(query: &str, values_clause: &str) -> String {
    let upper = query.to_uppercase();
    if let Some(where_pos) = upper.find("WHERE") {
        if let Some(brace_offset) = query[where_pos..].find('{') {
            let insert_at = where_pos + brace_offset + 1;
            let mut result = query[..insert_at].to_string();
            result.push(' ');
            result.push_str(values_clause);
            result.push_str(&query[insert_at..]);
            return result;
        }
    }

    format!("{values_clause} {query}")
}
