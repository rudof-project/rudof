use std::collections::HashSet;

use async_trait::async_trait;
use iri_s::IriS;
use oxrdf::Literal;
use oxrdf::*;
use prefixmap::IriRef;
use reqwest::{
    header::{self, ACCEPT, USER_AGENT},
    Url,
};
use sparesults::{QueryResultsFormat, QueryResultsParser, QueryResultsReader};
use srdf::{AsyncSRDF, SRDFComparisons, SRDF};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SRDFSPARQLError {
    #[error("HTTP Request error: {e:?}")]
    HTTPRequestError { e: reqwest::Error },

    #[error("URL parser error: {e:?}")]
    URLParseError { e: url::ParseError },

    #[error("SPARQL Results parser: {e:?}")]
    SPAResults { e: sparesults::ParseError },

    #[error(transparent)]
    IriParseError {
        #[from]
        err: IriParseError,
    },
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
    endpoint_iri: String,
}

impl SRDFComparisons for SRDFSPARQL {
    type IRI = NamedNode;
    type BNode = BlankNode;
    type Literal = Literal;
    type Subject = Subject;
    type Term = Term;
    type Err = SRDFSPARQLError;

    fn subject2iri(&self, subject: &Subject) -> Option<NamedNode> {
        match subject {
            Subject::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }
    fn subject2bnode(&self, subject: &Subject) -> Option<BlankNode> {
        match subject {
            Subject::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }
    fn subject_is_iri(&self, subject: &Subject) -> bool {
        match subject {
            Subject::NamedNode(_) => true,
            _ => false,
        }
    }
    fn subject_is_bnode(&self, subject: &Subject) -> bool {
        match subject {
            Subject::BlankNode(_) => true,
            _ => false,
        }
    }

    fn object2iri(&self, object: &Term) -> Option<NamedNode> {
        match object {
            Term::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }
    fn object2bnode(&self, object: &Term) -> Option<BlankNode> {
        match object {
            Term::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }
    fn object2literal(&self, object: &Term) -> Option<Literal> {
        match object {
            Term::Literal(l) => Some(l.clone()),
            _ => None,
        }
    }
    fn object_is_iri(&self, object: &Term) -> bool {
        match object {
            Term::NamedNode(_) => true,
            _ => false,
        }
    }
    fn object_is_bnode(&self, object: &Term) -> bool {
        match object {
            Term::BlankNode(_) => true,
            _ => false,
        }
    }

    fn object_is_literal(&self, object: &Term) -> bool {
        match object {
            Term::Literal(_) => true,
            _ => false,
        }
    }

    fn term_as_subject(&self, object: &Self::Term) -> Option<Subject> {
        match object {
            Term::NamedNode(n) => Some(Subject::NamedNode(n.clone())),
            Term::BlankNode(b) => Some(Subject::BlankNode(b.clone())),
            _ => None,
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

    /*fn iri_from_str(str: &str) -> Result<NamedNode,SRDFSPARQLError>  {
        NamedNode::new(str)
        .map_err(|err| {SRDFSPARQLError::IriParseError { err }})
    }*/

    fn iri_as_term(iri: NamedNode) -> Term {
        Term::NamedNode(iri)
    }

    fn iri_s2iri(iri_s: &IriS) -> &Self::IRI {
        iri_s.as_named_node()
    }

    fn term2object(term: Self::Term) -> srdf::Object {
        match term {
            Self::Term::BlankNode(bn) => srdf::Object::BlankNode(bn.to_string()),
            Self::Term::Literal(lit) => match lit.destruct() {
                (s, None, None) => srdf::Object::Literal(srdf::literal::Literal::StringLiteral {
                    lexical_form: s,
                    lang: None,
                }),
                (s, None, Some(lang)) => {
                    srdf::Object::Literal(srdf::literal::Literal::StringLiteral {
                        lexical_form: s,
                        lang: Some(srdf::lang::Lang::new(lang.as_str())),
                    })
                }
                (s, Some(datatype), _) => {
                    let iri_s = Self::iri2iri_s(datatype);
                    srdf::Object::Literal(srdf::literal::Literal::DatatypeLiteral {
                        lexical_form: s,
                        datatype: IriRef::Iri(iri_s),
                    })
                }
            },
            Self::Term::NamedNode(iri) => srdf::Object::Iri {
                iri: Self::iri2iri_s(iri),
            },
        }
    }

    fn iri2iri_s(iri: Self::IRI) -> IriS {
        IriS::from_named_node(iri)
    }
}

#[async_trait]
impl AsyncSRDF for SRDFSPARQL {
    type IRI = NamedNode;
    type BNode = BlankNode;
    type Literal = Literal;
    type Subject = Subject;
    type Term = Term;
    type Err = SRDFSPARQLError;

    async fn get_predicates_subject(
        &self,
        subject: &Subject,
    ) -> Result<HashSet<NamedNode>, SRDFSPARQLError> {
        let mut results = HashSet::new();
        let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);
        let mut headers = header::HeaderMap::new();
        headers.insert(
            ACCEPT,
            header::HeaderValue::from_static("application/sparql-results+json"),
        );
        headers.insert(USER_AGENT, header::HeaderValue::from_static("Rust App"));
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;
        let query = format!(
            r#"select ?pred where {{ 
            {} ?pred ?obj . }}
        "#,
            subject
        );
        let url = Url::parse_with_params(&self.endpoint_iri, &[("query", query)])?;
        println!("Url: {}", url);
        let body = client.get(url).send()?.text()?;

        if let QueryResultsReader::Solutions(solutions) =
            json_parser.read_results(body.as_bytes())?
        {
            for solution in solutions {
                let sol = solution?;
                match sol.get("pred") {
                    Some(v) => match v {
                        Term::NamedNode(n) => {
                            results.insert(n.clone());
                        }
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            }
            Ok(results)
        } else {
            todo!()
        }
    }

    async fn get_objects_for_subject_predicate(
        &self,
        subject: &Subject,
        pred: &NamedNode,
    ) -> Result<HashSet<Term>, SRDFSPARQLError> {
        todo!();
    }

    async fn get_subjects_for_object_predicate(
        &self,
        object: &Term,
        pred: &NamedNode,
    ) -> Result<HashSet<Subject>, SRDFSPARQLError> {
        todo!();
    }
}

impl SRDF for SRDFSPARQL {
    fn get_predicates_for_subject(
        &self,
        subject: &Subject,
    ) -> Result<HashSet<NamedNode>, SRDFSPARQLError> {
        let mut results = HashSet::new();
        let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);
        let mut headers = header::HeaderMap::new();
        headers.insert(
            ACCEPT,
            header::HeaderValue::from_static("application/sparql-results+json"),
        );
        headers.insert(USER_AGENT, header::HeaderValue::from_static("Rust App"));
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;
        let query = format!(
            r#"select ?pred where {{ 
            {} ?pred ?obj . }}
        "#,
            subject
        );
        let url = Url::parse_with_params(&self.endpoint_iri, &[("query", query)])?;
        println!("Url: {}", url);
        let body = client.get(url).send()?.text()?;

        if let QueryResultsReader::Solutions(solutions) =
            json_parser.read_results(body.as_bytes())?
        {
            for solution in solutions {
                let sol = solution?;
                match sol.get("pred") {
                    Some(v) => match v {
                        Term::NamedNode(n) => {
                            results.insert(n.clone());
                        }
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            }
            Ok(results)
        } else {
            todo!()
        }
    }

    fn get_objects_for_subject_predicate(
        &self,
        subject: &Subject,
        pred: &NamedNode,
    ) -> Result<HashSet<Term>, SRDFSPARQLError> {
        todo!();
    }

    fn get_subjects_for_object_predicate(
        &self,
        object: &Term,
        pred: &NamedNode,
    ) -> Result<HashSet<Subject>, SRDFSPARQLError> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use oxrdf::{NamedNode, Subject};
    use srdf::AsyncSRDF;

    use super::*;

    #[test]
    fn check_sparql() {
        let wikidata = SRDFSPARQL {
            endpoint_iri: "https://query.wikidata.org/sparql".to_string(),
        };
        let q80: Subject = Subject::NamedNode(NamedNode::new_unchecked(
            "http://www.wikidata.org/entity/Q80".to_string(),
        ));
        let maybe_data = wikidata.get_predicates_for_subject(&q80);
        let data = maybe_data.unwrap();
        let p19: NamedNode =
            NamedNode::new_unchecked("http://www.wikidata.org/prop/P19".to_string());

        assert_eq!(data.contains(&p19), true);
    }
}
