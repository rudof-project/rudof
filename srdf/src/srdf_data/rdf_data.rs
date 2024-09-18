use crate::lang::Lang;
use crate::literal::Literal;
use crate::numeric_literal::NumericLiteral;
use crate::query_srdf::QuerySolution;
use crate::Object;
use crate::QuerySRDF;
use crate::QuerySolutionIter;
use crate::RDFFormat;
use crate::SRDFBasic;
use crate::SRDFGraph;
use crate::SRDFSparql;
use crate::SRDFSparqlError;
use crate::RDF_TYPE_STR;
use colored::*;
use iri_s::IriS;
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, Subject as OxSubject,
    Term as OxTerm,
};
use oxrdfio::RdfFormat;
use prefixmap::IriRef;
use prefixmap::PrefixMap;
use rust_decimal::Decimal;
// use sparesults::QuerySolution as SparQuerySolution;
use std::str::FromStr;

use super::RdfDataError;

/// Generic abstraction that represents RDF Data which can be either behind SPARQL endpoints or an in-memory graph
#[derive(Debug, Clone)]
pub struct RdfData {
    endpoints: Vec<SRDFSparql>,
    _graph: Option<SRDFGraph>,
    prefixmap: PrefixMap,
}

impl RdfData {
    pub fn new() -> RdfData {
        RdfData {
            endpoints: Vec::new(),
            _graph: None,
            prefixmap: PrefixMap::new(),
        }
    }

    pub fn prefixmap(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        let str: String = format!("{}", bn);
        format!("{}", str.green())
    }

    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        let str: String = format!("{}", lit);
        format!("{}", str.red())
    }
}

impl SRDFBasic for RdfData {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = RdfDataError;

    fn subject_as_iri(subject: &Self::Subject) -> Option<Self::IRI> {
        match subject {
            OxSubject::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }

    fn subject_as_bnode(subject: &Self::Subject) -> Option<Self::BNode> {
        match subject {
            OxSubject::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }

    fn subject_is_iri(subject: &Self::Subject) -> bool {
        matches!(subject, OxSubject::NamedNode(_))
    }

    fn subject_is_bnode(subject: &Self::Subject) -> bool {
        matches!(subject, OxSubject::BlankNode(_))
    }

    fn term_as_iri(object: &Self::Term) -> Option<Self::IRI> {
        match object {
            OxTerm::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }

    fn term_as_bnode(object: &Self::Term) -> Option<Self::BNode> {
        match object {
            OxTerm::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }

    fn term_as_literal(object: &Self::Term) -> Option<Self::Literal> {
        match object {
            OxTerm::Literal(l) => Some(l.clone()),
            _ => None,
        }
    }

    fn term_as_object(term: &Self::Term) -> crate::Object {
        match term {
            OxTerm::BlankNode(bn) => Object::BlankNode(bn.as_str().to_string()),
            OxTerm::Literal(lit) => {
                let lit = lit.to_owned();
                match lit.destruct() {
                    (s, None, None) => Object::Literal(Literal::StringLiteral {
                        lexical_form: s,
                        lang: None,
                    }),
                    (s, None, Some(lang)) => Object::Literal(Literal::StringLiteral {
                        lexical_form: s,
                        lang: Some(Lang::new(lang.as_str())),
                    }),
                    (s, Some(datatype), _) => {
                        let iri_s = Self::iri2iri_s(&datatype);
                        Object::Literal(Literal::DatatypeLiteral {
                            lexical_form: s,
                            datatype: IriRef::Iri(iri_s),
                        })
                    }
                }
            }
            OxTerm::NamedNode(iri) => Object::Iri(Self::iri2iri_s(iri)),
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn object_as_term(obj: &crate::Object) -> Self::Term {
        match obj {
            Object::Iri(iri) => Self::iri_s2term(iri),
            Object::BlankNode(bn) => Self::bnode_id2term(bn),
            Object::Literal(lit) => {
                let literal: OxLiteral = match lit {
                    Literal::StringLiteral { lexical_form, lang } => match lang {
                        Some(lang) => OxLiteral::new_language_tagged_literal_unchecked(
                            lexical_form,
                            lang.to_string(),
                        ),
                        None => OxLiteral::new_simple_literal(lexical_form),
                    },
                    Literal::DatatypeLiteral {
                        lexical_form,
                        datatype,
                    } => OxLiteral::new_typed_literal(lexical_form, cnv_iri_ref(datatype)),
                    Literal::NumericLiteral(n) => match n {
                        NumericLiteral::Integer(n) => {
                            let n: i128 = *n as i128;
                            OxLiteral::from(n)
                        }
                        NumericLiteral::Decimal(d) => {
                            let decimal = cnv_decimal(d);
                            OxLiteral::from(decimal)
                        }
                        NumericLiteral::Double(d) => OxLiteral::from(*d),
                    },
                    Literal::BooleanLiteral(b) => OxLiteral::from(*b),
                };
                OxTerm::Literal(literal)
            }
        }
    }

    fn term_is_iri(object: &Self::Term) -> bool {
        matches!(object, OxTerm::NamedNode(_))
    }

    fn term_is_bnode(object: &Self::Term) -> bool {
        matches!(object, OxTerm::BlankNode(_))
    }

    fn term_is_literal(object: &Self::Term) -> bool {
        matches!(object, OxTerm::Literal(_))
    }

    fn term_as_subject(object: &Self::Term) -> Option<Self::Subject> {
        match object {
            OxTerm::NamedNode(n) => Some(OxSubject::NamedNode(n.clone())),
            OxTerm::BlankNode(b) => Some(OxSubject::BlankNode(b.clone())),
            _ => None,
        }
    }

    fn subject_as_term(subject: &Self::Subject) -> Self::Term {
        match subject {
            OxSubject::NamedNode(n) => OxTerm::NamedNode(n.clone()),
            OxSubject::BlankNode(b) => OxTerm::BlankNode(b.clone()),
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => unimplemented!(),
        }
    }

    fn lexical_form(literal: &Self::Literal) -> &str {
        literal.value()
    }

    fn lang(literal: &Self::Literal) -> Option<String> {
        literal.language().map(|s| s.to_string())
    }

    fn datatype(literal: &Self::Literal) -> Self::IRI {
        literal.datatype().into_owned()
    }

    fn iri_s2iri(iri_s: &iri_s::IriS) -> Self::IRI {
        iri_s.as_named_node().clone()
    }

    fn term_s2term(term: &oxrdf::Term) -> Self::Term {
        term.clone()
    }

    fn bnode_id2bnode(id: &str) -> Self::BNode {
        OxBlankNode::new_unchecked(id)
    }

    fn iri_as_term(iri: Self::IRI) -> Self::Term {
        OxTerm::NamedNode(iri)
    }

    fn iri_as_subject(iri: Self::IRI) -> Self::Subject {
        OxSubject::NamedNode(iri)
    }

    fn bnode_as_term(bnode: Self::BNode) -> Self::Term {
        OxTerm::BlankNode(bnode)
    }

    fn bnode_as_subject(bnode: Self::BNode) -> Self::Subject {
        OxSubject::BlankNode(bnode)
    }

    fn iri2iri_s(iri: &Self::IRI) -> iri_s::IriS {
        IriS::from_named_node(iri)
    }

    fn qualify_iri(&self, node: &Self::IRI) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap();
        self.prefixmap.qualify(&iri)
    }

    fn qualify_subject(&self, subj: &Self::Subject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_iri(n),
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => unimplemented!(),
        }
    }

    fn qualify_term(&self, term: &Self::Term) -> String {
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

    fn resolve_prefix_local(
        &self,
        prefix: &str,
        local: &str,
    ) -> Result<iri_s::IriS, prefixmap::PrefixMapError> {
        let iri = self.prefixmap.resolve_prefix_local(prefix, local)?;
        Ok(iri.clone())
    }
}

impl QuerySRDF for RdfData {
    fn query_select(&self, query: &str) -> Result<QuerySolutionIter<RdfData>, RdfDataError>
    where
        Self: Sized,
    {
        // let iter = QuerySolutionIter::empty();
        for endpoint in &self.endpoints {
            let iter1 = endpoint.query_select(query)?;
            let _iter2 = iter1.map(|s| cnv_sol(s));
            // iter.chain(iter2);
        }
        todo!() // Ok(iter))
    }

    fn query_ask(&self, _query: &str) -> Result<bool, Self::Err> {
        todo!()
    }
}

fn cnv_sol(
    _sol: Result<QuerySolution<SRDFSparql>, SRDFSparqlError>,
) -> Result<QuerySolution<RdfData>, RdfDataError> {
    todo!()
}

fn _cnv_rdf_format(rdf_format: RDFFormat) -> RdfFormat {
    match rdf_format {
        RDFFormat::NTriples => RdfFormat::NTriples,
        RDFFormat::Turtle => RdfFormat::Turtle,
        RDFFormat::RDFXML => RdfFormat::RdfXml,
        RDFFormat::TriG => RdfFormat::TriG,
        RDFFormat::N3 => RdfFormat::N3,
        RDFFormat::NQuads => RdfFormat::NQuads,
    }
}

fn _rdf_type() -> OxNamedNode {
    OxNamedNode::new_unchecked(RDF_TYPE_STR)
}

fn cnv_iri_ref(iri_ref: &IriRef) -> OxNamedNode {
    OxNamedNode::new_unchecked(iri_ref.to_string())
}

fn cnv_decimal(_d: &Decimal) -> oxsdatatypes::Decimal {
    todo!()
}
