use std::str::FromStr;

use iri_s::IriS;
use oxrdf::NamedNode;
use oxrdf::Subject;
use oxrdf::Term as OxTerm;
use oxrdf::Triple as OxTriple;
use prefixmap::PrefixMap;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header;
use reqwest::header::ACCEPT;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use sparesults::QueryResultsFormat;
use sparesults::QueryResultsParser;
use sparesults::QuerySolution as OxQuerySolution;
use sparesults::ReaderQueryResultsParserOutput;

use crate::model::rdf::PrefixMapRdf;
use crate::model::rdf::Rdf;
use crate::model::sparql::QuerySolution;
use crate::model::sparql::Sparql;
use crate::model::Triple;

use super::oxsparql_error::SparqlError;

/// Implements SRDF interface as a SPARQL endpoint
#[derive(Debug, Clone)]
pub struct SRDFSparql {
    endpoint_iri: IriS,
    prefixmap: PrefixMap,
    client: Client,
}

impl SRDFSparql {
    pub fn new(iri: IriS, prefixmap: PrefixMap) -> Result<Self, SparqlError> {
        let client = sparql_client()?;
        Ok(SRDFSparql {
            endpoint_iri: iri,
            prefixmap: prefixmap,
            client,
        })
    }

    pub fn iri(&self) -> &IriS {
        &self.endpoint_iri
    }

    pub fn wikidata() -> Result<Self, SparqlError> {
        SRDFSparql::new(
            IriS::from_str("https://query.wikidata.org/sparql").unwrap(),
            PrefixMap::wikidata(),
        )
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
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Error> {
        let pattern = format!(
            "{} {} {}",
            subject.map_or("?s".to_string(), |s| format!("{}", s)),
            predicate.map_or("?p".to_string(), |p| format!("{}", p)),
            object.map_or("?o".to_string(), |o| format!("{}", o))
        );
        let triples = self
            .select(&format!("SELECT ?s ?p ?o WHERE {{ {} . }}", pattern))?
            .into_iter()
            .map(move |solution| {
                println!("{:?}", solution);
                let subject: Subject = match subject {
                    Some(subj) => subj.clone(),
                    None => solution.get(0).unwrap().clone().try_into().unwrap(),
                };
                let pred: NamedNode = match predicate {
                    Some(pred) => pred.clone(),
                    None => solution.get(1).unwrap().clone().try_into().unwrap(),
                };
                let obj = match object {
                    Some(obj) => obj.clone(),
                    None => solution.get(2).unwrap().clone(),
                };
                Self::Triple::from_spo(subject, pred, obj)
            });

        Ok(triples)
    }
}

impl PrefixMapRdf for SRDFSparql {
    fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }
}

impl Sparql for SRDFSparql {
    type QuerySolution = OxQuerySolution;
    type Value = OxTerm;
    type SparqlError = SparqlError;

    fn make_sparql_query(
        &self,
        query: &str,
    ) -> Result<Vec<Self::QuerySolution>, Self::SparqlError> {
        let url = Url::parse_with_params(self.endpoint_iri.as_str(), &[("query", query)])?;
        let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);
        tracing::debug!("SPARQL query: {}", url);
        let body = self.client.get(url).send()?.text()?;
        let mut results = Vec::new();

        match json_parser.for_reader(body.as_bytes())? {
            ReaderQueryResultsParserOutput::Solutions(solutions) => {
                for solution in solutions {
                    let sol = solution?;
                    results.push(sol)
                }
                Ok(results)
            }
            ReaderQueryResultsParserOutput::Boolean(_) => Err(SparqlError::ParsingBody { body }),
        }
    }
}

impl QuerySolution<OxTerm> for OxQuerySolution {
    fn get(&self, index: usize) -> Option<&OxTerm> {
        self.get(index)
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

fn sparql_client() -> Result<Client, SparqlError> {
    let mut headers = header::HeaderMap::new();
    let json = "application/sparql-results+json";
    headers.insert(ACCEPT, header::HeaderValue::from_static(json));
    headers.insert(USER_AGENT, header::HeaderValue::from_static("rudof"));

    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use crate::model::Triple;

    use super::*;
    use oxrdf::{NamedNode, Subject};

    const ENTITY: &str = "http://www.wikidata.org/entity/";
    const PROPERTY: &str = "http://www.wikidata.org/prop/";

    fn q80() -> Subject {
        Subject::NamedNode(NamedNode::new_unchecked(format!("{ENTITY}Q80")))
    }

    fn p19() -> NamedNode {
        NamedNode::new_unchecked(format!("{PROPERTY}P19"))
    }

    #[test]
    fn check_sparql() {
        let wikidata = SRDFSparql::wikidata().unwrap();

        let data: Vec<_> = wikidata
            .triples_matching(Some(&q80()), None, None)
            .unwrap()
            .map(Triple::into_predicate)
            .collect();

        println!("{:?}", data);

        assert!(data.contains(&p19()));
    }
}
