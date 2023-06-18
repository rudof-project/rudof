use reqwest::{header::{ACCEPT, self, USER_AGENT}, Url};
use srdf::SRDF;
use oxrdf::*;
use thiserror::Error;
use async_trait::async_trait;

#[derive(Error, Debug)]
pub enum SRDFSPARQLError {

    #[error("HTTP Request error: {e:?}")]
    HTTPRequestError { e: reqwest::Error },

    #[error("URL parser error: {e:?}")]
    URLParseError { e: url::ParseError }

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

    async fn get_predicates_subject(&self, subject: &Subject) -> Result<Vec<NamedNode>, SRDFSPARQLError> {
        let results = vec![];
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
        println!("response body: {:?}", body);
        Ok(results)
    } 

    async fn get_objects_for_subject_predicate(&self, subject: &Subject, pred: &NamedNode) -> Result<Vec<Term>, SRDFSPARQLError> {
        todo!();
    }

    async fn get_subjects_for_object_predicate(&self, object: &Term, pred: &NamedNode) -> Result<Vec<Subject>,SRDFSPARQLError> {
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
        let data = 
           wikidata.get_predicates_subject(&q80).await;
        assert_eq!(22,22);
    }
}
