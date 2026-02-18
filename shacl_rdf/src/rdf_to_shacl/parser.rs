use crate::error::ShaclParserError;
use crate::rdf_to_shacl::parsers::{node_shape, property_shape};
use prefixmap::PrefixMap;
use rdf::rdf_core::parser::rdf_node_parser::ParserExt;
use rdf::rdf_core::{
    Any, FocusRDF, RDFError, Rdf,
    parser::{
        RDFParse,
        rdf_node_parser::{
            RDFNodeParse,
            constructors::{ListParser, SuccessParser},
        },
    },
    term::{Object, Triple},
    vocab::rdf_type,
};
use shacl_ast::{ShaclVocab, schema::ShaclSchema, shape::Shape};
use std::collections::{HashMap, HashSet};

/// State used during the parsing process
/// This is used to keep track of pending shapes to be parsed
struct State {
    pending: Vec<Object>,
}

impl State {
    fn pop_pending(&mut self) -> Option<Object> {
        self.pending.pop()
    }
}

impl From<Vec<Object>> for State {
    fn from(value: Vec<Object>) -> Self {
        Self { pending: value }
    }
}

pub struct ShaclParser<RDF: FocusRDF> {
    rdf_parser: RDFParse<RDF>,
    shapes: HashMap<Object, Shape<RDF>>,
}

impl<RDF: FocusRDF> ShaclParser<RDF> {
    pub fn new(rdf: RDF) -> ShaclParser<RDF> {
        ShaclParser {
            rdf_parser: RDFParse::new(rdf),
            shapes: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<ShaclSchema<RDF>, ShaclParserError> {
        let prefixmap: PrefixMap = self.rdf_parser.prefixmap().unwrap_or_default();

        let mut state: State = self.shapes_candidates()?.into();
        while let Some(node) = state.pop_pending() {
            if let std::collections::hash_map::Entry::Vacant(e) = self.shapes.entry(node.clone()) {
                self.rdf_parser.rdf_mut().set_focus(&node.clone().into());
                let shape = Self::shape()
                    .parse_focused(self.rdf_parser.rdf_mut())
                    .map_err(|e| ShaclParserError::RDFParseError { err: e })?;
                e.insert(shape);
            }
        }

        Ok(ShaclSchema::new()
            .with_prefixmap(prefixmap)
            .with_shapes(self.shapes.clone()))
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
        let node_shape_instances: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching(&Any, &Self::rdf_type_iri(), &Self::sh_node_shape_iri())
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // instances of `sh:PropertyShape`
        let property_shapes_instances: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching(&Any, &Self::rdf_type_iri(), &Self::sh_property_shape_iri())
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Instances of `sh:Shape`
        let shape_instances: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching(&Any, &Self::rdf_type_iri(), &Self::sh_shape_iri())
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetClass
        let subjects_target_class: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching::<_, RDF::IRI, _>(&Any, &ShaclVocab::sh_target_class().clone().into(), &Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetSubjectsOf
        let subjects_target_subjects_of: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching::<_, RDF::IRI, _>(&Any, &ShaclVocab::sh_target_subjects_of().clone().into(), &Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetObjectsOf
        let subjects_target_objects_of: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching::<_, RDF::IRI, _>(&Any, &ShaclVocab::sh_target_objects_of().clone().into(), &Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetNode
        let subjects_target_node: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching::<_, RDF::IRI, _>(&Any, &ShaclVocab::sh_target_node().clone().into(), &Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Search shape expecting parameters: https://www.w3.org/TR/shacl12-core/#dfn-shape-expecting
        // elements of `sh:and` list
        let sh_and_values = self.get_sh_and_values()?;

        // elements of `sh:or` list
        let sh_or_values = self.get_sh_or_values()?;

        // elements of `sh:not` list
        let sh_not_values = self.get_sh_not_values()?;

        // subjects with property `sh:property`
        let subjects_property = self.objects_with_predicate(Self::sh_property_iri())?;

        // elements of `sh:node` list
        let sh_qualified_value_shape_nodes = self.get_sh_qualified_value_shape()?;

        // elements of `sh:node` list
        let sh_node_values = self.get_sh_node_values()?;

        // elements of `sh:xone` list
        let sh_xone_values = self.get_sh_xone_values()?;

        // elements of `sh:reifierShape` list
        let sh_reifier_shape_values = self.get_sh_reifier_shape_values()?;

        // I would prefer a code like: node_shape_instances.union(subjects_property).union(...)
        // But looking to the union API in HashSet, I think it can't be chained
        let mut candidates = HashSet::new();
        candidates.extend(node_shape_instances);
        candidates.extend(subjects_property);
        candidates.extend(sh_or_values);
        candidates.extend(sh_xone_values);
        candidates.extend(sh_and_values);
        candidates.extend(sh_not_values);
        candidates.extend(sh_qualified_value_shape_nodes);
        candidates.extend(sh_node_values);
        candidates.extend(property_shapes_instances);
        candidates.extend(sh_reifier_shape_values);
        candidates.extend(shape_instances);
        candidates.extend(subjects_target_class);
        candidates.extend(subjects_target_subjects_of);
        candidates.extend(subjects_target_objects_of);
        candidates.extend(subjects_target_node);

        Ok(subjects_as_nodes::<RDF>(candidates)?)
    }

    fn get_sh_or_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for subject in self.objects_with_predicate(Self::sh_or_iri())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = ListParser::new().parse_focused(self.rdf_parser.rdf_mut())?;
            for v in vs {
                if let Ok(subj) = term_to_subject::<RDF>(&v, "sh:or") {
                    rs.insert(subj.clone());
                } else {
                    return Err(ShaclParserError::OrValueNoSubject { term: format!("{v}") });
                }
            }
        }
        Ok(rs)
    }

    fn get_sh_xone_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for subject in self.objects_with_predicate(Self::sh_xone_iri())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = ListParser::new().parse_focused(self.rdf_parser.rdf_mut())?;
            for v in vs {
                if let Ok(subj) = &term_to_subject::<RDF>(&v, "sh:xone") {
                    rs.insert(subj.clone());
                } else {
                    return Err(ShaclParserError::XOneValueNoSubject { term: format!("{v}") });
                }
            }
        }
        Ok(rs)
    }

    fn get_sh_reifier_shape_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_reifier_shape_iri())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn get_sh_and_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for subject in self.objects_with_predicate(Self::sh_and_iri())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = ListParser::new().parse_focused(self.rdf_parser.rdf_mut())?;
            for v in vs {
                if let Ok(subj) = term_to_subject::<RDF>(&v, "sh:and") {
                    rs.insert(subj);
                } else {
                    return Err(ShaclParserError::AndValueNoSubject { term: format!("{v}") });
                }
            }
        }
        Ok(rs)
    }

    fn get_sh_not_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_not_iri())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn get_sh_qualified_value_shape(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_qualified_value_shape_iri())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn get_sh_node_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_node_iri())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn objects_with_predicate(&self, pred: RDF::IRI) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let msg = format!("objects with predicate {pred}");
        let values_as_subjects = self
            .rdf_parser
            .rdf()
            .triples_with_predicate(&pred)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_object)
            .flat_map(|t| term_to_subject::<RDF>(&t.clone(), msg.as_str()))
            .collect();
        Ok(values_as_subjects)
    }

    fn rdf_type_iri() -> RDF::IRI {
        rdf_type().clone().into()
    }

    fn sh_node_shape_iri() -> RDF::Term {
        RDF::iris_as_term(ShaclVocab::sh_node_shape())
    }

    fn sh_property_shape_iri() -> RDF::Term {
        RDF::iris_as_term(ShaclVocab::sh_property_shape())
    }

    fn sh_shape_iri() -> RDF::Term {
        RDF::iris_as_term(ShaclVocab::sh_shape())
    }

    fn sh_property_iri() -> RDF::IRI {
        ShaclVocab::sh_property().clone().into()
    }

    fn sh_or_iri() -> RDF::IRI {
        ShaclVocab::sh_or().clone().into()
    }

    fn sh_xone_iri() -> RDF::IRI {
        ShaclVocab::sh_xone().clone().into()
    }

    fn sh_reifier_shape_iri() -> RDF::IRI {
        ShaclVocab::sh_reifier_shape().clone().into()
    }

    fn sh_and_iri() -> RDF::IRI {
        ShaclVocab::sh_and().clone().into()
    }

    fn sh_not_iri() -> RDF::IRI {
        ShaclVocab::sh_not().clone().into()
    }

    fn sh_node_iri() -> RDF::IRI {
        ShaclVocab::sh_node().clone().into()
    }

    fn sh_qualified_value_shape_iri() -> RDF::IRI {
        ShaclVocab::sh_qualified_value_shape().clone().into()
    }

    fn shape() -> impl RDFNodeParse<RDF, Output = Shape<RDF>>
    where
        RDF: FocusRDF + 'static,
    {
        node_shape()
            .then(move |ns| SuccessParser::new(Shape::NodeShape(Box::new(ns))))
            .or(property_shape().then(|ps| SuccessParser::new(Shape::PropertyShape(Box::new(ps)))))
    }
}

fn term_to_subject<RDF>(term: &RDF::Term, context: &str) -> Result<RDF::Subject, ShaclParserError>
where
    RDF: FocusRDF,
{
    RDF::term_as_subject(term).map_err(|_| ShaclParserError::ExpectedSubject {
        term: term.to_string(),
        context: context.to_string(),
    })
}

fn subjects_as_nodes<RDF: Rdf>(subjs: HashSet<RDF::Subject>) -> Result<Vec<Object>, RDFError> {
    subjs
        .into_iter()
        .map(|s| RDF::subject_as_node(&s).map_err(|_| RDFError::FailedSubjectToRDFNodeError { subject: s.to_string() }))
        .collect()
}
