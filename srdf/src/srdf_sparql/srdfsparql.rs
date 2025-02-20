use crate::{lang::Lang, literal::Literal, Object, SRDFSparqlError};
use crate::{AsyncSRDF, Query, QuerySolution, QuerySolutions, Rdf, Sparql, VarName};
use async_trait::async_trait;
use colored::*;
use iri_s::IriS;
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, Subject as OxSubject,
    Term as OxTerm,
};
use prefixmap::{IriRef, PrefixMap};
use regex::Regex;
use sparesults::QuerySolution as OxQuerySolution;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

#[cfg(target_family = "wasm")]
#[derive(Debug, Clone)]
struct Client();

#[cfg(not(target_family = "wasm"))]
pub use reqwest::blocking::Client;

type Result<A> = std::result::Result<A, SRDFSparqlError>;

/// Implements SRDF interface as a SPARQL endpoint
#[derive(Debug, Clone)]
pub struct SRDFSparql {
    endpoint_iri: IriS,
    prefixmap: PrefixMap,
    client: Client,
}

impl SRDFSparql {
    pub fn new(iri: &IriS, prefixmap: &PrefixMap) -> Result<SRDFSparql> {
        let client = sparql_client()?;
        Ok(SRDFSparql {
            endpoint_iri: iri.clone(),
            prefixmap: prefixmap.clone(),
            client,
        })
    }

    pub fn iri(&self) -> &IriS {
        &self.endpoint_iri
    }

    pub fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }

    pub fn wikidata() -> Result<SRDFSparql> {
        let endpoint = SRDFSparql::new(
            &IriS::new_unchecked("https://query.wikidata.org/sparql"),
            &PrefixMap::wikidata(),
        )?;
        Ok(endpoint)
    }

    pub fn with_prefixmap(mut self, pm: PrefixMap) -> SRDFSparql {
        self.prefixmap = pm;
        self
    }

    fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        let str: String = format!("{}", bn);
        format!("{}", str.green())
    }

    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        let str: String = format!("{}", lit);
        format!("{}", str.red())
    }
}

impl FromStr for SRDFSparql {
    type Err = SRDFSparqlError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re_iri = Regex::new(r"<(.*)>").unwrap();
        if let Some(iri_str) = re_iri.captures(s) {
            let iri_s = IriS::from_str(&iri_str[1])?;
            let client = sparql_client()?;
            Ok(SRDFSparql {
                endpoint_iri: iri_s,
                prefixmap: PrefixMap::new(),
                client,
            })
        } else {
            match s.to_lowercase().as_str() {
                "wikidata" => SRDFSparql::wikidata(),
                name => Err(SRDFSparqlError::UnknownEndpontName {
                    name: name.to_string(),
                }),
            }
        }
    }
}

impl Rdf for SRDFSparql {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = SRDFSparqlError;

    fn lang(literal: &OxLiteral) -> Option<String> {
        literal.language().map(|s| s.to_string())
    }
    fn datatype(literal: &OxLiteral) -> OxNamedNode {
        literal.datatype().into_owned()
    }

    fn term_as_object(term: &Self::Term) -> Object {
        match term {
            Self::Term::BlankNode(bn) => Object::BlankNode(bn.to_string()),
            Self::Term::Literal(lit) => match lit.to_owned().destruct() {
                (s, None, None) => Object::Literal(Literal::StringLiteral {
                    lexical_form: s,
                    lang: None,
                }),
                (s, None, Some(lang)) => Object::Literal(Literal::StringLiteral {
                    lexical_form: s,
                    lang: Some(Lang::new(lang.as_str())),
                }),
                (s, Some(datatype), _) => {
                    let iri_s = IriS::from_named_node(&datatype);
                    Object::Literal(Literal::DatatypeLiteral {
                        lexical_form: s,
                        datatype: IriRef::Iri(iri_s),
                    })
                }
            },
            Self::Term::NamedNode(iri) => Object::Iri(IriS::from_named_node(iri)),

            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn resolve_prefix_local(
        &self,
        prefix: &str,
        local: &str,
    ) -> std::result::Result<IriS, prefixmap::PrefixMapError> {
        self.prefixmap.resolve_prefix_local(prefix, local)
    }

    fn qualify_iri(&self, node: &OxNamedNode) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap();
        self.prefixmap.qualify(&iri)
    }

    fn qualify_subject(&self, subj: &OxSubject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_iri(n),
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => unimplemented!(),
        }
    }

    fn qualify_term(&self, term: &OxTerm) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn prefixmap(&self) -> Option<PrefixMap> {
        Some(self.prefixmap.clone())
    }

    fn object_as_term(_obj: &Object) -> Self::Term {
        todo!()
    }
}

#[async_trait]
impl AsyncSRDF for SRDFSparql {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = SRDFSparqlError;

    async fn get_predicates_subject(&self, subject: &OxSubject) -> Result<HashSet<OxNamedNode>> {
        let query = format!(r#"select ?pred where {{ {} ?pred ?obj . }}"#, subject);
        let solutions = make_sparql_query(query.as_str(), &self.client, &self.endpoint_iri)?;
        let mut results = HashSet::new();
        for solution in solutions {
            let n = get_iri_solution(solution, "pred")?;
            results.insert(n.clone());
        }
        Ok(results)
    }

    async fn get_objects_for_subject_predicate(
        &self,
        _subject: &OxSubject,
        _pred: &OxNamedNode,
    ) -> Result<HashSet<OxTerm>> {
        todo!();
    }

    async fn get_subjects_for_object_predicate(
        &self,
        _object: &OxTerm,
        _pred: &OxNamedNode,
    ) -> Result<HashSet<OxSubject>> {
        todo!();
    }
}

impl Query for SRDFSparql {
    fn predicates_for_subject(&self, subject: &OxSubject) -> Result<HashSet<OxNamedNode>> {
        let query = format!(r#"select ?pred where {{ {} ?pred ?obj . }}"#, subject);
        tracing::debug!(
            "SPARQL query (get predicates for subject {subject}): {}",
            query
        );
        let solutions = make_sparql_query(query.as_str(), &self.client, &self.endpoint_iri)?;
        let mut results = HashSet::new();
        for solution in solutions {
            let n = get_iri_solution(solution, "pred")?;
            results.insert(n.clone());
        }
        Ok(results)
    }

    fn objects_for_subject_predicate(
        &self,
        subject: &OxSubject,
        pred: &OxNamedNode,
    ) -> Result<HashSet<OxTerm>> {
        let query = format!(r#"select ?obj where {{ {} {} ?obj . }}"#, subject, pred);
        let solutions = make_sparql_query(query.as_str(), &self.client, &self.endpoint_iri)?;
        let mut results = HashSet::new();
        for solution in solutions {
            let n = get_object_solution(solution, "obj")?;
            results.insert(n.clone());
        }
        Ok(results)
    }

    fn subjects_with_predicate_object(
        &self,
        pred: &OxNamedNode,
        object: &OxTerm,
    ) -> Result<HashSet<OxSubject>> {
        let query = format!(r#"select ?subj where {{ ?subj {} {} . }}"#, pred, object);
        let solutions = make_sparql_query(query.as_str(), &self.client, &self.endpoint_iri)?;
        let mut results = HashSet::new();
        for solution in solutions {
            let n = get_subject_solution(solution, "subj")?;
            results.insert(n.clone());
        }
        Ok(results)
    }

    fn outgoing_arcs(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Term>>> {
        outgoing_neighs(
            subject.to_string().as_str(),
            &self.client,
            &self.endpoint_iri,
        )
    }

    fn incoming_arcs(
        &self,
        object: &Self::Term,
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Subject>>> {
        incoming_neighs(
            object.to_string().as_str(),
            &self.client,
            &self.endpoint_iri,
        )
    }

    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: &[Self::IRI],
    ) -> std::prelude::v1::Result<
        (HashMap<Self::IRI, HashSet<Self::Term>>, Vec<Self::IRI>),
        Self::Err,
    > {
        outgoing_neighs_from_list(subject, preds, &self.client, &self.endpoint_iri)
    }

    fn triples_with_predicate(
        &self,
        _pred: &Self::IRI,
    ) -> std::prelude::v1::Result<Vec<crate::Triple<Self>>, Self::Err> {
        todo!()
    }
}

/*
impl QuerySRDF for SRDFSparql {
    fn query_select(&self, query: &str) -> Result<QuerySolutions<SRDFSparql>> {
        let solutions = make_sparql_query(query, &self.client, &self.endpoint_iri)?;
        let mut variables = Vec::new();
        let mut values = Vec::new();
        for solution in solutions {
            if variables.is_empty() {
                variables.extend(solution.variables().iter().map(|v| v.to_string().into()))
            }
            values.push(Ok(solution.values().to_vec()))
        }
        Ok(QuerySolutions::new(Rc::new(variables), values.into_iter()))
    }

    fn query_ask(&self, query: &str) -> Result<bool> {
        make_sparql_query(query, &self.client, &self.endpoint_iri)?
            .first()
            .and_then(|query_solution| query_solution.get(0))
            .and_then(|term| match term {
                OxTerm::Literal(literal) => Some(literal.value()),
                _ => None,
            })
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| todo!())
    }
} */

impl Sparql for SRDFSparql {
    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>> {
        let solutions = make_sparql_query(query, &self.client, &self.endpoint_iri)?;
        let qs: Vec<QuerySolution<SRDFSparql>> = solutions.iter().map(cnv_query_solution).collect();
        Ok(QuerySolutions::new(qs))
    }

    fn query_ask(&self, query: &str) -> Result<bool> {
        make_sparql_query(query, &self.client, &self.endpoint_iri)?
            .first()
            .and_then(|query_solution| query_solution.get(0))
            .and_then(|term| match term {
                OxTerm::Literal(literal) => Some(literal.value()),
                _ => None,
            })
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| todo!())
    }
}

fn cnv_query_solution(qs: &OxQuerySolution) -> QuerySolution<SRDFSparql> {
    let mut variables = Vec::new();
    let mut values = Vec::new();
    for v in qs.variables() {
        let varname = VarName::new(v.as_str());
        variables.push(varname);
    }
    for t in qs.values() {
        let term = t.clone();
        values.push(term)
    }
    QuerySolution::new(variables, values)
}

#[cfg(target_family = "wasm")]
fn sparql_client() -> Result<Client> {
    Ok(Client())
}

#[cfg(not(target_family = "wasm"))]
fn sparql_client() -> Result<Client> {
    use reqwest::header::{self, ACCEPT, USER_AGENT};

    let mut headers = header::HeaderMap::new();
    headers.insert(
        ACCEPT,
        header::HeaderValue::from_static("application/sparql-results+json"),
    );
    headers.insert(USER_AGENT, header::HeaderValue::from_static("rudof"));
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

#[cfg(target_family = "wasm")]
fn make_sparql_query(
    _query: &str,
    _client: &Client,
    _endpoint_iri: &IriS,
) -> Result<Vec<OxQuerySolution>> {
    Err(SRDFSparqlError::UnknownEndpontName {
        name: String::from("WASM"),
    })
}

#[cfg(not(target_family = "wasm"))]
fn make_sparql_query(
    query: &str,
    client: &Client,
    endpoint_iri: &IriS,
) -> Result<Vec<OxQuerySolution>> {
    use sparesults::{QueryResultsFormat, QueryResultsParser, ReaderQueryResultsParserOutput};
    use url::Url;

    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;
    tracing::debug!("SPARQL query: {}", url);
    let body = client.get(url).send()?.text()?;
    let mut results = Vec::new();
    let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);
    if let ReaderQueryResultsParserOutput::Solutions(solutions) =
        json_parser.for_reader(body.as_bytes())?
    {
        for solution in solutions {
            let sol = solution?;
            results.push(sol)
        }
        Ok(results)
    } else {
        Err(SRDFSparqlError::ParsingBody { body })
    }
}

#[derive(Debug)]
pub struct SparqlVars {
    values: Vec<String>,
}

impl SparqlVars {
    pub(crate) fn new(vs: Vec<String>) -> SparqlVars {
        SparqlVars { values: vs }
    }
}

impl Display for SparqlVars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.values.join(", ").as_str())
    }
}

#[cfg(not(target_family = "wasm"))]
fn outgoing_neighs(
    subject: &str,
    client: &Client,
    endpoint_iri: &IriS,
) -> Result<HashMap<OxNamedNode, HashSet<OxTerm>>> {
    use std::collections::hash_map::Entry;

    use sparesults::{QueryResultsFormat, QueryResultsParser, ReaderQueryResultsParserOutput};
    use url::Url;

    let pred = "pred";
    let obj = "obj";
    let query = format!("select ?{pred} ?{obj} where {{ {subject} ?{pred} ?{obj} }}");
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;
    let body = client.get(url).send()?.text()?;
    let mut results: HashMap<OxNamedNode, HashSet<OxTerm>> = HashMap::new();
    let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);
    if let ReaderQueryResultsParserOutput::Solutions(solutions) =
        json_parser.for_reader(body.as_bytes())?
    {
        for solution in solutions {
            let sol = solution?;
            match (sol.get(pred), sol.get(obj)) {
                (Some(p), Some(v)) => match p {
                    OxTerm::NamedNode(iri) => match results.entry(iri.clone()) {
                        Entry::Occupied(mut vs) => {
                            vs.get_mut().insert(v.clone());
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(HashSet::from([v.clone()]));
                        }
                    },
                    _ => {
                        return Err(SRDFSparqlError::SPARQLSolutionErrorNoIRI { value: p.clone() })
                    }
                },
                (None, None) => {
                    return Err(SRDFSparqlError::NotFoundVarsInSolution {
                        vars: SparqlVars::new(vec![pred.to_string(), obj.to_string()]),
                        solution: format!("{sol:?}"),
                    })
                }
                (None, Some(_)) => {
                    return Err(SRDFSparqlError::NotFoundVarsInSolution {
                        vars: SparqlVars::new(vec![pred.to_string()]),
                        solution: format!("{sol:?}"),
                    })
                }
                (Some(_), None) => {
                    return Err(SRDFSparqlError::NotFoundVarsInSolution {
                        vars: SparqlVars::new(vec![obj.to_string()]),
                        solution: format!("{sol:?}"),
                    })
                }
            }
        }
        Ok(results)
    } else {
        Err(SRDFSparqlError::ParsingBody { body })
    }
}

#[cfg(target_family = "wasm")]
fn outgoing_neighs(
    _subject: &str,
    _client: &Client,
    _endpoint_iri: &IriS,
) -> Result<HashMap<OxNamedNode, HashSet<OxTerm>>> {
    Err(SRDFSparqlError::UnknownEndpontName {
        name: String::from("WASM"),
    })
}

type OutputNodes = HashMap<OxNamedNode, HashSet<OxTerm>>;

fn outgoing_neighs_from_list(
    subject: &OxSubject,
    preds: &[OxNamedNode],
    client: &Client,
    endpoint_iri: &IriS,
) -> Result<(OutputNodes, Vec<OxNamedNode>)> {
    // This is not an efficient way to obtain the neighbours related with a set of predicates
    // At this moment, it obtains all neighbours and them removes the ones that are not in the list
    let mut remainder = Vec::new();
    let mut all_results = outgoing_neighs(subject.to_string().as_str(), client, endpoint_iri)?;
    let mut remove_keys = Vec::new();
    for key in all_results.keys() {
        if !preds.contains(key) {
            remainder.push(key.clone());
            remove_keys.push(key.clone());
        }
    }
    for key in remove_keys {
        all_results.remove(&key);
    }
    Ok((all_results, remainder))
}

#[cfg(target_family = "wasm")]
fn incoming_neighs(
    _object: &str,
    _client: &Client,
    _endpoint_iri: &IriS,
) -> Result<HashMap<OxNamedNode, HashSet<OxSubject>>> {
    Err(SRDFSparqlError::UnknownEndpontName {
        name: String::from("WASM"),
    })
}

#[cfg(not(target_family = "wasm"))]
fn incoming_neighs(
    object: &str,
    client: &Client,
    endpoint_iri: &IriS,
) -> Result<HashMap<OxNamedNode, HashSet<OxSubject>>> {
    use sparesults::{QueryResultsFormat, QueryResultsParser, ReaderQueryResultsParserOutput};
    use std::collections::hash_map::Entry;
    use url::Url;

    let pred = "pred";
    let subj = "subj";
    let query = format!("select ?{pred} ?{subj} where {{ ?{subj} ?{pred} {object} }}");
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;
    let body = client.get(url).send()?.text()?;
    let mut results: HashMap<OxNamedNode, HashSet<OxSubject>> = HashMap::new();
    let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);
    if let ReaderQueryResultsParserOutput::Solutions(solutions) =
        json_parser.for_reader(body.as_bytes())?
    {
        for solution in solutions {
            let sol = solution?;
            match (sol.get(pred), sol.get(subj)) {
                (Some(p), Some(v)) => match p {
                    OxTerm::NamedNode(iri) => match term_as_subject(v) {
                        Some(subj) => match results.entry(iri.clone()) {
                            Entry::Occupied(mut vs) => {
                                vs.get_mut().insert(subj.clone());
                            }
                            Entry::Vacant(vacant) => {
                                vacant.insert(HashSet::from([subj.clone()]));
                            }
                        },
                        None => return Err(SRDFSparqlError::NoSubject { term: v.clone() }),
                    },
                    _ => {
                        return Err(SRDFSparqlError::SPARQLSolutionErrorNoIRI { value: p.clone() })
                    }
                },
                (None, None) => {
                    return Err(SRDFSparqlError::NotFoundVarsInSolution {
                        vars: SparqlVars::new(vec![pred.to_string(), subj.to_string()]),
                        solution: format!("{sol:?}"),
                    })
                }
                (None, Some(_)) => {
                    return Err(SRDFSparqlError::NotFoundVarsInSolution {
                        vars: SparqlVars::new(vec![pred.to_string()]),
                        solution: format!("{sol:?}"),
                    })
                }
                (Some(_), None) => {
                    return Err(SRDFSparqlError::NotFoundVarsInSolution {
                        vars: SparqlVars::new(vec![subj.to_string()]),
                        solution: format!("{sol:?}"),
                    })
                }
            }
        }
        Ok(results)
    } else {
        Err(SRDFSparqlError::ParsingBody { body })
    }
}

fn get_iri_solution(solution: OxQuerySolution, name: &str) -> Result<OxNamedNode> {
    match solution.get(name) {
        Some(v) => match v {
            OxTerm::NamedNode(n) => Ok(n.clone()),
            _ => Err(SRDFSparqlError::SPARQLSolutionErrorNoIRI { value: v.clone() }),
        },
        None => Err(SRDFSparqlError::NotFoundInSolution {
            value: name.to_string(),
            solution: format!("{solution:?}"),
        }),
    }
}

fn get_object_solution(solution: OxQuerySolution, name: &str) -> Result<OxTerm> {
    match solution.get(name) {
        Some(v) => Ok(v.clone()),
        None => Err(SRDFSparqlError::NotFoundInSolution {
            value: name.to_string(),
            solution: format!("{solution:?}"),
        }),
    }
}

fn get_subject_solution(solution: OxQuerySolution, name: &str) -> Result<OxSubject> {
    match solution.get(name) {
        Some(v) => match term_as_subject(v) {
            Some(s) => Ok(s),
            None => Err(SRDFSparqlError::SPARQLSolutionErrorNoSubject { value: v.clone() }),
        },
        None => Err(SRDFSparqlError::NotFoundInSolution {
            value: name.to_string(),
            solution: format!("{solution:?}"),
        }),
    }
}

fn term_as_subject(object: &OxTerm) -> Option<OxSubject> {
    match object {
        OxTerm::NamedNode(n) => Some(OxSubject::NamedNode(n.clone())),
        OxTerm::BlankNode(b) => Some(OxSubject::BlankNode(b.clone())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxrdf::{NamedNode, Subject};

    #[test]
    fn check_sparql() {
        let wikidata = SRDFSparql::wikidata().unwrap();
        let q80: Subject = Subject::NamedNode(NamedNode::new_unchecked(
            "http://www.wikidata.org/entity/Q80".to_string(),
        ));
        let maybe_data = wikidata.predicates_for_subject(&q80);
        let data = maybe_data.unwrap();
        let p19: NamedNode =
            NamedNode::new_unchecked("http://www.wikidata.org/prop/P19".to_string());

        assert!(data.contains(&p19));
    }
}
