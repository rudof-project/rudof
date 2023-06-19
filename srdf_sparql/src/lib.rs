use bag::Bag;
use reqwest::{header::{ACCEPT, self, USER_AGENT}, Url};
use srdf::SRDF;
use oxrdf::*;
use thiserror::Error;
use async_trait::async_trait;
use sparesults::{QueryResultsFormat, QueryResultsParser, QueryResultsReader};
use oxrdf::{Literal, Variable};

#[derive(Error, Debug)]
pub enum SRDFSPARQLError {

    #[error("HTTP Request error: {e:?}")]
    HTTPRequestError { e: reqwest::Error },

    #[error("URL parser error: {e:?}")]
    URLParseError { e: url::ParseError },

    #[error("SPARQL Results parser: {e:?}")]
    SPAResults { e: sparesults::ParseError }
}

impl From<reqwest::Error> for SRDFSPARQLError {
    fn from(e: reqwest::Error) -> SRDFSPARQLError {
        SRDFSPARQLError::HTTPRequestError { e: e }
    }
}

impl From<url::ParseError> for SRDFSPARQLError {
    fn from(e: url::ParseError) -> SRDFSPARQLError {
        SRDFSPARQLError::URLParseError { e: e }
    }
}

impl From<sparesults::ParseError> for SRDFSPARQLError {
    fn from(e: sparesults::ParseError) -> SRDFSPARQLError {
        SRDFSPARQLError::SPAResults { e: e } 
    }
}


struct SRDFSPARQL {
    endpoint_iri: String
}

#[async_trait]
impl SRDF for SRDFSPARQL {
    type IRI = NamedNode;
    type BNode = BlankNode;
    type Literal = Literal;
    type Subject = Subject;
    type Term = Term;
    type Err = SRDFSPARQLError;

    async fn get_predicates_subject(&self, subject: &Subject) -> Result<Bag<NamedNode>, SRDFSPARQLError> {
        let mut results = Bag::new();
        let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);
        let mut headers = header::HeaderMap::new();
        headers.insert(ACCEPT, header::HeaderValue::from_static("application/sparql-results+json"));
        headers.insert(USER_AGENT, header::HeaderValue::from_static("Rust App"));
        let client = reqwest::Client::builder().default_headers(headers).build()?;
        let query = format!(r#"select ?pred where {{ 
            {} ?pred ?obj . }}
        "#, subject);
        let url = Url::parse_with_params(&self.endpoint_iri, &[("query", query)])?;
        println!("Url: {}", url);
        let body = client.get(url).send().await?.text().await?;
        
        if let QueryResultsReader::Solutions(solutions) = json_parser.read_results(body.as_bytes())? {
            for solution in solutions {
                let sol = solution?;
                match sol.get("pred") {
                    Some(v) => match v {
                        Term::NamedNode(n) => { results.add(n.clone()); },
                        _ => todo!()
                    }
                    _ => todo!()
                }
                
            }
            Ok(results)
        }  else {
            todo!()
        }

    } 

    async fn get_objects_for_subject_predicate(&self, subject: &Subject, pred: &NamedNode) -> Result<Bag<Term>, SRDFSPARQLError> {
        todo!();
    }

    async fn get_subjects_for_object_predicate(&self, object: &Term, pred: &NamedNode) -> Result<Bag<Subject>,SRDFSPARQLError> {
        todo!();
    }

    fn subject2iri(&self, subject:&Subject) -> Option<NamedNode> {
        match subject {
            Subject::NamedNode(n) => Some(n.clone()),
            _ => None
        }
    }
    fn subject2bnode(&self, subject:&Subject) -> Option<BlankNode> {
        match subject {
            Subject::BlankNode(b) => Some(b.clone()),
            _ => None
        }
    }
    fn subject_is_iri(&self, subject:&Subject) -> bool {
        match subject {
            Subject::NamedNode(_) => true,
            _ => false
        }
    }
    fn subject_is_bnode(&self, subject:&Subject) -> bool {
        match subject {
            Subject::BlankNode(_) => true,
            _ => false
        }
    }

    fn object2iri(&self, object:&Term) -> Option<NamedNode> {
        match object {
            Term::NamedNode(n) => Some(n.clone()),
            _ => None
        }
    }
    fn object2bnode(&self, object:&Term) -> Option<BlankNode> {
        match object {
            Term::BlankNode(b) => Some(b.clone()),
            _ => None
        }
    }
    fn object2literal(&self, object:&Term) -> Option<Literal> {
        match object {
            Term::Literal(l) => Some(l.clone()),
            _ => None
        }
    }
    fn object_is_iri(&self, object: &Term) -> bool {
        match object {
            Term::NamedNode(_) => true,
            _ => false
        }
    }
    fn object_is_bnode(&self, object:&Term) -> bool {
        match object {
            Term::BlankNode(_) => true,
            _ => false
        }
    }

    fn object_is_literal(&self, object:&Term) -> bool {
        match object {
            Term::Literal(_) => true,
            _ => false
        }
    }

    fn lexical_form(&self, literal: &Literal) -> String {
        literal.to_string()
    }
    fn lang(&self, literal: &Literal) -> Option<String> {
        literal.language().map(|s| s.to_string())
    }
    fn datatype(&self, literal: &Literal) -> NamedNode {
        literal.datatype().into_owned()
    }


}



#[cfg(test)]
mod tests {
    use oxrdf::{Subject, NamedNode};
    use srdf::SRDF;

    use crate::SRDFSPARQL;


    #[tokio::test]
    async fn check_sparql() {
        let wikidata = SRDFSPARQL {
            endpoint_iri: "https://query.wikidata.org/sparql".to_string()
        };
        let q80 : Subject = Subject::NamedNode(NamedNode::new_unchecked("http://www.wikidata.org/entity/Q80".to_string()));
        let p31 : NamedNode = NamedNode::new_unchecked("http://www.wikidata.org/entity/P31");
        let maybe_data = wikidata.get_predicates_subject(&q80).await;
        let data = maybe_data.unwrap();
        for (n,c) in data.iter() {
            println!("Node: {n}/{c}");
        }
        assert_eq!(22,22);
    }
}
