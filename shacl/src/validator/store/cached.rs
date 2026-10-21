//! A caching view of [`RdfData`] for validating against remote SPARQL endpoints.
//!
//! Every lookup the validator makes ends up in [`NeighsRDF::triples_matching`].
//! Against an endpoint, each of those is an HTTP request, so validating many
//! nodes costs many requests, and the same `(node, predicate)` pair is often
//! asked for more than once (by several shapes, or by `sh:class` checks of the
//! same type). [`CachedRdfData`] answers the lookups whose result it already
//! holds *completely* from memory, and forwards the others to the wrapped data,
//! keeping their result.
//!
//! The cache can also be filled in advance, many nodes per query, with the
//! `prefetch_*` methods: [`crate::validator::prefetch`] uses them to fetch the
//! data a shapes graph needs shape by shape, before validation starts.
//!
//! Only patterns the cache can answer exactly are served from it: a subject
//! and a predicate (`s p ?o`), a subject alone (`s ?p ?o`), or a predicate and
//! an object (`?s p o`). Anything else is forwarded unchanged.

use prefixmap::{PrefixMap, PrefixMapError};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::query::{QueryRDF, QueryResultFormat, QuerySolutions};
use rudof_rdf::rdf_core::{Any, Matcher, NeighsRDF, Rdf};
use sparql_service::{RdfData, RdfDataError};
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::debug;

use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, NamedOrBlankNode as OxSubject,
    Term as OxTerm, Triple as OxTriple,
};

/// Default number of nodes one prefetch query asks about.
pub const DEFAULT_PREFETCH_BATCH_SIZE: usize = 50;

/// A prefetch query returning at least this many rows may have been cut off
/// by the endpoint (some silently cap their results, e.g. Virtuoso at 10000
/// rows), so its result is not cached: the lookups then fetch these nodes one
/// at a time.
const MAX_PREFETCH_ROWS: usize = 10_000;

/// See the [module documentation](self).
#[derive(Debug)]
pub struct CachedRdfData {
    inner: RdfData,
    batch_size: usize,
    /// `(s, p)` → every `o` with `s p o`.
    outgoing: RwLock<HashMap<(OxSubject, OxNamedNode), Vec<OxTerm>>>,
    /// `s` → every `(p, o)` with `s p o`.
    all_outgoing: RwLock<HashMap<OxSubject, Vec<(OxNamedNode, OxTerm)>>>,
    /// `(o, p)` → every `s` with `s p o`.
    incoming: RwLock<HashMap<(OxTerm, OxNamedNode), Vec<OxSubject>>>,
    /// Lookups that had to go to the wrapped data.
    forwarded: AtomicUsize,
    /// Queries sent by the `prefetch_*` methods.
    prefetch_queries: AtomicUsize,
}

impl CachedRdfData {
    pub fn new(inner: RdfData) -> Self {
        CachedRdfData {
            inner,
            batch_size: DEFAULT_PREFETCH_BATCH_SIZE,
            outgoing: RwLock::new(HashMap::new()),
            all_outgoing: RwLock::new(HashMap::new()),
            incoming: RwLock::new(HashMap::new()),
            forwarded: AtomicUsize::new(0),
            prefetch_queries: AtomicUsize::new(0),
        }
    }

    /// Sets how many nodes one prefetch query asks about (at least 1).
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size.max(1);
        self
    }

    pub fn inner(&self) -> &RdfData {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut RdfData {
        &mut self.inner
    }

    pub fn into_inner(self) -> RdfData {
        self.inner
    }

    /// Number of lookups that the cache could not answer and forwarded to the
    /// wrapped data.
    pub fn forwarded_lookups(&self) -> usize {
        self.forwarded.load(Ordering::Relaxed)
    }

    /// Number of queries sent by the `prefetch_*` methods.
    pub fn prefetch_queries(&self) -> usize {
        self.prefetch_queries.load(Ordering::Relaxed)
    }

    /// Fetches, for each of `subjects`, the objects of each of `preds` (`s p ?o`).
    /// Pairs already cached are skipped; blank nodes, which another query
    /// cannot name, are left to the lookups.
    pub fn prefetch_outgoing(&self, subjects: &[OxTerm], preds: &[OxNamedNode]) -> Result<(), RdfDataError> {
        if preds.is_empty() {
            return Ok(());
        }
        let todo: Vec<OxNamedNode> = {
            let outgoing = self.outgoing.read().unwrap();
            let all_outgoing = self.all_outgoing.read().unwrap();
            unique(subjects.iter().filter_map(|s| match s {
                OxTerm::NamedNode(n) => Some(n.clone()),
                _ => None,
            }))
            .into_iter()
            .filter(|n| {
                let subject = OxSubject::NamedNode(n.clone());
                !all_outgoing.contains_key(&subject)
                    && preds
                        .iter()
                        .any(|p| !outgoing.contains_key(&(subject.clone(), p.clone())))
            })
            .collect()
        };
        let preds = unique(preds.iter().cloned());
        for batch in todo.chunks(self.batch_size) {
            let values = batch.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
            let query = format!(
                "SELECT ?s ?p ?o WHERE {{ VALUES ?s {{ {values} }} {} }}",
                union_of(&preds, |p| format!("{{ ?s {p} ?o BIND({p} AS ?p) }}"))
            );
            let Some(rows) = self.run_prefetch(&query)? else {
                continue;
            };
            let mut outgoing = self.outgoing.write().unwrap();
            for n in batch {
                for p in &preds {
                    outgoing
                        .entry((OxSubject::NamedNode(n.clone()), p.clone()))
                        .or_default();
                }
            }
            for (s, p, o) in rows {
                if let (Ok(s), OxTerm::NamedNode(p)) = (OxSubject::try_from(s), p) {
                    outgoing.entry((s, p)).or_default().push(o);
                }
            }
        }
        Ok(())
    }

    /// Fetches, for each of `objects`, the subjects of each of `preds` (`?s p o`).
    /// Pairs already cached are skipped, and so are blank nodes.
    pub fn prefetch_incoming(&self, objects: &[OxTerm], preds: &[OxNamedNode]) -> Result<(), RdfDataError> {
        if preds.is_empty() {
            return Ok(());
        }
        let todo: Vec<OxTerm> = {
            let incoming = self.incoming.read().unwrap();
            unique(objects.iter().filter(|o| !matches!(o, OxTerm::BlankNode(_))).cloned())
                .into_iter()
                .filter(|o| preds.iter().any(|p| !incoming.contains_key(&(o.clone(), p.clone()))))
                .collect()
        };
        let preds = unique(preds.iter().cloned());
        for batch in todo.chunks(self.batch_size) {
            let values = batch.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(" ");
            let query = format!(
                "SELECT ?s ?p ?o WHERE {{ VALUES ?o {{ {values} }} {} }}",
                union_of(&preds, |p| format!("{{ ?s {p} ?o BIND({p} AS ?p) }}"))
            );
            let Some(rows) = self.run_prefetch(&query)? else {
                continue;
            };
            let mut incoming = self.incoming.write().unwrap();
            for o in batch {
                for p in &preds {
                    incoming.entry((o.clone(), p.clone())).or_default();
                }
            }
            for (s, p, o) in rows {
                if let (Ok(s), OxTerm::NamedNode(p)) = (OxSubject::try_from(s), p) {
                    incoming.entry((o, p)).or_default().push(s);
                }
            }
        }
        Ok(())
    }

    /// Fetches every triple whose subject is one of `subjects` (`s ?p ?o`).
    pub fn prefetch_all_outgoing(&self, subjects: &[OxTerm]) -> Result<(), RdfDataError> {
        let todo: Vec<OxNamedNode> = {
            let all_outgoing = self.all_outgoing.read().unwrap();
            unique(subjects.iter().filter_map(|s| match s {
                OxTerm::NamedNode(n) => Some(n.clone()),
                _ => None,
            }))
            .into_iter()
            .filter(|n| !all_outgoing.contains_key(&OxSubject::NamedNode(n.clone())))
            .collect()
        };
        for batch in todo.chunks(self.batch_size) {
            let values = batch.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
            let query = format!("SELECT ?s ?p ?o WHERE {{ VALUES ?s {{ {values} }} ?s ?p ?o }}");
            let Some(rows) = self.run_prefetch(&query)? else {
                continue;
            };
            let mut all_outgoing = self.all_outgoing.write().unwrap();
            for n in batch {
                all_outgoing.entry(OxSubject::NamedNode(n.clone())).or_default();
            }
            for (s, p, o) in rows {
                if let (Ok(s), OxTerm::NamedNode(p)) = (OxSubject::try_from(s), p) {
                    all_outgoing.entry(s).or_default().push((p, o));
                }
            }
        }
        Ok(())
    }

    /// Runs a prefetch query binding `?s ?p ?o`, returning its rows, or `None`
    /// when the result may have been cut off by the endpoint.
    fn run_prefetch(&self, query: &str) -> Result<Option<Vec<(OxTerm, OxTerm, OxTerm)>>, RdfDataError> {
        self.prefetch_queries.fetch_add(1, Ordering::Relaxed);
        let solutions = self.inner.query_select(query)?;
        let rows: Vec<(OxTerm, OxTerm, OxTerm)> = solutions
            .iter()
            .filter_map(|solution| {
                Some((
                    solution.find_solution("s")?.clone(),
                    solution.find_solution("p")?.clone(),
                    solution.find_solution("o")?.clone(),
                ))
            })
            .collect();
        if rows.len() >= MAX_PREFETCH_ROWS {
            debug!(rows = rows.len(), "prefetch: result may be truncated, not caching it");
            return Ok(None);
        }
        Ok(Some(rows))
    }

    /// The triples matching `(s, p, o)`, from the cache when it holds them all.
    fn cached_triples(
        &self,
        s: Option<&OxSubject>,
        p: Option<&OxNamedNode>,
        o: Option<&OxTerm>,
    ) -> Option<Vec<OxTriple>> {
        match (s, p, o) {
            (Some(s), Some(p), _) => {
                if let Some(objects) = self.outgoing.read().unwrap().get(&(s.clone(), p.clone())) {
                    return Some(
                        objects
                            .iter()
                            .filter(|obj| o.is_none_or(|o| o == *obj))
                            .map(|obj| OxTriple::new(s.clone(), p.clone(), obj.clone()))
                            .collect(),
                    );
                }
                self.all_outgoing.read().unwrap().get(s).map(|arcs| {
                    arcs.iter()
                        .filter(|(pred, obj)| pred == p && o.is_none_or(|o| o == obj))
                        .map(|(pred, obj)| OxTriple::new(s.clone(), pred.clone(), obj.clone()))
                        .collect()
                })
            },
            (Some(s), None, _) => self.all_outgoing.read().unwrap().get(s).map(|arcs| {
                arcs.iter()
                    .filter(|(_, obj)| o.is_none_or(|o| o == obj))
                    .map(|(pred, obj)| OxTriple::new(s.clone(), pred.clone(), obj.clone()))
                    .collect()
            }),
            (None, Some(p), Some(o)) => self
                .incoming
                .read()
                .unwrap()
                .get(&(o.clone(), p.clone()))
                .map(|subjects| {
                    subjects
                        .iter()
                        .map(|subj| OxTriple::new(subj.clone(), p.clone(), o.clone()))
                        .collect()
                }),
            _ => None,
        }
    }

    /// Forwards a lookup to the wrapped data. A pattern the cache can hold is
    /// fetched without its object bound, so that the full result can be kept.
    fn forward(
        &self,
        s: Option<&OxSubject>,
        p: Option<&OxNamedNode>,
        o: Option<&OxTerm>,
    ) -> Result<Vec<OxTriple>, RdfDataError> {
        self.forwarded.fetch_add(1, Ordering::Relaxed);
        let s_m = AnyOr(s.cloned());
        let p_m = AnyOr(p.cloned());
        match (s, p, o) {
            (Some(s), Some(p), _) => {
                let triples: Vec<OxTriple> = self.inner.triples_matching(&s_m, &p_m, &Any)?.collect();
                self.outgoing.write().unwrap().insert(
                    (s.clone(), p.clone()),
                    triples.iter().map(|t| t.object.clone()).collect(),
                );
                Ok(triples
                    .into_iter()
                    .filter(|t| o.is_none_or(|o| *o == t.object))
                    .collect())
            },
            (Some(s), None, _) => {
                let triples: Vec<OxTriple> = self.inner.triples_matching(&s_m, &Any, &Any)?.collect();
                self.all_outgoing.write().unwrap().insert(
                    s.clone(),
                    triples
                        .iter()
                        .map(|t| (t.predicate.clone(), t.object.clone()))
                        .collect(),
                );
                Ok(triples
                    .into_iter()
                    .filter(|t| o.is_none_or(|o| *o == t.object))
                    .collect())
            },
            (None, Some(p), Some(o)) => {
                let o_m = AnyOr(Some(o.clone()));
                let triples: Vec<OxTriple> = self.inner.triples_matching(&Any, &p_m, &o_m)?.collect();
                self.incoming.write().unwrap().insert(
                    (o.clone(), p.clone()),
                    triples.iter().map(|t| t.subject.clone()).collect(),
                );
                Ok(triples)
            },
            _ => {
                let o_m = AnyOr(o.cloned());
                Ok(self.inner.triples_matching(&s_m, &p_m, &o_m)?.collect())
            },
        }
    }
}

/// A matcher for an optional value: the value itself, or anything.
#[derive(Debug, Clone, PartialEq)]
struct AnyOr<T>(Option<T>);

impl<T: PartialEq> PartialEq<T> for AnyOr<T> {
    fn eq(&self, other: &T) -> bool {
        self.0.as_ref().is_none_or(|v| v == other)
    }
}

impl<T: PartialEq> Matcher<T> for AnyOr<T> {
    fn value(&self) -> Option<&T> {
        self.0.as_ref()
    }
}

fn unique<T: Clone + Eq + std::hash::Hash>(items: impl IntoIterator<Item = T>) -> Vec<T> {
    let mut seen = HashSet::new();
    items.into_iter().filter(|i| seen.insert(i.clone())).collect()
}

fn union_of(preds: &[OxNamedNode], branch: impl Fn(&OxNamedNode) -> String) -> String {
    preds.iter().map(branch).collect::<Vec<_>>().join(" UNION ")
}

impl From<RdfData> for CachedRdfData {
    fn from(inner: RdfData) -> Self {
        CachedRdfData::new(inner)
    }
}

impl Rdf for CachedRdfData {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Triple = OxTriple;
    type Err = RdfDataError;

    fn prefixmap(&self) -> Option<PrefixMap> {
        self.inner.prefixmap()
    }

    fn qualify_iri(&self, iri: &Self::IRI) -> String {
        self.inner.qualify_iri(iri)
    }

    fn qualify_subject(&self, subj: &Self::Subject) -> String {
        self.inner.qualify_subject(subj)
    }

    fn qualify_term(&self, term: &Self::Term) -> String {
        self.inner.qualify_term(term)
    }

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        self.inner.resolve_prefix_local(prefix, local)
    }
}

impl NeighsRDF for CachedRdfData {
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        let triples: Vec<OxTriple> = self.inner.triples()?.collect();
        Ok(triples.into_iter())
    }

    fn triples_matching<S, P, O>(
        &self,
        subject: &S,
        predicate: &P,
        object: &O,
    ) -> Result<impl Iterator<Item = Self::Triple> + '_, Self::Err>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        let (s, p, o) = (subject.value(), predicate.value(), object.value());
        let triples = match self.cached_triples(s, p, o) {
            Some(triples) => triples,
            None => self.forward(s, p, o)?,
        };
        Ok(triples.into_iter())
    }
}

impl QueryRDF for CachedRdfData {
    fn query_construct(&self, query: &str, format: &QueryResultFormat) -> Result<String, Self::Err> {
        self.inner.query_construct(query, format)
    }

    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>, Self::Err> {
        let solutions = self.inner.query_select(query)?;
        let pm = self.inner.prefixmap().unwrap_or_default();
        let mut converted = QuerySolutions::empty();
        converted.extend(solutions.iter().map(|s| s.convert(|t| t.clone())).collect(), pm);
        Ok(converted)
    }

    fn query_ask(&self, query: &str) -> Result<bool, Self::Err> {
        self.inner.query_ask(query)
    }
}
