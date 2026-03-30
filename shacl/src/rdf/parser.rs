use crate::ast::error::ASTError;
use crate::ast::{ASTSchema, ASTShape};
use crate::rdf::error::ShaclParserError;
use crate::rdf::parsers::{node_shape, property_shape};
use crate::rdf::State;
use itertools::Itertools;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{ListParser, SuccessParser};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::parser::RDFParse;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::Triple;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, ShaclVocab};
use rudof_rdf::rdf_core::{Any, FocusRDF, Matcher, Rdf};
use std::collections::{HashMap, HashSet};

pub(crate) struct ShaclParser<RDF: FocusRDF> {
    rdf_parser: RDFParse<RDF>,
    shapes: HashMap<Object, ASTShape>,
}

impl<RDF: FocusRDF> ShaclParser<RDF> {
    pub fn new(rdf: RDF) -> Self<> {
        Self {
            rdf_parser: RDFParse::new(rdf),
            shapes: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<ASTSchema, ASTError> {
        let pm = self.rdf_parser.prefixmap().unwrap_or_default();

        let mut state: State = self.shapes_candidates().map_err(|_| ASTError::ShapeNotFound {
            shape: Box::new(Object::BlankNode("TODO".to_string()))
        })?.into();

        while let Some(node) = state.pop_pending() {
            if self.shapes.get(&node).is_some() {
                self.rdf_parser.rdf_mut().set_focus(&node.clone().into());
                let shape = shape()
                    .parse_focused(self.rdf_parser.rdf_mut())
                    .map_err(|_| ASTError::ShapeNotFound { shape: Box::new(Object::BlankNode("TODO".to_string())) });
                self.shapes.insert(node, shape?);
            }
        }

        Ok(ASTSchema::new()
            .with_prefixmap(pm)
            .with_shapes(self.shapes.clone()) // TODO - Maybe avoid the shapes clone
        )
    }

    /// Shapes candidates are defined in Appendix A of SHACL spec (Syntax rules)
    /// The text is:
    /// A shape is an IRI or blank node s that fulfills at least one of the following conditions in the shapes graph:
    /// - s is a SHACL instance of sh:NodeShape or sh:PropertyShape.
    /// - s is subject of a triple that has sh:targetClass, sh:targetNode, sh:targetObjectsOf or sh:targetSubjectsOf as predicate.
    /// - s is subject of a triple that has a parameter as predicate.
    /// - s is a value of a shape-expecting, non-list-taking parameter such as sh:node,
    ///   or a member of a SHACL list that is a value of a shape-expecting and list-taking parameter such as sh:or.
    fn shapes_candidates(&mut self) -> Result<Vec<Object>, ShaclParserError> {
        // instances of `sh:NodeShape`
        let mut node_shapes_instances = self.get_triples::<_, RDF::IRI, RDF::Term>(
            &Any, &RdfVocab::rdf_type().clone().into(), &ShaclVocab::sh_node_shape().clone().into()
        )?;
        // instances of `sh:PropertyShape`
        let property_shapes_instances = self.get_triples::<_, RDF::IRI, RDF::Term>(
            &Any, &RdfVocab::rdf_type().clone().into(), &ShaclVocab::sh_property_shape().clone().into()
        )?;
        // instances of `sh:Shape`
        let shape_instances = self.get_triples::<_, RDF::IRI, RDF::Term>(
            &Any, &RdfVocab::rdf_type().clone().into(), &ShaclVocab::sh_shape().clone().into()
        )?;
        // subjects of sh:targetClass
        let subjects_target_class = self.get_triples::<_, RDF::IRI, _>(
            &Any, &ShaclVocab::sh_target_class().clone().into(), &Any
        )?;
        // subjects of sh:targetSubjectsOf
        let subjects_target_subjects_of = self.get_triples::<_, RDF::IRI, _>(
            &Any, &ShaclVocab::sh_target_subjects_of().clone().into(), &Any
        )?;
        // subjects of sh:targetObjectsOf
        let subjects_target_objects_of = self.get_triples::<_, RDF::IRI, _>(
            &Any, &ShaclVocab::sh_target_objects_of().clone().into(), &Any
        )?;
        // subjects of sh:targetNode
        let subjects_target_node = self.get_triples::<_, RDF::IRI, _>(
            &Any, &ShaclVocab::sh_target_node().clone().into(), &Any
        )?;
        // Search shape expecting parameters: https://www.w3.org/TR/shacl12-core/#dfn-shape-expecting
        // elements of `sh:and` list
        let sh_and_values = self.get_triples_list(
            &ShaclVocab::sh_and().clone().into(),
            "sh:and",
            |v| ShaclParserError::AndValueNoSubject { term: v.to_string() },
        )?;
        // elements of `sh:or` list
        let sh_or_values = self.get_triples_list(
            &ShaclVocab::sh_or().clone().into(),
            "sh:or",
            |v| ShaclParserError::OrValueNoObject { term: v.to_string() },
        )?;
        // elements of `sh:not` list
        let sh_not_values = self.objects_with_predicate(&ShaclVocab::sh_not().clone().into())?;
        // subjects with property `sh:property`
        let subjects_property = self.objects_with_predicate(&ShaclVocab::sh_property().clone().into())?;
        // elements of `sh:node` list
        let sh_qualified_value_shape_nodes = self.objects_with_predicate(&ShaclVocab::sh_qualified_value_shape().clone().into())?;
        // elements of `sh:node` list
        let sh_node_values = self.objects_with_predicate(&ShaclVocab::sh_node().clone().into())?;
        // elements of `sh:xone` list
        let sh_xone_values = self.get_triples_list(
            &ShaclVocab::sh_xone().clone().into(),
            "sh:xone",
            |v| ShaclParserError::XOneValueNoSubject { term: v.to_string() },
        )?;
        // elements of `sh:reifierShape` list
        let sh_reifier_shape_values = self.objects_with_predicate(&ShaclVocab::sh_reifier_shape().clone().into())?;

        node_shapes_instances.extend(property_shapes_instances);
        node_shapes_instances.extend(shape_instances);
        node_shapes_instances.extend(subjects_target_class);
        node_shapes_instances.extend(subjects_target_subjects_of);
        node_shapes_instances.extend(subjects_target_objects_of);
        node_shapes_instances.extend(subjects_target_node);
        node_shapes_instances.extend(sh_and_values);
        node_shapes_instances.extend(sh_or_values);
        node_shapes_instances.extend(sh_not_values);
        node_shapes_instances.extend(subjects_property);
        node_shapes_instances.extend(sh_qualified_value_shape_nodes);
        node_shapes_instances.extend(sh_node_values);
        node_shapes_instances.extend(sh_xone_values);
        node_shapes_instances.extend(sh_reifier_shape_values);

        Ok(node_shapes_instances
            .into_iter()
            .map(|s| RDF::subject_as_node(&s))
            .try_collect()?
        )
    }

    fn get_triples<S, P, O>(&self, s: &S, p: &P, o: &O) -> Result<HashSet<RDF::Subject>, ShaclParserError>
    where
        S: Matcher<RDF::Subject>,
        P: Matcher<RDF::IRI>,
        O: Matcher<RDF::Term>
    {
        Ok(self
            .rdf_parser
            .rdf()
            .triples_matching(s, p, o)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect())
    }

    fn get_triples_list<ERR>(&mut self, pred: &RDF::IRI, context: &str, err_fn: ERR) -> Result<HashSet<RDF::Subject>, ShaclParserError>
    where
        ERR: Fn(RDF::Term) -> ShaclParserError
    {
        let mut rs = HashSet::new();

        for subject in self.objects_with_predicate(&pred)? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = ListParser::new().parse_focused(self.rdf_parser.rdf_mut())?;
            for v in vs {
                if let Ok(subj) = term_to_subject::<RDF>(&v, context) {
                    rs.insert(subj);
                } else {
                    return Err(err_fn(v))
                }
            }
        }

        Ok(rs)
    }

    fn objects_with_predicate(&self, pred: &RDF::IRI) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let msg = format!("objects with predicate {pred}");
        let subjects = self
            .rdf_parser
            .rdf()
            .triples_with_predicate(pred)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_object)
            .flat_map(|t| term_to_subject::<RDF>(&t, msg.as_str()))
            .collect();
        Ok(subjects)
    }
}

fn term_to_subject<RDF: FocusRDF>(term: &RDF::Term, context: &str) -> Result<RDF::Subject, ShaclParserError> {
    RDF::term_as_subject(term).map_err(|_| ShaclParserError::ExpectedSubject {
        term: term.to_string(),
        context: context.to_string(),
    })
}

fn shape<RDF: FocusRDF + 'static>() -> impl RDFNodeParse<RDF, Output = ASTShape> {
    node_shape()
        .then(move |ns| SuccessParser::new(ASTShape::NodeShape(Box::new(ns))))
        .or(property_shape()
            .then(|ps| SuccessParser::new(ASTShape::PropertyShape(Box::new(ps))))
        )
}