use std::str::FromStr;

use iri_s::IriS;
use oxrdf::NamedNode as OxIri;
use oxrdf::Subject as OxSubject;
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

use crate::model::rdf::Object;
use crate::model::rdf::Predicate;
use crate::model::rdf::Rdf;
use crate::model::rdf::Subject;
use crate::model::sparql::QuerySolution;
use crate::model::sparql::Sparql;

use super::oxsparql_error::SparqlError;

/// Implements SRDF interface as a SPARQL endpoint
#[derive(Debug, Clone)]
pub struct SRDFSparql {
    endpoint_iri: IriS,
    prefixmap: PrefixMap,
    client: Client,
}

impl SRDFSparql {
    pub fn new(iri: &IriS, prefixmap: &PrefixMap) -> Result<Self, SparqlError> {
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

    pub fn wikidata() -> Result<Self, SparqlError> {
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
        subject: Option<&'a Subject<Self>>,
        predicate: Option<&'a Predicate<Self>>,
        object: Option<&'a Object<Self>>,
    ) -> Result<impl Iterator<Item = &Self::Triple>, Self::Error> {
        let query = format!(
            "SELECT * WHERE {{ {} {} {} . }}",
            subject.map_or("?subj".to_string(), |s| s.to_string()),
            predicate.map_or("?pred".to_string(), |p| p.to_string()),
            object.map_or("?obj".to_string(), |o| o.to_string()),
        );

        let triples: Vec<_> = self
            .select(&query)?
            .iter()
            .map(move |solution| {
                let subj: OxSubject = solution.get(0).unwrap().clone().try_into().unwrap();
                let pred: OxIri = solution.get(1).unwrap().clone().try_into().unwrap();
                let obj = solution.get(2).unwrap().clone();
                OxTriple::new(subj, pred, obj)
            })
            .collect();

        Ok(triples.iter())
    }

    fn prefixmap(&self) -> Option<PrefixMap> {
        Some(self.prefixmap.clone())
    }
}

impl Sparql for SRDFSparql {
    type QuerySolution = OxQuerySolution;
    type Object = OxTerm;
    type SparqlError = SparqlError;

    fn make_sparql_query(
        &self,
        query: &str,
    ) -> Result<Vec<Self::QuerySolution>, Self::SparqlError> {
        let url = Url::parse_with_params(self.endpoint_iri.as_str(), &[("query", query)])?;
        let body = self.client.get(url.clone()).send()?.text()?;
        let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);

        tracing::debug!("SPARQL query: {}", url);

        match json_parser.for_reader(body.as_bytes())? {
            ReaderQueryResultsParserOutput::Solutions(solutions) => {
                let mut results = Vec::new();
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
    fn get(&self, index: usize) -> Option<OxTerm> {
        self.get(index).cloned()
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
            .map(Triple::pred)
            .map(Clone::clone)
            .collect();

        assert!(data.contains(&p19()));
    }
}
