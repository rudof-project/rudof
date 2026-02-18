use rdf::rdf_core::parser::rdf_node_parser::constructors::{
    ObjectsPropertyParser, SingleBoolPropertyParser, SingleIntegerPropertyParser,
};
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rdf::rdf_core::term::Object;
use rdf::rdf_core::term::literal::ConcreteLiteral;
use rdf::rdf_core::{FocusRDF, RDFError, SHACLPath};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;
use std::collections::HashSet;
use std::marker::PhantomData;

/// This parser gets the siblings of a focus node
/// Siblings are the other qualified value shapes that share the same parent(s)
/// The defnition in the spec is: https://www.w3.org/TR/shacl12-core/#dfn-sibling-shapes
/// "Let Q be a shape in shapes graph G that declares a qualified cardinality constraint
/// (by having values for sh:qualifiedValueShape and at least one of sh:qualifiedMinCount
/// or sh:qualifiedMaxCount).
/// Let ps be the set of shapes in G that have Q as a value of sh:property.
/// If Q has true as a value for sh:qualifiedValueShapesDisjoint then the set of sibling
/// shapes for Q is defined as the set of all values of the SPARQL property path
/// sh:property/sh:qualifiedValueShape for any shape in ps minus the value of
/// sh:qualifiedValueShape of Q itself. The set of sibling shapes is empty otherwise."
struct QualifiedValueShapeSiblings<RDF: FocusRDF> {
    _marker: PhantomData<RDF>,
    property_qualified_value_shape_path: SHACLPath,
}

impl<RDF> RDFNodeParse<RDF> for QualifiedValueShapeSiblings<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<Object>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match rdf.get_focus() {
            Some(focus) => {
                let mut siblings = Vec::new();
                let maybe_disjoint =
                    rdf.object_for(focus, &ShaclVocab::sh_qualified_value_shapes_disjoint().clone().into())?;
                if let Some(disjoint) = maybe_disjoint {
                    match disjoint {
                        Object::Literal(ConcreteLiteral::BooleanLiteral(true)) => {
                            let qvs = rdf.objects_for(focus, &ShaclVocab::sh_qualified_value_shape().clone().into())?;
                            if !qvs.is_empty() {
                                let ps = rdf.subjects_for(&ShaclVocab::sh_property().clone().into(), focus)?;
                                for property_parent in ps {
                                    let candidate_siblings = rdf.objects_for_shacl_path(
                                        &property_parent,
                                        &self.property_qualified_value_shape_path,
                                    )?;
                                    for sibling in candidate_siblings {
                                        if !qvs.contains(&sibling) {
                                            let sibling_node = RDF::term_as_object(&sibling)?;
                                            siblings.push(sibling_node);
                                        }
                                    }
                                }
                            }
                        },
                        Object::Literal(ConcreteLiteral::BooleanLiteral(false)) => {},
                        _ => { /* TODO - Raise error */ },
                    }
                }
                Ok(siblings)
            },
            None => Err(RDFError::NoFocusNodeError),
        }
    }
}

pub(crate) fn qualified_value_shape<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    ObjectsPropertyParser::new(ShaclVocab::sh_qualified_value_shape().clone()).then(|qvs| {
        let set = qvs.into_iter().collect();
        parse_qualified_value_shape::<RDF>(set)
    })
}

fn qualified_value_shape_disjoint_parser<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Option<bool>> {
    SingleBoolPropertyParser::new(ShaclVocab::sh_qualified_value_shapes_disjoint().clone()).optional()
}

fn qualified_min_count_parser<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Option<isize>> {
    SingleIntegerPropertyParser::new(ShaclVocab::sh_qualified_min_count().clone()).optional()
}

fn qualified_max_count_parser<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Option<isize>> {
    SingleIntegerPropertyParser::new(ShaclVocab::sh_qualified_max_count().clone()).optional()
}

fn qualified_value_shape_siblings<RDF: FocusRDF>() -> QualifiedValueShapeSiblings<RDF> {
    QualifiedValueShapeSiblings {
        _marker: PhantomData,
        property_qualified_value_shape_path: SHACLPath::sequence(vec![
            SHACLPath::iri(ShaclVocab::sh_property().clone()),
            SHACLPath::iri(ShaclVocab::sh_qualified_value_shape().clone()),
        ]),
    }
}

fn parse_qualified_value_shape<RDF: FocusRDF>(qvs: HashSet<Object>) -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    qualified_value_shape_disjoint_parser()
        .and(qualified_min_count_parser())
        .and(qualified_max_count_parser())
        .and(qualified_value_shape_siblings())
        .flat_map(move |(((maybe_disjoint, maybe_mins), maybe_maxs), siblings)| {
            Ok(build_qualified_shape(
                qvs.clone(),
                maybe_disjoint,
                maybe_mins,
                maybe_maxs,
                siblings,
            ))
        })
}

fn build_qualified_shape(
    terms: HashSet<Object>,
    disjoint: Option<bool>,
    q_min_count: Option<isize>,
    q_max_count: Option<isize>,
    siblings: Vec<Object>,
) -> Vec<Component> {
    let mut result = Vec::new();
    for term in terms {
        let shape = Component::QualifiedValueShape {
            shape: term.clone(),
            q_min_count,
            q_max_count,
            disjoint,
            siblings: siblings.clone(),
        };
        result.push(shape);
    }
    result
}
