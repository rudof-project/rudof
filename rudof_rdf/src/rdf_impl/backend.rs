//! Strategy enum unifying every concrete RDF backend.

use std::io;

use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, NamedOrBlankNode as OxSubject,
    Term as OxTerm, Triple as OxTriple,
};
use prefixmap::{PrefixMap, PrefixMapError};
use rudof_iri::IriS;

#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
use super::OxigraphEndpoint;
#[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
use super::QleverGraphContainer;
use super::{OxigraphInMemory, RdfBackendError};
#[cfg(feature = "sparql")]
use crate::rdf_core::query::{QueryRDF, QueryResultFormat, QuerySolution, QuerySolutions};
use crate::rdf_core::{BuildRDF, FocusRDF, Matcher, NeighsRDF, RDFFormat, Rdf};

/// Strategy enum that owns one concrete RDF backend.
#[derive(Debug, Clone)]
pub enum RdfBackend {
    InMemory(OxigraphInMemory),
    #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
    Endpoint(OxigraphEndpoint),
    #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
    Qlever(QleverGraphContainer),
}

impl RdfBackend {
    /// Convenience constructor for the default empty in-memory backend.
    pub fn in_memory() -> Self {
        RdfBackend::InMemory(OxigraphInMemory::new())
    }

    /// Return the wrapped in-memory backend if this variant is `InMemory`.
    pub fn as_in_memory(&self) -> Option<&OxigraphInMemory> {
        match self {
            RdfBackend::InMemory(g) => Some(g),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => None,
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(_) => None,
        }
    }

    /// Mutable accessor
    pub fn as_in_memory_mut(&mut self) -> Option<&mut OxigraphInMemory> {
        match self {
            RdfBackend::InMemory(g) => Some(g),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => None,
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(_) => None,
        }
    }

    /// True iff this backend has read-only triple semantics (`add_triple` / `remove_triple` would return `ReadOnly`).
    pub fn is_read_only(&self) -> bool {
        match self {
            RdfBackend::InMemory(_) => false,
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => true,
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(_) => true,
        }
    }

    /// Static name for diagnostics / `ReadOnly` errors.
    pub fn variant_name(&self) -> &'static str {
        match self {
            RdfBackend::InMemory(_) => "in-memory",
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => "sparql-endpoint",
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(_) => "qlever",
        }
    }
}

impl Default for RdfBackend {
    fn default() -> Self {
        RdfBackend::in_memory()
    }
}

impl Rdf for RdfBackend {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Triple = OxTriple;
    type Err = RdfBackendError;

    fn prefixmap(&self) -> Option<PrefixMap> {
        match self {
            RdfBackend::InMemory(b) => <OxigraphInMemory as Rdf>::prefixmap(b),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(b) => <OxigraphEndpoint as Rdf>::prefixmap(b),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => <QleverGraphContainer as Rdf>::prefixmap(b),
        }
    }

    fn qualify_iri(&self, node: &Self::IRI) -> String {
        match self {
            RdfBackend::InMemory(b) => b.qualify_iri(node),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(b) => b.qualify_iri(node),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => b.qualify_iri(node),
        }
    }

    fn qualify_subject(&self, subj: &Self::Subject) -> String {
        match self {
            RdfBackend::InMemory(b) => b.qualify_subject(subj),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(b) => b.qualify_subject(subj),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => b.qualify_subject(subj),
        }
    }

    fn qualify_term(&self, term: &Self::Term) -> String {
        match self {
            RdfBackend::InMemory(b) => b.qualify_term(term),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(b) => b.qualify_term(term),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => b.qualify_term(term),
        }
    }

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        match self {
            RdfBackend::InMemory(b) => b.resolve_prefix_local(prefix, local),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(b) => b.resolve_prefix_local(prefix, local),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => b.resolve_prefix_local(prefix, local),
        }
    }
}

impl NeighsRDF for RdfBackend {
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        let iter: Box<dyn Iterator<Item = OxTriple> + '_> = match self {
            RdfBackend::InMemory(b) => Box::new(b.triples()?),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(b) => Box::new(b.triples()?),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => {
                let v: Vec<OxTriple> = b.triples()?.collect();
                Box::new(v.into_iter())
            },
        };
        Ok(iter)
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
        let iter: Box<dyn Iterator<Item = OxTriple> + '_> = match self {
            RdfBackend::InMemory(b) => Box::new(b.triples_matching(subject, predicate, object)?),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(b) => Box::new(b.triples_matching(subject, predicate, object)?),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => {
                let v: Vec<OxTriple> = b.triples_matching(subject, predicate, object)?.collect();
                Box::new(v.into_iter())
            },
        };
        Ok(iter)
    }

    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: &[Self::IRI],
    ) -> std::result::Result<
        (std::collections::HashMap<Self::IRI, std::collections::HashSet<Self::Term>>, Vec<Self::IRI>),
        Self::Err,
    > {
        #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
        if let RdfBackend::Endpoint(b) = self {
            return b.outgoing_arcs_from_list(subject, preds).map_err(Into::into);
        }
        // For InMemory and Qlever: default behavior — fetch all triples and filter in memory
        if preds.is_empty() {
            return Ok((std::collections::HashMap::new(), Vec::new()));
        }
        let mut results: std::collections::HashMap<OxNamedNode, std::collections::HashSet<OxTerm>> =
            std::collections::HashMap::new();
        let mut reminder: Vec<OxNamedNode> = Vec::new();
        for triple in self.triples_with_subject(subject)? {
            let p = triple.predicate.clone();
            let o = triple.object.clone();
            if preds.contains(&p) {
                results.entry(p).or_default().insert(o);
            } else {
                reminder.push(p);
            }
        }
        Ok((results, reminder))
    }
}

#[cfg(feature = "sparql")]
impl QueryRDF for RdfBackend {
    fn query_select(&self, query_str: &str) -> Result<QuerySolutions<Self>, Self::Err>
    where
        Self: Sized,
    {
        let (raw, pm): (Vec<QuerySolution<Self>>, PrefixMap) = match self {
            RdfBackend::InMemory(b) => {
                let s = b.query_select(query_str)?;
                let pm = b.prefixmap().clone();
                (s.iter().map(|x| x.convert(|t| t.clone())).collect(), pm)
            },
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(b) => {
                let s = b.query_select(query_str)?;
                let pm = b.prefixmap().clone();
                (s.iter().map(|x| x.convert(|t| t.clone())).collect(), pm)
            },
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => {
                let s = b.query_select(query_str)?;
                let pm = <QleverGraphContainer as Rdf>::prefixmap(b).unwrap_or_default();
                (s.iter().map(|x| x.convert(|t| t.clone())).collect(), pm)
            },
        };
        Ok(QuerySolutions::new(raw, pm))
    }

    fn query_construct(&self, query_str: &str, format: &QueryResultFormat) -> Result<String, Self::Err>
    where
        Self: Sized,
    {
        match self {
            RdfBackend::InMemory(b) => Ok(b.query_construct(query_str, format)?),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(b) => Ok(b.query_construct(query_str, format)?),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => Ok(b.query_construct(query_str, format)?),
        }
    }

    fn query_ask(&self, query_str: &str) -> Result<bool, Self::Err> {
        match self {
            RdfBackend::InMemory(b) => Ok(b.query_ask(query_str)?),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(b) => Ok(b.query_ask(query_str)?),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => Ok(b.query_ask(query_str)?),
        }
    }
}

impl FocusRDF for RdfBackend {
    fn set_focus(&mut self, focus: &Self::Term) {
        match self {
            RdfBackend::InMemory(b) => b.set_focus(focus),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => {},
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => b.set_focus(focus),
        }
    }

    fn get_focus(&self) -> Option<&Self::Term> {
        match self {
            RdfBackend::InMemory(b) => b.get_focus(),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => None,
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => b.get_focus(),
        }
    }
}

impl BuildRDF for RdfBackend {
    fn empty() -> Self {
        RdfBackend::in_memory()
    }

    fn add_base(&mut self, base: &Option<IriS>) {
        match self {
            RdfBackend::InMemory(b) => b.add_base(base),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => {},
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => b.add_base(base),
        }
    }

    fn add_prefix(&mut self, alias: &str, iri: &IriS) {
        match self {
            RdfBackend::InMemory(b) => b.add_prefix(alias, iri),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => {},
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => b.add_prefix(alias, iri),
        }
    }

    fn set_prefix_map(&mut self, prefix_map: PrefixMap) {
        match self {
            RdfBackend::InMemory(b) => b.set_prefix_map(prefix_map),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => {},
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => b.set_prefix_map(prefix_map),
        }
    }

    fn merge_prefixes(&mut self, prefix_map: PrefixMap) {
        match self {
            RdfBackend::InMemory(b) => b.merge_prefixes(prefix_map),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => {},
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => b.merge_prefixes(prefix_map),
        }
    }

    fn add_bnode(&mut self) -> Result<Self::BNode, Self::Err> {
        match self {
            RdfBackend::InMemory(b) => Ok(b.add_bnode()?),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => Err(RdfBackendError::ReadOnly {
                op: "add_bnode",
                backend: self.variant_name(),
            }),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(_) => Err(RdfBackendError::ReadOnly {
                op: "add_bnode",
                backend: "qlever",
            }),
        }
    }

    fn add_triple<S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>,
    {
        match self {
            RdfBackend::InMemory(b) => Ok(b.add_triple(subj, pred, obj)?),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => Err(RdfBackendError::ReadOnly {
                op: "add_triple",
                backend: "sparql-endpoint",
            }),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(_) => Err(RdfBackendError::ReadOnly {
                op: "add_triple",
                backend: "qlever",
            }),
        }
    }

    fn remove_triple<S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>,
    {
        match self {
            RdfBackend::InMemory(b) => Ok(b.remove_triple(subj, pred, obj)?),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => Err(RdfBackendError::ReadOnly {
                op: "remove_triple",
                backend: "sparql-endpoint",
            }),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(_) => Err(RdfBackendError::ReadOnly {
                op: "remove_triple",
                backend: "qlever",
            }),
        }
    }

    fn add_type<S, T>(&mut self, node: S, type_: T) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        T: Into<Self::Term>,
    {
        match self {
            RdfBackend::InMemory(b) => Ok(b.add_type(node, type_)?),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => Err(RdfBackendError::ReadOnly {
                op: "add_type",
                backend: "sparql-endpoint",
            }),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(_) => Err(RdfBackendError::ReadOnly {
                op: "add_type",
                backend: "qlever",
            }),
        }
    }

    fn serialize<W: io::Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), Self::Err> {
        match self {
            RdfBackend::InMemory(b) => Ok(BuildRDF::serialize(b, format, writer)?),
            #[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
            RdfBackend::Endpoint(_) => Err(RdfBackendError::ReadOnly {
                op: "serialize",
                backend: "sparql-endpoint",
            }),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            RdfBackend::Qlever(b) => Ok(BuildRDF::serialize(b, format, writer)?),
        }
    }
}
