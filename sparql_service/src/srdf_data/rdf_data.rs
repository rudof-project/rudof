use super::RdfDataError;
use colored::*;
use iri_s::IriS;
use oxigraph::sparql::Query;
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;
use oxiri::Iri;
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, Subject as OxSubject,
    Term as OxTerm,
};
use oxrdfio::RdfFormat;
use prefixmap::IriRef;
use prefixmap::PrefixMap;
use rust_decimal::Decimal;
use sparesults::QuerySolution;
use srdf::lang::Lang;
use srdf::literal::Literal;
use srdf::numeric_literal::NumericLiteral;
use srdf::ListOfIriAndTerms;
use srdf::Object;
use srdf::QuerySRDF2;
use srdf::QuerySolution2;
use srdf::QuerySolutions;
use srdf::RDFFormat;
use srdf::ReaderMode;
use srdf::SRDFBasic;
use srdf::SRDFBuilder;
use srdf::SRDFGraph;
use srdf::SRDFSparql;
use srdf::VarName2;
use srdf::RDF_TYPE_STR;
use srdf::SRDF;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::io;
use std::rc::Rc;
use std::str::FromStr;

/// Generic abstraction that represents RDF Data which can be either behind SPARQL endpoints or an in-memory graph
#[derive(Clone)]
pub struct RdfData {
    endpoints: Vec<SRDFSparql>,
    graph: Option<SRDFGraph>,
    store: Option<Store>,
}

impl Debug for RdfData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RdfData")
            .field("endpoints", &self.endpoints)
            .field("graph", &self.graph)
            .finish()
    }
}

impl RdfData {
    pub fn new() -> RdfData {
        RdfData {
            endpoints: Vec::new(),
            graph: None,
            store: None,
        }
    }

    pub fn from_graph(graph: SRDFGraph) -> Result<RdfData, RdfDataError> {
        let store = Store::new()?;
        store.bulk_loader().load_quads(graph.quads())?;
        Ok(RdfData {
            endpoints: Vec::new(),
            graph: Some(graph),
            store: Some(store),
        })
    }

    // Cleans the values of endpoints and graph
    pub fn clean_all(&mut self) {
        self.endpoints = Vec::new();
        self.graph = None
    }

    // Cleans the value graph
    pub fn clean_graph(&mut self) {
        self.graph = None
    }

    pub fn merge_from_reader<R: io::Read>(
        &mut self,
        read: R,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), RdfDataError> {
        let base = base.map(|str| Iri::parse_unchecked(str.to_string()));
        match &mut self.graph {
            Some(ref mut graph) => graph
                .merge_from_reader(read, format, base, reader_mode)
                .map_err(|e| RdfDataError::SRDFGraphError { err: e }),
            None => {
                let mut graph = SRDFGraph::new();
                graph
                    .merge_from_reader(read, format, base, reader_mode)
                    .map_err(|e| RdfDataError::SRDFGraphError { err: e })?;
                self.graph = Some(graph);
                Ok(())
            }
        }
    }

    pub fn from_endpoint(endpoint: SRDFSparql) -> RdfData {
        RdfData {
            endpoints: vec![endpoint],
            graph: None,
            store: None,
        }
    }

    /// Add a new endpoint to the list of endpoints
    pub fn add_endpoint(&mut self, endpoint: SRDFSparql) {
        // TODO: Ensure that there are no repeated endpoints
        self.endpoints.push(endpoint);
    }

    pub fn prefixmap_in_memory(&self) -> PrefixMap {
        self.graph
            .as_ref()
            .map(|g| g.prefixmap())
            .unwrap_or_default()
    }

    pub fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        let str: String = format!("{}", bn);
        format!("{}", str.green())
    }

    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        let str: String = format!("{}", lit);
        format!("{}", str.red())
    }
}

impl Default for RdfData {
    fn default() -> Self {
        Self::new()
    }
}

impl SRDFBasic for RdfData {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = RdfDataError;

    fn prefixmap(&self) -> std::option::Option<PrefixMap> {
        self.graph.as_ref().map(|g| g.prefixmap())
    }

    fn subject_as_iri(subject: &Self::Subject) -> Option<Self::IRI> {
        match subject {
            OxSubject::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }

    fn subject_as_bnode(subject: &Self::Subject) -> Option<Self::BNode> {
        match subject {
            OxSubject::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }

    fn subject_is_iri(subject: &Self::Subject) -> bool {
        matches!(subject, OxSubject::NamedNode(_))
    }

    fn subject_is_bnode(subject: &Self::Subject) -> bool {
        matches!(subject, OxSubject::BlankNode(_))
    }

    fn term_as_iri(object: &Self::Term) -> Option<Self::IRI> {
        match object {
            OxTerm::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }

    fn term_as_bnode(object: &Self::Term) -> Option<Self::BNode> {
        match object {
            OxTerm::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }

    fn term_as_literal(object: &Self::Term) -> Option<Self::Literal> {
        match object {
            OxTerm::Literal(l) => Some(l.clone()),
            _ => None,
        }
    }

    fn term_as_object(term: &Self::Term) -> srdf::Object {
        match term {
            OxTerm::BlankNode(bn) => Object::BlankNode(bn.as_str().to_string()),
            OxTerm::Literal(lit) => {
                let lit = lit.to_owned();
                match lit.destruct() {
                    (s, None, None) => Object::Literal(Literal::StringLiteral {
                        lexical_form: s,
                        lang: None,
                    }),
                    (s, None, Some(lang)) => Object::Literal(Literal::StringLiteral {
                        lexical_form: s,
                        lang: Some(Lang::new(lang.as_str())),
                    }),
                    (s, Some(datatype), _) => {
                        let iri_s = Self::iri2iri_s(&datatype);
                        Object::Literal(Literal::DatatypeLiteral {
                            lexical_form: s,
                            datatype: IriRef::Iri(iri_s),
                        })
                    }
                }
            }
            OxTerm::NamedNode(iri) => Object::Iri(Self::iri2iri_s(iri)),
            // #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn object_as_term(obj: &srdf::Object) -> Self::Term {
        match obj {
            Object::Iri(iri) => Self::iri_s2term(iri),
            Object::BlankNode(bn) => Self::bnode_id2term(bn),
            Object::Literal(lit) => {
                let literal: OxLiteral = match lit {
                    Literal::StringLiteral { lexical_form, lang } => match lang {
                        Some(lang) => OxLiteral::new_language_tagged_literal_unchecked(
                            lexical_form,
                            lang.to_string(),
                        ),
                        None => OxLiteral::new_simple_literal(lexical_form),
                    },
                    Literal::DatatypeLiteral {
                        lexical_form,
                        datatype,
                    } => OxLiteral::new_typed_literal(lexical_form, cnv_iri_ref(datatype)),
                    Literal::NumericLiteral(n) => match n {
                        NumericLiteral::Integer(n) => {
                            let n: i128 = *n as i128;
                            OxLiteral::from(n)
                        }
                        NumericLiteral::Decimal(d) => {
                            let decimal = cnv_decimal(d);
                            OxLiteral::from(decimal)
                        }
                        NumericLiteral::Double(d) => OxLiteral::from(*d),
                    },
                    Literal::BooleanLiteral(b) => OxLiteral::from(*b),
                };
                OxTerm::Literal(literal)
            }
        }
    }

    fn term_is_iri(object: &Self::Term) -> bool {
        matches!(object, OxTerm::NamedNode(_))
    }

    fn term_is_bnode(object: &Self::Term) -> bool {
        matches!(object, OxTerm::BlankNode(_))
    }

    fn term_is_literal(object: &Self::Term) -> bool {
        matches!(object, OxTerm::Literal(_))
    }

    fn term_as_subject(object: &Self::Term) -> Option<Self::Subject> {
        match object {
            OxTerm::NamedNode(n) => Some(OxSubject::NamedNode(n.clone())),
            OxTerm::BlankNode(b) => Some(OxSubject::BlankNode(b.clone())),
            _ => None,
        }
    }

    fn subject_as_term(subject: &Self::Subject) -> Self::Term {
        match subject {
            OxSubject::NamedNode(n) => OxTerm::NamedNode(n.clone()),
            OxSubject::BlankNode(b) => OxTerm::BlankNode(b.clone()),
            // #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => unimplemented!(),
        }
    }

    fn lexical_form(literal: &Self::Literal) -> &str {
        literal.value()
    }

    fn lang(literal: &Self::Literal) -> Option<String> {
        literal.language().map(|s| s.to_string())
    }

    fn datatype(literal: &Self::Literal) -> Self::IRI {
        literal.datatype().into_owned()
    }

    fn iri_s2iri(iri_s: &iri_s::IriS) -> Self::IRI {
        iri_s.as_named_node().clone()
    }

    fn term_s2term(term: &oxrdf::Term) -> Self::Term {
        term.clone()
    }

    fn bnode_id2bnode(id: &str) -> Self::BNode {
        OxBlankNode::new_unchecked(id)
    }

    fn iri_as_term(iri: Self::IRI) -> Self::Term {
        OxTerm::NamedNode(iri)
    }

    fn iri_as_subject(iri: Self::IRI) -> Self::Subject {
        OxSubject::NamedNode(iri)
    }

    fn bnode_as_term(bnode: Self::BNode) -> Self::Term {
        OxTerm::BlankNode(bnode)
    }

    fn bnode_as_subject(bnode: Self::BNode) -> Self::Subject {
        OxSubject::BlankNode(bnode)
    }

    fn iri2iri_s(iri: &Self::IRI) -> iri_s::IriS {
        IriS::from_named_node(iri)
    }

    fn qualify_iri(&self, node: &Self::IRI) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap();
        self.prefixmap_in_memory().qualify(&iri)
    }

    fn qualify_subject(&self, subj: &Self::Subject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_iri(n),
            // #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => unimplemented!(),
        }
    }

    fn qualify_term(&self, term: &Self::Term) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
            // #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn resolve_prefix_local(
        &self,
        prefix: &str,
        local: &str,
    ) -> Result<iri_s::IriS, prefixmap::PrefixMapError> {
        let iri = self
            .prefixmap_in_memory()
            .resolve_prefix_local(prefix, local)?;
        Ok(iri.clone())
    }
}

impl QuerySRDF2 for RdfData {
    fn query_select(&self, query_str: &str) -> Result<QuerySolutions<RdfData>, RdfDataError>
    where
        Self: Sized,
    {
        let mut sols: QuerySolutions<RdfData> = QuerySolutions::empty();
        let query = Query::parse(query_str, None)?;
        if let Some(store) = &self.store {
            let new_sol = store.query(query)?;
            let sol = cnv_query_results(new_sol)?;
            sols.extend(sol)
        }
        for endpoint in &self.endpoints {
            let new_sols = endpoint.query_select(query_str)?;
            let new_sols_converted: Vec<QuerySolution2<RdfData>> =
                new_sols.iter().map(cnv_sol).collect();
            sols.extend(new_sols_converted)
        }
        Ok(sols)
    }

    fn query_ask(&self, _query: &str) -> Result<bool, Self::Err> {
        todo!()
    }
}

fn cnv_sol(sol: &QuerySolution2<SRDFSparql>) -> QuerySolution2<RdfData> {
    sol.convert(|t| t.clone())
}

fn cnv_query_results(
    query_results: QueryResults,
) -> Result<Vec<QuerySolution2<RdfData>>, RdfDataError> {
    let mut results = Vec::new();
    if let QueryResults::Solutions(solutions) = query_results {
        for solution in solutions {
            let result = cnv_query_solution(solution?);
            results.push(result)
        }
    }
    Ok(results)
}

fn cnv_query_solution(qs: QuerySolution) -> QuerySolution2<RdfData> {
    let mut variables = Vec::new();
    let mut values = Vec::new();
    for v in qs.variables() {
        let varname = VarName2::new(v.as_str());
        variables.push(varname);
    }
    for t in qs.values() {
        let term = t.clone();
        values.push(term)
    }
    QuerySolution2::new(Rc::new(variables), values)
}

fn _cnv_rdf_format(rdf_format: RDFFormat) -> RdfFormat {
    match rdf_format {
        RDFFormat::NTriples => RdfFormat::NTriples,
        RDFFormat::Turtle => RdfFormat::Turtle,
        RDFFormat::RDFXML => RdfFormat::RdfXml,
        RDFFormat::TriG => RdfFormat::TriG,
        RDFFormat::N3 => RdfFormat::N3,
        RDFFormat::NQuads => RdfFormat::NQuads,
    }
}

fn _rdf_type() -> OxNamedNode {
    OxNamedNode::new_unchecked(RDF_TYPE_STR)
}

fn cnv_iri_ref(iri_ref: &IriRef) -> OxNamedNode {
    OxNamedNode::new_unchecked(iri_ref.to_string())
}

fn cnv_decimal(_d: &Decimal) -> oxsdatatypes::Decimal {
    todo!()
}

impl SRDF for RdfData {
    fn predicates_for_subject(
        &self,
        _subject: &Self::Subject,
    ) -> Result<std::collections::HashSet<Self::IRI>, Self::Err> {
        todo!()
    }

    fn objects_for_subject_predicate(
        &self,
        _subject: &Self::Subject,
        _pred: &Self::IRI,
    ) -> Result<std::collections::HashSet<Self::Term>, Self::Err> {
        todo!()
    }

    fn subjects_with_predicate_object(
        &self,
        _pred: &Self::IRI,
        _object: &Self::Term,
    ) -> Result<std::collections::HashSet<Self::Subject>, Self::Err> {
        todo!()
    }

    fn triples_with_predicate(
        &self,
        _pred: &Self::IRI,
    ) -> Result<Vec<srdf::Triple<Self>>, Self::Err> {
        todo!()
    }

    fn outgoing_arcs(
        &self,
        _subject: &Self::Subject,
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Term>>, Self::Err> {
        todo!()
    }

    fn incoming_arcs(
        &self,
        _object: &Self::Term,
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Subject>>, Self::Err> {
        todo!()
    }

    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: &[Self::IRI],
    ) -> Result<(HashMap<Self::IRI, HashSet<Self::Term>>, Vec<Self::IRI>), Self::Err> {
        let mut result = (HashMap::new(), Vec::new());
        if let Some(graph) = &self.graph {
            merge_outgoing_arcs(&mut result, graph.outgoing_arcs_from_list(subject, preds)?);
        }
        for endpoint in &self.endpoints {
            let next = endpoint.outgoing_arcs_from_list(subject, preds)?;
            merge_outgoing_arcs(&mut result, next)
        }
        Ok(result)
    }

    fn neighs(
        &self,
        node: &Self::Term,
    ) -> Result<ListOfIriAndTerms<Self::IRI, Self::Term>, Self::Err> {
        match Self::term_as_subject(node) {
            None => Ok(Vec::new()),
            Some(subject) => {
                let mut result = Vec::new();
                let preds = self.predicates_for_subject(&subject)?;
                for pred in preds {
                    let objs = self.objects_for_subject_predicate(&subject, &pred)?;
                    result.push((pred.clone(), objs));
                }
                Ok(result)
            }
        }
    }
}

impl SRDFBuilder for RdfData {
    fn empty() -> Self {
        todo!()
    }

    fn add_base(&mut self, _base: &Option<IriS>) -> Result<(), Self::Err> {
        todo!()
    }

    fn add_prefix(&mut self, _alias: &str, _iri: &IriS) -> Result<(), Self::Err> {
        todo!()
    }

    fn add_prefix_map(&mut self, _prefix_map: PrefixMap) -> Result<(), Self::Err> {
        todo!()
    }

    fn add_triple(
        &mut self,
        _subj: &Self::Subject,
        _pred: &Self::IRI,
        _obj: &Self::Term,
    ) -> Result<(), Self::Err> {
        todo!()
    }

    fn remove_triple(
        &mut self,
        _subj: &Self::Subject,
        _pred: &Self::IRI,
        _obj: &Self::Term,
    ) -> Result<(), Self::Err> {
        todo!()
    }

    fn add_type(&mut self, _node: &srdf::RDFNode, _type_: Self::Term) -> Result<(), Self::Err> {
        todo!()
    }

    fn serialize<W: std::io::Write>(
        &self,
        format: RDFFormat,
        writer: &mut W,
    ) -> Result<(), Self::Err> {
        if let Some(graph) = &self.graph {
            graph.serialize(format, writer)?;
            Ok::<(), Self::Err>(())
        } else {
            Ok(())
        }?;
        for endpoint in &self.endpoints {
            writeln!(writer, "Endpoint {}", endpoint.iri())?;
        }
        Ok(())
    }
}

fn merge_outgoing_arcs<I, T>(
    current: &mut (HashMap<I, HashSet<T>>, Vec<I>),
    next: (HashMap<I, HashSet<T>>, Vec<I>),
) where
    I: Eq + Hash,
    T: Eq + Hash,
{
    let (next_map, next_vs) = next;
    let (ref mut current_map, ref mut current_vs) = current;
    for v in next_vs {
        current_vs.push(v)
    }
    for (key, values) in next_map {
        current_map
            .entry(key)
            .and_modify(|current_values| {
                let _ = current_values.union(&values);
            })
            .or_insert(values);
    }
}
