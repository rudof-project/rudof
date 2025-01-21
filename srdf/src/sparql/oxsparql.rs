use std::io::Cursor;
use std::io::Read;
use std::marker::PhantomData;
use std::str::FromStr;

use iri_s::IriS;
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

use crate::model;
use crate::model::rdf::Object;
use crate::model::rdf::Predicate;
use crate::model::rdf::PrefixMapRdf;
use crate::model::rdf::Rdf;
use crate::model::rdf::Subject;
use crate::model::sparql::QuerySolution;
use crate::model::sparql::QuerySolutionParser;
use crate::model::sparql::Sparql;
use crate::model::Triple;

use super::oxsparql_error::SparqlError;

pub type OxSparql = GenericSparql<OxTriple>;

/// Implements SRDF interface as a SPARQL endpoint
#[derive(Debug, Clone)]
pub struct GenericSparql<T: Triple> {
    endpoint_iri: IriS,
    prefixmap: PrefixMap,
    client: Client,
    phantom: PhantomData<T>,
}

impl<T: Triple> GenericSparql<T> {
    pub fn new(iri: IriS, prefixmap: PrefixMap) -> Result<Self, SparqlError> {
        let sparql = Self {
            endpoint_iri: iri,
            prefixmap: prefixmap,
            client: sparql_client()?,
            phantom: PhantomData,
        };
        Ok(sparql)
    }

    pub fn iri(&self) -> &IriS {
        &self.endpoint_iri
    }

    pub fn wikidata() -> Result<Self, SparqlError> {
        Self::new(
            IriS::from_str("https://query.wikidata.org/sparql").unwrap(),
            PrefixMap::wikidata(),
        )
    }
}

impl<T: Triple> FromStr for GenericSparql<T> {
    type Err = SparqlError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re_iri = Regex::new(r"<(.*)>").unwrap();
        if let Some(iri_str) = re_iri.captures(s) {
            let iri_s = IriS::from_str(&iri_str[1])?;
            Self::new(iri_s, PrefixMap::default())
        } else {
            match s.to_lowercase().as_str() {
                "wikidata" => Self::wikidata(),
                _ => Err(SparqlError::UnknownEndpointName {
                    name: s.to_string(),
                }),
            }
        }
    }
}

impl Rdf for OxSparql {
    type Triple = OxTriple;
    type Error = SparqlError;

    fn triples_matching<'a>(
        &self,
        subject: Option<&'a Subject<Self>>,
        predicate: Option<&'a Predicate<Self>>,
        object: Option<&'a Object<Self>>,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Error> {
        let basic_graph_pattern = format!(
            "SELECT ?s ?p ?o WHERE {{ {} {} {} . }}",
            subject.map_or("?s".to_string(), |s| format!("{}", s)),
            predicate.map_or("?p".to_string(), |p| format!("{}", p)),
            object.map_or("?o".to_string(), |o| format!("{}", o))
        );
        let triples = self
            .select(basic_graph_pattern)?
            .into_iter()
            .map(move |solution| {
                let subject = match subject {
                    Some(subj) => subj.clone(),
                    None => match solution.get(0).unwrap().clone().try_into() {
                        Ok(subj) => subj,
                        Err(_) => unreachable!(),
                    },
                };
                let pred = match predicate {
                    Some(pred) => pred.clone(),
                    None => match solution.get(1).unwrap().clone().try_into() {
                        Ok(pred) => pred,
                        Err(_) => unreachable!(),
                    },
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

impl PrefixMapRdf for OxSparql {
    fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }
}

impl<T: Triple> Sparql for GenericSparql<T> {
    type QuerySolution = OxQuerySolution;
    type SparqlError = SparqlError;

    fn make_sparql_query(
        &self,
        query: String,
    ) -> Result<Vec<Self::QuerySolution>, Self::SparqlError> {
        let url = Url::parse_with_params(self.endpoint_iri.as_str(), &[("query", query)])?;
        let body = self.client.get(url).send()?.text()?;
        let reader = Cursor::new(body);
        let solutions = QueryResultsParser::parse(model::sparql::QueryResultsFormat::Json, reader)?;
        Ok(solutions)
    }
}

impl QuerySolutionParser for QueryResultsParser {
    type QuerySolution = OxQuerySolution;
    type Error = SparqlError;

    fn parse<R: Read>(
        format: model::sparql::QueryResultsFormat,
        reader: R,
    ) -> Result<Vec<Self::QuerySolution>, Self::Error> {
        match format {
            model::sparql::QueryResultsFormat::Json => {
                let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);
                if let ReaderQueryResultsParserOutput::Solutions(solutions) =
                    json_parser.for_reader(reader)?
                {
                    let mut results = Vec::new();
                    for solution in solutions {
                        let sol = solution?;
                        results.push(sol)
                    }
                    Ok(results)
                } else {
                    Err(SparqlError::ParsingBody)
                }
            }
        }
    }
}

impl QuerySolution for OxQuerySolution {
    type Value = OxTerm;

    fn get(&self, index: usize) -> Option<&Self::Value> {
        self.get(index)
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
    use oxrdf::NamedNode;
    use oxrdf::Subject;

    use crate::model::Triple;

    use super::*;

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
        let wikidata = OxSparql::wikidata().unwrap();

        let data: Vec<_> = wikidata
            .triples_matching(Some(&q80()), None, None)
            .unwrap()
            .map(Triple::into_predicate)
            .collect();

        assert!(data.contains(&p19()));
    }
}
