use crate::matcher::{Any, Matcher};
use crate::SRDFSparqlError;
use crate::{AsyncSRDF, Query, QuerySolution, QuerySolutions, Rdf, Sparql, VarName};
use async_trait::async_trait;
use colored::*;
use iri_s::IriS;
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, Subject as OxSubject,
    Term as OxTerm, Triple as OxTriple,
};
use prefixmap::PrefixMap;
use regex::Regex;
use sparesults::QuerySolution as OxQuerySolution;
use std::{collections::HashSet, fmt::Display, str::FromStr};

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
    type Triple = OxTriple;
    type Err = SRDFSparqlError;

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
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>> {
        self.triples_matching(Any, Any, Any)
    }

    fn triples_matching<S, P, O>(
        &self,
        subject: S,
        predicate: P,
        object: O,
    ) -> Result<impl Iterator<Item = Self::Triple>>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        let query = format!(
            "SELECT ?s ?p ?o WHERE {{ {} {} {} }}",
            match subject.value() {
                Some(s) => s.to_string(),
                None => "?s".to_string(),
            },
            match predicate.value() {
                Some(p) => p.to_string(),
                None => "?p".to_string(),
            },
            match object.value() {
                Some(o) => o.to_string(),
                None => "?o".to_string(),
            },
        );

        let triples = self
            .query_select(&query)? // TODO: check this unwrap
            .into_iter()
            .map(move |solution| {
                let subject: Self::Subject = match subject.value() {
                    Some(s) => s,
                    None => solution
                        .find_solution(0)
                        .and_then(|s| s.clone().try_into().ok())
                        .unwrap(), // we know that this won't panic
                };

                let predicate: Self::IRI = match predicate.value() {
                    Some(p) => p,
                    None => solution
                        .find_solution(1)
                        .and_then(|pred| pred.clone().try_into().ok())
                        .unwrap(), // we know that this won't panic
                };

                let object = match object.value() {
                    Some(o) => o,
                    None => solution.find_solution(2).cloned().unwrap(), // we know that this won't panic
                };

                OxTriple::new(subject, predicate, object)
            });

        Ok(triples)
    }
}

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

#[cfg(test)]
mod tests {
    use crate::Triple;

    use super::*;
    use oxrdf::{NamedNode, Subject};

    #[test]
    fn check_sparql() {
        let wikidata = SRDFSparql::wikidata().unwrap();

        let q80: Subject = NamedNode::new_unchecked("http://www.wikidata.org/entity/Q80").into();
        let p19: NamedNode = NamedNode::new_unchecked("http://www.wikidata.org/prop/P19");

        let data: Vec<_> = wikidata
            .triples_with_subject(q80)
            .unwrap()
            .map(Triple::into_predicate)
            .collect();

        assert!(data.contains(&p19));
    }
}
