use iri_s::IriS;
use prefixmap::PrefixMap;
use pretty_print::{helpers, PrettyBuilder, PrettyProvider, PrettyTree};
/// This file converts ShEx AST to ShEx compact syntax
use shex_ast::Schema;

struct ShExCompactPrinter {
    width: usize,
}

impl ShExCompactPrinter {
    pub fn new(width: usize) -> ShExCompactPrinter {
        ShExCompactPrinter { width }
    }

    pub fn print_schema<'a>(&self, schema: &Schema) -> String {
        let mut w = Vec::new();
        pp_schema(&schema).render(self.width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

fn pp_schema(schema: &Schema) -> PrettyTree {
    opt_pp(schema.prefixmap(), pp_prefix_map)
}

fn pp_prefix_map(pm: &PrefixMap) -> PrettyTree {
    let mut pms = Vec::new();
    for (alias, iri) in pm.map.clone().into_iter() {
        pms.push(
            PrettyTree::text("prefix")
                .append(space())
                .append(PrettyTree::text(alias))
                .append(PrettyTree::text(":"))
                .append(space())
                .append(pp_iri(&iri)),
        )
    }
    PrettyTree::join(pms, PrettyTree::line_or_nil())
}

fn space() -> PrettyTree {
    PrettyTree::text(" ")
}

fn pp_iri(iri: &IriS) -> PrettyTree {
    PrettyTree::text("<")
        .append(PrettyTree::text(iri.to_string()))
        .append(PrettyTree::text(">"))
}

fn opt_pp<A>(maybe_a: Option<A>, pp: impl Fn(&A) -> PrettyTree) -> PrettyTree {
    match maybe_a {
        None => PrettyTree::Nil,
        Some(ref a) => pp(a),
    }
}

#[cfg(test)]
mod tests {
    use iri_s::IriS;
    use prefixmap::PrefixMap;

    use super::*;

    #[test]
    fn empty_schema() {
        let mut pm = PrefixMap::new();
        pm.insert("", &IriS::new_unchecked("http://example.org/"));
        pm.insert("schema", &IriS::new_unchecked("https://schema.org/"));
        let schema = Schema::new().with_prefixmap(Some(pm));
        let s = ShExCompactPrinter::new(10).print_schema(&schema);
        assert_eq!(
            s,
            "prefix : <http://example.org/>\nprefix schema: <https://schema.org/>"
        );
    }
}
