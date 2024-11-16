use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;

use iri_s::IriS;
use oxrdf::BlankNode as OxBlankNode;
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxIri;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use oxrdf::Triple as OxTriple;
use prefixmap::PrefixMap;
use regex::Regex;
use reqwest::blocking::Client;

use crate::model::rdf::Rdf;
use crate::model::sparql::Sparql;

use super::oxsparql_error::SparqlError;

type Result<A> = std::result::Result<A, SparqlError>;

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

    pub fn with_prefixmap(mut self, pm: PrefixMap) -> SRDFSparql {
        self.prefixmap = pm;
        self
    }

    pub fn iri(&self) -> &IriS {
        &self.endpoint_iri
    }

    pub fn wikidata() -> Result<SRDFSparql> {
        let endpoint = SRDFSparql::new(
            &IriS::new_unchecked("https://query.wikidata.org/sparql".to_string()),
            &PrefixMap::wikidata(),
        )?;
        Ok(endpoint)
    }
}

impl Rdf for SRDFSparql {
    type Triple = OxTriple;
    type Error = SparqlError;

    fn triples_matching<'a>(
        &self,
        subject: Option<&'a crate::model::rdf::Subject<Self>>,
        predicate: Option<&'a crate::model::rdf::Predicate<Self>>,
        object: Option<&'a crate::model::rdf::Object<Self>>,
    ) -> std::result::Result<impl Iterator<Item = &Self::Triple>, Self::Error> {
        todo!()
    }

    fn prefixmap(&self) -> Option<PrefixMap> {
        Some(self.prefixmap)
    }
}

impl Sparql for SRDFSparql {
    type QuerySolution;
    type Error;

    fn select(&self, query: &str) -> Result<QuerySolutions<Self>> {
        let solutions = make_sparql_query(query, &self.client, &self.endpoint_iri)?;
        let qs: Vec<QuerySolution<SRDFSparql>> = solutions.iter().map(cnv_query_solution).collect();
        Ok(QuerySolutions::new(qs))
    }

    fn ask(&self, query: &str) -> Result<bool> {
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

impl FromStr for SRDFSparql {
    type Err = SparqlError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re_iri = Regex::new(r"<(.*)>").unwrap();
        if let Some(iri_str) = re_iri.captures(s) {
            let iri_s = IriS::from_str(&iri_str[1])?;
            let client = sparql_client()?;
            Ok(SRDFSparql {
                endpoint_iri: iri_s,
                prefixmap: PrefixMap::default(),
                client,
            })
        } else {
            match s.to_lowercase().as_str() {
                "wikidata" => SRDFSparql::wikidata(),
                name => Err(SparqlError::UnknownEndpontName {
                    name: name.to_string(),
                }),
            }
        }
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
    QuerySolution::new(Rc::new(variables), values)
}

fn sparql_client() -> Result<Client> {
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

fn make_sparql_query(
    query: &str,
    client: &Client,
    endpoint_iri: &IriS,
) -> Result<Vec<OxQuerySolution>> {
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
        Err(SparqlError::ParsingBody { body })
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
