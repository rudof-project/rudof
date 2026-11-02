use prefixmap::PrefixMap;
use rudof_rdf::rdf_impl::{GraphDiff, GraphRelation, SubgraphMode, show_term, show_triple, used_prefixes};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// The differences between two RDF graphs, as returned by `compare_data(...).execute_diff()`,
/// with the prefixes of their documents to write them.
///
/// It dereferences to its [`GraphDiff`]. Displayed, it is a report for people: the
/// relation, the `PREFIX` declarations used, the triples only in each graph
/// (with prefixed names) and the blank node mapping.
#[derive(Debug, Clone)]
pub struct DataDiff {
    /// The differences.
    pub diff: GraphDiff,
    /// The prefixes of the first document, and those of the second document
    /// whose alias and IRI the first one does not have.
    pub prefixmap: PrefixMap,
    /// `true` if the first graph is the loaded data.
    pub first_is_loaded_data: bool,
}

impl Deref for DataDiff {
    type Target = GraphDiff;

    fn deref(&self) -> &GraphDiff {
        &self.diff
    }
}

impl DataDiff {
    /// `triple` in Turtle syntax, with the prefixes of the documents, without the final `.`.
    pub fn show_triple(&self, triple: &oxrdf::Triple) -> String {
        show_triple(triple, &self.prefixmap)
    }

    /// The prefixes used to write the triples and the blank node mapping.
    pub fn used_prefixes(&self) -> PrefixMap {
        let d = &self.diff;
        let triples = d
            .only_in_first
            .iter()
            .chain(&d.only_in_second)
            .map(oxrdf::TermRef::Triple);
        let mapped = d
            .first_to_second
            .iter()
            .chain(&d.second_to_first)
            .map(|(_, t)| t.as_ref());
        used_prefixes(triples.chain(mapped), &self.prefixmap)
    }

    /// How the report names the first and second graphs.
    fn names(&self) -> (&'static str, &'static str) {
        names(self.first_is_loaded_data)
    }
}

fn names(first_is_loaded_data: bool) -> (&'static str, &'static str) {
    if first_is_loaded_data {
        ("the loaded data", "the given graph")
    } else {
        ("the first graph", "the second graph")
    }
}

/// One line that explains `relation`, e.g. `subgraph: the first graph is
/// contained in the second graph, but not the other way round (mode: injective)`.
pub fn describe_relation(relation: GraphRelation, mode: SubgraphMode, first_is_loaded_data: bool) -> String {
    let (first, second) = names(first_is_loaded_data);
    let explanation = match relation {
        GraphRelation::Isomorphic => format!("{first} and {second} are equal up to blank node renaming"),
        GraphRelation::Equivalent => format!("{first} and {second} contain each other, but are not isomorphic"),
        GraphRelation::Subgraph => format!("{first} is contained in {second}, but not the other way round"),
        GraphRelation::Supergraph => format!("{second} is contained in {first}, but not the other way round"),
        GraphRelation::Different => format!("neither {first} nor {second} is contained in the other"),
    };
    format!("{relation}: {explanation} (mode: {mode})")
}

impl Display for DataDiff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let d = &self.diff;
        let (first, second) = self.names();
        writeln!(
            f,
            "{}",
            describe_relation(d.relation, d.mode, self.first_is_loaded_data)
        )?;
        let mut prefixes: Vec<(String, String)> = self
            .used_prefixes()
            .iter()
            .map(|(alias, iri)| (alias.clone(), iri.to_string()))
            .collect();
        prefixes.sort();
        for (alias, iri) in prefixes {
            writeln!(f, "PREFIX {alias}: <{iri}>")?;
        }
        for (name, triples) in [(first, &d.only_in_first), (second, &d.only_in_second)] {
            writeln!(f, "Only in {name} ({}):", triples.len())?;
            for t in triples {
                writeln!(f, "  {} .", self.show_triple(t))?;
            }
        }
        let (from, to, pairs) = if d.first_to_second.is_empty() {
            (second, first, &d.second_to_first)
        } else {
            (first, second, &d.first_to_second)
        };
        if !pairs.is_empty() {
            writeln!(f, "Blank nodes of {from} matched in {to}:")?;
            for (b, t) in pairs {
                writeln!(f, "  {b} = {}", show_term(t.as_ref(), &self.prefixmap))?;
            }
        }
        if !d.exact {
            writeln!(
                f,
                "The diff is approximate: the search for the best blank node mapping reached its limit. \
                 Use a larger exact search limit for an exact diff."
            )?;
        }
        Ok(())
    }
}
