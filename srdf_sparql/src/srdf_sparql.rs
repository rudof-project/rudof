use std::{collections::{HashSet, HashMap, hash_map::Entry}, str::FromStr, fmt::Display};
use colored::*;
use regex::Regex;
use async_trait::async_trait;
use iri_s::IriS;
use oxrdf::Literal;
use oxrdf::*;
use prefixmap::{IriRef, PrefixMap};
use reqwest::{
    header::{self, ACCEPT, USER_AGENT},
    Url, blocking::Client,
};
use sparesults::{QueryResultsFormat, QueryResultsParser, QueryResultsReader, QuerySolution};
use srdf::{AsyncSRDF, SRDFComparisons, SRDF};
use log::debug;
use crate::SRDFSparqlError;

type Result<A> = std::result::Result<A, SRDFSparqlError>;

/// Implements SRDF interface as a SPARQL endpoint
pub struct SRDFSparql {
    endpoint_iri: IriS,
    prefixmap: PrefixMap,
    client: Client
}

impl SRDFSparql {
    pub fn new(iri: &IriS) -> Result<SRDFSparql> {
        let client = sparql_client()?;
        Ok(SRDFSparql { 
         endpoint_iri: iri.clone(),
         prefixmap: PrefixMap::new(),
         client: client
        })
    }

    pub fn from_str(s: &str) -> Result<SRDFSparql> {
        let re_iri = Regex::new(r"<(.*)>").unwrap();
        if let Some(iri_str) = re_iri.captures(s) {
          let iri_s = IriS::from_str(&iri_str[1])?;
          let client = sparql_client()?;
          Ok(SRDFSparql { endpoint_iri: iri_s, prefixmap: PrefixMap::new(), client })
        } else {
            match s.to_lowercase().as_str() {
              "wikidata" => SRDFSparql::wikidata(),
              name => Err(SRDFSparqlError::UnknownEndpontName { name: name.to_string() })
            } 
        }
    }

    pub fn wikidata() -> Result<SRDFSparql> {
        let endpoint = SRDFSparql::new(&IriS::new_unchecked("https://query.wikidata.org/sparql"))?;
        Ok(endpoint.with_prefixmap(PrefixMap::wikidata()))
    }

    pub fn with_prefixmap(mut self, pm: PrefixMap) -> SRDFSparql {
        self.prefixmap = pm;
        self
    }

    fn show_blanknode(&self, bn: &BlankNode) -> String {
        let str: String = format!("{}", bn);
        format!("{}", str.green())
    }

    pub fn show_literal(&self, lit: &Literal) -> String {
        let str: String = format!("{}", lit);
        format!("{}", str.red())
    }


}

impl SRDFComparisons for SRDFSparql {
    type IRI = NamedNode;
    type BNode = BlankNode;
    type Literal = Literal;
    type Subject = Subject;
    type Term = Term;
    type Err = SRDFSparqlError;

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
        term_as_subject(object)
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

    fn iri_as_term(iri: NamedNode) -> Term {
        Term::NamedNode(iri)
    }

    fn iri_s2iri(iri_s: &IriS) -> Self::IRI {
        iri_s.as_named_node().clone()
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

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> std::result::Result<IriS, prefixmap::PrefixMapError> {
        self.prefixmap.resolve_prefix_local(prefix, local)
    }

    fn qualify_iri(&self, node: &NamedNode) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap();
        self.prefixmap.qualify(&iri)
    }


    fn qualify_subject(&self, subj: &Subject) -> String {
        match subj {
            Subject::BlankNode(bn) => self.show_blanknode(bn),
            Subject::NamedNode(n) => self.qualify_iri(n),
        }
    }

    
    fn qualify_term(&self, term: &Term) -> String {
        match term {
            Term::BlankNode(bn) => self.show_blanknode(bn),
            Term::Literal(lit) => self.show_literal(&lit),
            Term::NamedNode(n) => self.qualify_iri(n),
        }
    }

}

#[async_trait]
impl AsyncSRDF for SRDFSparql {
    type IRI = NamedNode;
    type BNode = BlankNode;
    type Literal = Literal;
    type Subject = Subject;
    type Term = Term;
    type Err = SRDFSparqlError;

    async fn get_predicates_subject(
        &self,
        subject: &Subject,
    ) -> Result<HashSet<NamedNode>> {
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
        subject: &Subject,
        pred: &NamedNode,
    ) -> Result<HashSet<Term>> {
        todo!();
    }

    async fn get_subjects_for_object_predicate(
        &self,
        object: &Term,
        pred: &NamedNode,
    ) -> Result<HashSet<Subject>> {
        todo!();
    }
}

impl SRDF for SRDFSparql {

    fn get_predicates_for_subject(
        &self,
        subject: &Subject,
    ) -> Result<HashSet<NamedNode>> {
        let query = format!(r#"select ?pred where {{ {} ?pred ?obj . }}"#,subject);
        debug!("SPARQL query (get predicates for subject {subject}): {}", query);
        let solutions = make_sparql_query(query.as_str(), &self.client, &self.endpoint_iri)?;
        let mut results = HashSet::new();
        for solution in solutions {
            let n = get_iri_solution(solution, "pred")?; 
            results.insert(n.clone());
        }
        Ok(results)
    }

    fn get_objects_for_subject_predicate(
        &self,
        subject: &Subject,
        pred: &NamedNode,
    ) -> Result<HashSet<Term>> {
        let query = format!(r#"select ?obj where {{ {} {} ?obj . }}"#, subject, pred);
        let solutions = make_sparql_query(query.as_str(), &self.client, &self.endpoint_iri)?;
        let mut results = HashSet::new();
        for solution in solutions {
            let n = get_object_solution(solution, "obj")?; 
            results.insert(n.clone());
        }
        Ok(results)
    }

    fn get_subjects_for_object_predicate(
        &self,
        object: &Term,
        pred: &NamedNode,
    ) -> Result<HashSet<Subject>> {
        let query = format!(r#"select ?subj where {{ ?subj {} {} . }}"#, pred, object);
        let solutions = make_sparql_query(query.as_str(), &self.client, &self.endpoint_iri)?;
        let mut results = HashSet::new();
        for solution in solutions {
            let n = get_subject_solution(solution, "subj")?; 
            results.insert(n.clone());
        }
        Ok(results)
    }

    fn outgoing_arcs(&self, 
        subject: &Self::Subject
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Term>>> {
        outgoing_neighs(subject.to_string().as_str(), &self.client, &self.endpoint_iri)
    }
}

fn sparql_client() -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        ACCEPT,
        header::HeaderValue::from_static("application/sparql-results+json"),
    );
    headers.insert(USER_AGENT, header::HeaderValue::from_static("Rust App"));
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

fn make_sparql_query(query: &str, client: &Client, endpoint_iri: &IriS) -> Result<Vec<QuerySolution>> {
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;
    debug!("SPARQL query: {}", url);
    let body = client.get(url).send()?.text()?;
    let mut results = Vec::new();
    let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);
    if let QueryResultsReader::Solutions(solutions) = json_parser.read_results(body.as_bytes())?  {
        for solution in solutions {
            let sol = solution?;
            results.push(sol)
        }
        Ok(results)
    } else {
        Err(SRDFSparqlError::ParsingBody{ body })
    }
}

#[derive(Debug)]
pub(crate) struct SparqlVars { 
values: Vec<String> 
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

fn outgoing_neighs(subject: &str, client: &Client, endpoint_iri: &IriS) -> Result<HashMap<NamedNode, HashSet<Term>>> {
    let pred = "pred";
    let obj = "obj";
    let query = format!("select ?{pred} ?{obj} where {{ {subject} ?{pred} ?{obj} }}");
    let url = Url::parse_with_params(endpoint_iri.as_str(), &[("query", query)])?;
    let body = client.get(url).send()?.text()?;
    let mut results: HashMap<NamedNode, HashSet<Term>> = HashMap::new();
    let json_parser = QueryResultsParser::from_format(QueryResultsFormat::Json);
    if let QueryResultsReader::Solutions(solutions) = json_parser.read_results(body.as_bytes())?  {
        for solution in solutions {
            let sol = solution?;
            match (sol.get(pred), sol.get(obj)) {
                (Some(p), Some(v)) => match p {
                   Term::NamedNode(iri) => {
                      match results.entry(iri.clone()) {
                        Entry::Occupied(mut vs) => {
                            vs.get_mut().insert(v.clone());
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(HashSet::from([v.clone()]));
                        }
                      }
                   },
                   _ => return Err(SRDFSparqlError::SPARQLSolutionErrorNoIRI { value: p.clone() }),        
                },
                (None,None) => {
                     return Err(SRDFSparqlError::NotFoundVarsInSolution { vars: SparqlVars::new(vec![pred.to_string(), obj.to_string()]), solution: format!("{sol:?}") })
                 } 
                 (None,Some(_)) => {
                    return Err(SRDFSparqlError::NotFoundVarsInSolution { vars: SparqlVars::new(vec![pred.to_string()]), solution: format!("{sol:?}") })
                } 
                (Some(_), None) => {
                    return Err(SRDFSparqlError::NotFoundVarsInSolution { vars: SparqlVars::new(vec![obj.to_string()]), solution: format!("{sol:?}") })
                } 
            }
        }
        Ok(results)
    } else {
        Err(SRDFSparqlError::ParsingBody{ body })
    }
}

fn get_iri_solution(solution: QuerySolution, name: &str) -> Result<NamedNode> {
    match solution.get(name) {
        Some(v) => match v {
           Term::NamedNode(n) => {
              Ok(n.clone())
           }
           _ => Err(SRDFSparqlError::SPARQLSolutionErrorNoIRI { value: v.clone() }),
          }
         None => {
             Err(SRDFSparqlError::NotFoundInSolution { value: name.to_string(), solution: format!("{solution:?}") })
         } 
     }
}

fn get_object_solution(solution: QuerySolution, name: &str) -> Result<Term> {
    match solution.get(name) {
        Some(v) => Ok(v.clone()),
        None => {
             Err(SRDFSparqlError::NotFoundInSolution { value: name.to_string(), solution: format!("{solution:?}") })
         } 
     }
}

fn get_subject_solution(solution: QuerySolution, name: &str) -> Result<Subject> {
    match solution.get(name) {
        Some(v) => match term_as_subject(v) {
            Some(s) => Ok(s),
            None => Err(SRDFSparqlError::SPARQLSolutionErrorNoSubject { value: v.clone() })
        },
        None => {
             Err(SRDFSparqlError::NotFoundInSolution { value: name.to_string(), solution: format!("{solution:?}") })
         } 
     }
}

fn term_as_subject(object: &Term) -> Option<Subject> {
    match object {
        Term::NamedNode(n) => Some(Subject::NamedNode(n.clone())),
        Term::BlankNode(b) => Some(Subject::BlankNode(b.clone())),
        _ => None,
    }
}


#[cfg(test)]
mod tests {
    use oxrdf::{NamedNode, Subject};
    use super::*;

    #[test]
    fn check_sparql() {
        let wikidata = SRDFSparql::wikidata().unwrap();
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
