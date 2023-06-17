use std::fs::File;
use oxhttp::Client;
use oxhttp::model::{Request, Method};
use srdf::SRDF;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use oxiri::Iri;

use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, Subject as OxSubject,
    Term as OxTerm, Triple as OxTriple, Graph, TripleRef,
};
use srdf::bnode::BNode;
use srdf::iri::IRI;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SRDFSPARQLError {
}

struct SRDFSPARQL {
    endpoint_iri: String
}

impl SRDF for SRDFSPARQL {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;

    fn get_predicates_subject(&self, subject: &OxSubject) -> Vec<OxNamedNode> {
        let client = Client::new();
        let response = client.request(
            Request::builder(Method::GET, self.endpoint_iri.parse().unwrap()).build()
        ).unwrap();

        todo!();
    } 
    fn get_objects_for_subject_predicate(&self, subject: &OxSubject, pred: &OxNamedNode) -> Vec<OxTerm> {
        todo!();
    }
    fn get_subjects_for_object_predicate(&self, object: &OxTerm, pred: &OxNamedNode) -> Vec<OxSubject> {
        todo!();
    }

    fn subject2iri(&self, subject:&OxSubject) -> Option<OxNamedNode> {
        match subject {
            OxSubject::NamedNode(n) => Some(n.clone()),
            _ => None
        }
    }
    fn subject2bnode(&self, subject:&OxSubject) -> Option<OxBlankNode> {
        match subject {
            OxSubject::BlankNode(b) => Some(b.clone()),
            _ => None
        }
    }
    fn subject_is_iri(&self, subject:&OxSubject) -> bool {
        match subject {
            OxSubject::NamedNode(_) => true,
            _ => false
        }
    }
    fn subject_is_bnode(&self, subject:&OxSubject) -> bool {
        match subject {
            OxSubject::BlankNode(_) => true,
            _ => false
        }
    }

    fn object2iri(&self, object:&OxTerm) -> Option<OxNamedNode> {
        match object {
            OxTerm::NamedNode(n) => Some(n.clone()),
            _ => None
        }
    }
    fn object2bnode(&self, object:&OxTerm) -> Option<OxBlankNode> {
        match object {
            OxTerm::BlankNode(b) => Some(b.clone()),
            _ => None
        }
    }
    fn object2literal(&self, object:&OxTerm) -> Option<OxLiteral> {
        match object {
            OxTerm::Literal(l) => Some(l.clone()),
            _ => None
        }
    }
    fn object_is_iri(&self, object: &OxTerm) -> bool {
        match object {
            OxTerm::NamedNode(_) => true,
            _ => false
        }
    }
    fn object_is_bnode(&self, object:&OxTerm) -> bool {
        match object {
            OxTerm::BlankNode(_) => true,
            _ => false
        }
    }

    fn object_is_literal(&self, object:&OxTerm) -> bool {
        match object {
            OxTerm::Literal(_) => true,
            _ => false
        }
    }

    fn lexical_form(&self, literal: &OxLiteral) -> String {
        literal.to_string()
    }
    fn lang(&self, literal: &OxLiteral) -> Option<String> {
        literal.language().map(|s| s.to_string())
    }
    fn datatype(&self, literal: &OxLiteral) -> OxNamedNode {
        literal.datatype().into_owned()
    }


}



#[cfg(test)]
mod tests {
}
