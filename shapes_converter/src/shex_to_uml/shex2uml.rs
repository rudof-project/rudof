use std::collections::BTreeSet;
use std::io::Write;

use either::Either;
use prefixmap::error::PrefixMapError;
use prefixmap::{IriRef, PrefixMap};
use rudof_rdf::rdf_core::visualizer::uml_converter::{UmlConverter, UmlGenerationMode, errors::UmlConverterError};
use shex_ast::{
    Annotation, NodeKind, ObjectValue, Schema, Shape, ShapeExpr, ShapeExprLabel, ShapeExprWrapper, TripleExpr,
    ValueSetValue, XsFacet,
};
use tracing::trace;

use crate::shex_to_uml::{UmlLabel, UmlLabelType};
use crate::{
    find_annotation, object_value2string,
    shex_to_uml::{ShEx2UmlConfig, ShEx2UmlError, Uml},
};

use super::{Name, NodeId, UmlCardinality, UmlClass, UmlComponent, UmlEntry, ValueConstraint};

pub struct ShEx2Uml {
    config: ShEx2UmlConfig,
    current_uml: Uml,
    current_prefixmap: PrefixMap,
}

impl ShEx2Uml {
    pub fn new(config: &ShEx2UmlConfig) -> ShEx2Uml {
        ShEx2Uml {
            config: config.clone(),
            current_uml: Uml::new(),
            current_prefixmap: PrefixMap::new(),
        }
    }

    pub fn as_plantuml<W: Write>(&self, writer: &mut W, mode: &UmlGenerationMode) -> Result<(), UmlConverterError> {
        match mode {
            UmlGenerationMode::AllNodes => {
                self.current_uml
                    .as_plantuml_all(&self.config, writer)
                    .map_err(|e| UmlConverterError::UmlError { error: e.to_string() })?;
                Ok(())
            },
            UmlGenerationMode::Neighs(str) => {
                let label = UmlLabel::Class(str.clone());
                if let Some(node_id) = self.current_uml.get_node(&label) {
                    self.current_uml
                        .as_plantuml_neighs(&self.config, writer, &node_id)
                        .map_err(|e| UmlConverterError::UmlError { error: e.to_string() })?;
                    Ok(())
                } else {
                    Err(UmlConverterError::NotFoundLabel { name: str.clone() })
                }
            },
        }
    }

    pub fn convert(&mut self, shex: &Schema) -> Result<(), ShEx2UmlError> {
        self.current_prefixmap = shex.prefixmap().unwrap_or_default();
        if let Some(shapes) = shex.shapes() {
            for shape_decl in shapes {
                let mut name = self.shape_label2name(&shape_decl.id)?;
                let label = UmlLabel::Class(name.name());
                let (node_id, _found) = self.current_uml.get_node_adding_label(&label);
                let component = self.shape_expr2component(&mut name, &shape_decl.shape_expr, &node_id)?;
                self.current_uml.update_component(node_id, component)?;
            }
        }
        Ok(())
    }

    fn shape_label2name(&self, label: &ShapeExprLabel) -> Result<Name, ShEx2UmlError> {
        match label {
            ShapeExprLabel::IriRef { value } => iri_ref2name(value, &self.config, &None, &self.current_prefixmap),
            ShapeExprLabel::BNode { value } => Ok(Name::new(format!("_:{value}").as_str(), None)),
            ShapeExprLabel::Start => Ok(Name::new("start", None)),
        }
    }

    fn shape_expr2component(
        &mut self,
        name: &mut Name,
        shape_expr: &ShapeExpr,
        current_node_id: &NodeId,
    ) -> Result<UmlComponent, ShEx2UmlError> {
        match shape_expr {
            ShapeExpr::Shape(shape) => self.shape2component(name, shape, current_node_id),
            /*ShapeExpr::ShapeOr { shape_exprs } => {
                let cs: Vec<_> = shape_exprs
                    .iter()
                    .flat_map(|se| {
                        let c = self.shape_expr2component(name, &se.se, current_node_id)?;
                        Ok::<UmlComponent, ShEx2UmlError>(c)
                    })
                    .collect();
                Ok(UmlComponent::or(cs.into_iter()))
            }, */
            other => Err(ShEx2UmlError::not_implemented(
                format!("Complex shape expressions are not implemented yet\nShape: {other:?}").as_str(),
            )),
        }
    }

    fn shape2component(
        &mut self,
        name: &mut Name,
        shape: &Shape,
        current_node_id: &NodeId,
    ) -> Result<UmlComponent, ShEx2UmlError> {
        if let Some(label) = get_label(&shape.annotations, &self.current_prefixmap, &self.config)? {
            name.add_label(label.as_str())
        }
        let mut uml_class = UmlClass::new(name.clone());
        if let Some(extends) = &shape.extends {
            for e in extends.iter() {
                let extended_name = self.shape_label2name(e)?;
                let label = UmlLabel::Class(extended_name.name());
                let (extended_node, found) = self.current_uml.get_node_adding_label(&label);
                self.current_uml.add_extends(current_node_id, &extended_node);
                uml_class.add_extends(&extended_node);
                if !found {
                    self.current_uml
                        .add_component(extended_node, UmlComponent::class(UmlClass::new(extended_name)))?;
                }
            }
        }
        if let Some(te) = &shape.expression {
            match &te.te {
                TripleExpr::EachOf {
                    id: _,
                    expressions,
                    min: _,
                    max: _,
                    sem_acts: _,
                    annotations: _,
                } => {
                    for e in expressions {
                        match &e.te {
                            TripleExpr::TripleConstraint {
                                id: _,
                                negated: _,
                                inverse: _,
                                predicate,
                                value_expr,
                                min,
                                max,
                                sem_acts: _,
                                annotations,
                            } => {
                                let pred_name = mk_name(predicate, annotations, &self.config, &self.current_prefixmap)?;
                                let card = mk_card(min, max)?;
                                let value_constraint = if let Some(se) = value_expr {
                                    self.value_expr2value_constraint(se, current_node_id, &pred_name, &card)?
                                } else {
                                    ValueConstraint::default()
                                };
                                match value_constraint {
                                    ValueConstraint::None => {},
                                    _ => {
                                        let entry = UmlEntry::new(pred_name, value_constraint, card);
                                        uml_class.add_entry(entry)
                                    },
                                }
                            },
                            TripleExpr::EachOf { .. } => {
                                todo!()
                            },
                            TripleExpr::OneOf { .. } => {
                                todo!()
                            },
                            TripleExpr::Ref(_) => {
                                todo!()
                            },
                        }
                    }
                },
                TripleExpr::OneOf {
                    id: _,
                    expressions: _,
                    min: _,
                    max: _,
                    sem_acts: _,
                    annotations: _,
                } => {
                    todo!()
                },
                TripleExpr::TripleConstraint {
                    id: _,
                    negated: _,
                    inverse: _,
                    predicate,
                    value_expr,
                    min,
                    max,
                    sem_acts: _,
                    annotations,
                } => {
                    let pred_name = mk_name(predicate, annotations, &self.config, &self.current_prefixmap)?;
                    let card = mk_card(min, max)?;
                    let value_constraint = if let Some(se) = value_expr {
                        self.value_expr2value_constraint(se, current_node_id, &pred_name, &card)?
                    } else {
                        ValueConstraint::default()
                    };
                    match value_constraint {
                        ValueConstraint::None => {},
                        _ => {
                            let entry = UmlEntry::new(pred_name, value_constraint, card);
                            uml_class.add_entry(entry)
                        },
                    }
                },
                TripleExpr::Ref(_) => todo!(),
            }
            Ok(UmlComponent::class(uml_class))
        } else {
            Ok(UmlComponent::class(uml_class))
        }
    }

    fn value_expr2value_constraint(
        &mut self,
        value_expr: &ShapeExpr,
        current_node_id: &NodeId,
        current_predicate: &Name,
        current_card: &UmlCardinality,
    ) -> Result<ValueConstraint, ShEx2UmlError> {
        match value_expr {
            ShapeExpr::ShapeOr { shape_exprs } => {
                trace!(
                    "Processing ShapeOr for predicate {} with cardinality {} in node {}",
                    current_predicate.name(),
                    current_card,
                    current_node_id
                );
                if let Either::Right(refs) = all_references(shape_exprs) {
                    self.component_for_all_references(
                        current_node_id,
                        current_predicate,
                        current_card,
                        &refs,
                        &UmlLabelType::Or,
                    )?;
                    return Ok(ValueConstraint::None);
                }
                if let Either::Right(datatypes) = all_datatypes(shape_exprs, &self.current_prefixmap) {
                    return self.value_constraint_for_all_datatypes(&datatypes, &UmlLabelType::Or);
                }
                Err(ShEx2UmlError::not_implemented(
                    format!("ShapeOr has shapes_exprs which are not all references or datatypes: {shape_exprs:?}")
                        .as_str(),
                ))
            },
            ShapeExpr::ShapeAnd { shape_exprs } => {
                trace!(
                    "Processing ShapeAnd for predicate {} with cardinality {} in node {}",
                    current_predicate.name(),
                    current_card,
                    current_node_id
                );
                if let Either::Right(refs) = all_references(shape_exprs) {
                    self.component_for_all_references(
                        current_node_id,
                        current_predicate,
                        current_card,
                        &refs,
                        &UmlLabelType::And,
                    )?;
                    return Ok(ValueConstraint::None);
                }
                if let Either::Right(datatypes) = all_datatypes(shape_exprs, &self.current_prefixmap) {
                    return self.value_constraint_for_all_datatypes(&datatypes, &UmlLabelType::And);
                }
                Err(ShEx2UmlError::not_implemented(
                    format!("ShapeOr has shapes_exprs which are not all references or datatypes: {shape_exprs:?}")
                        .as_str(),
                ))
            },
            ShapeExpr::ShapeNot { shape_expr } => {
                trace!(
                    "Processing ShapeNot for predicate {} with cardinality {} in node {}",
                    current_predicate.name(),
                    current_card,
                    current_node_id
                );
                // I convert the single shape_expr to a slice of shape_exprs to reuse the all_references and all_datatypes functions.
                // I can do this because ShapeNot has only one shape_expr.
                let slice_shape_expr = std::slice::from_ref(&**shape_expr);
                if let Either::Right(refs) = all_references(slice_shape_expr) {
                    self.component_for_all_references(
                        current_node_id,
                        current_predicate,
                        current_card,
                        &refs,
                        &UmlLabelType::Not,
                    )?;
                    return Ok(ValueConstraint::None);
                }
                if let Either::Right(datatypes) = all_datatypes(slice_shape_expr, &self.current_prefixmap) {
                    self.value_constraint_for_all_datatypes(&datatypes, &UmlLabelType::Not)?;
                }
                Err(ShEx2UmlError::not_implemented(
                    format!("ShapeOr has shapes_exprs which are not all references or datatypes: {shape_expr:?}")
                        .as_str(),
                ))
            },
            ShapeExpr::NodeConstraint(nc) => {
                let maybe_nk = cnv_nodekind(nc.node_kind())?;
                let maybe_dt = cnv_datatype(nc.datatype(), &self.current_prefixmap)?;
                let maybe_facets = cnv_facets(nc.facets())?;
                let maybe_values = cnv_values(nc.values())?;
                Ok(mk_and(vec![maybe_nk, maybe_dt, maybe_facets, maybe_values]))
            },
            ShapeExpr::Shape(s) => Err(ShEx2UmlError::not_implemented(
                format!("ShapeExpr::Shape in value_expr2value_constraint not implemented yet: {s:?}").as_str(),
            )),
            ShapeExpr::External => Err(ShEx2UmlError::not_implemented(
                "ShapeExpr::External in value_expr2value_constraint not implemented yet",
            )),
            ShapeExpr::Ref(r) => match &r {
                ShapeExprLabel::IriRef { value } => {
                    let ref_name = iri_ref2name(value, &self.config, &None, &self.current_prefixmap)?;
                    let label = UmlLabel::Class(ref_name.name());
                    self.current_uml.add_link(
                        *current_node_id,
                        label,
                        current_predicate.clone(),
                        current_card.clone(),
                    )?;
                    Ok(ValueConstraint::None)
                },
                ShapeExprLabel::BNode { value } => Err(ShEx2UmlError::not_implemented(
                    format!("ShapeExprLabel::BNode in value_expr2value_constraint not implemented yet: _:{value}")
                        .as_str(),
                )),
                ShapeExprLabel::Start => Err(ShEx2UmlError::not_implemented(
                    "ShapeExprLabel::Start in value_expr2value_constraint not implemented yet",
                )),
            },
        }
    }

    /// When the value expression is a ShapeOr and all the shape expressions are references, we can create an Or component with the nodes corresponding to the references as values.
    fn component_for_all_references(
        &mut self,
        current_node_id: &NodeId,
        current_predicate: &Name,
        current_card: &UmlCardinality,
        labels: &[ShapeExprLabel],
        label_type: &UmlLabelType,
    ) -> Result<(), ShEx2UmlError> {
        let mut nodes = BTreeSet::new();
        for label in labels {
            let name = self.shape_label2name(&label)?;
            let label = UmlLabel::Class(name.name());
            let (node_id, _found) = self.current_uml.get_node_adding_label(&label);
            nodes.insert(node_id);
        }
        let component = match label_type {
            UmlLabelType::Or => UmlComponent::or(nodes.clone()),
            UmlLabelType::And => UmlComponent::and(nodes.clone()),
            UmlLabelType::Not => {
                if nodes.len() != 1 {
                    return Err(ShEx2UmlError::not_implemented(
                        "internalError: ShapeNot with more than one shape_expr",
                    ));
                }
                UmlComponent::not(nodes.clone().into_iter().next().unwrap())
            },
            _ => {
                return Err(ShEx2UmlError::not_implemented(
                    format!("internal error: component_for_all_references with label_type {label_type:?}").as_str(),
                ));
            },
        };
        let component_idx = self.current_uml.get_logical_component_idx(&nodes, &label_type);
        let component_label = UmlLabel::mk_logical_label(label_type, component_idx);
        let (component_node, _found) = self.current_uml.get_node_adding_label(&component_label);
        if self.current_uml.get_component(&component_node).is_none() {
            self.current_uml.add_component(component_node, component)?;
            for node in nodes {
                self.current_uml
                    .make_link(component_node, node, Name::new("member", None), UmlCardinality::OneOne);
            }
        }
        self.current_uml.make_link(
            current_node_id.clone(),
            component_node,
            current_predicate.clone(),
            current_card.clone(),
        );
        Ok(())
    }

    /// When the value expression is a ShapeOr and all the shape expressions are references, we can create an Or component with the nodes corresponding to the references as values.
    fn value_constraint_for_all_datatypes(
        &mut self,
        datatypes: &[Name],
        label_type: &UmlLabelType,
    ) -> Result<ValueConstraint, ShEx2UmlError> {
        let mut vcs = Vec::new();
        for datatype in datatypes {
            vcs.push(ValueConstraint::Datatype(datatype.clone()));
        }
        match label_type {
            UmlLabelType::And => Ok(ValueConstraint::and(vcs)),
            UmlLabelType::Or => Ok(ValueConstraint::or(vcs)),
            UmlLabelType::Not => {
                if vcs.len() != 1 {
                    return Err(ShEx2UmlError::not_implemented(
                        "ShapeNot with more than one shape_expr is not implemented yet",
                    ));
                }
                Ok(ValueConstraint::not(vcs.into_iter().next().unwrap()))
            },
            UmlLabelType::Class => {
                return Err(ShEx2UmlError::not_implemented(
                    "internalError: ShapeNot with a LabelType Class",
                ));
            },
        }
    }
}

fn all_references(shape_exprs: &[ShapeExprWrapper]) -> Either<(), Vec<ShapeExprLabel>> {
    let mut labels = Vec::new();
    for se in shape_exprs {
        if let ShapeExpr::Ref(r) = &se.se {
            labels.push(r.clone())
        } else {
            return Either::Left(());
        }
    }
    Either::Right(labels)
}

fn all_datatypes(shape_exprs: &[ShapeExprWrapper], prefixmap: &PrefixMap) -> Either<(), Vec<Name>> {
    let mut names = Vec::new();
    for se in shape_exprs {
        if let ShapeExpr::NodeConstraint(nc) = &se.se {
            if let Some(dt) = nc.datatype() {
                let name = iri_ref2name(&dt, &ShEx2UmlConfig::default(), &None, &prefixmap).unwrap();
                names.push(name)
            } else {
                return Either::Left(());
            }
        } else {
            return Either::Left(());
        }
    }
    Either::Right(names)
}

fn value_set2value_constraint(
    value_set: &Vec<ValueSetValue>,
    config: &ShEx2UmlConfig,
    prefixmap: &PrefixMap,
) -> Result<Vec<Name>, ShEx2UmlError> {
    let mut result = Vec::new();
    for value in value_set {
        match value {
            ValueSetValue::ObjectValue(ObjectValue::IriRef(iri)) => {
                let name = iri_ref2name(iri, config, &None, prefixmap)?;
                result.push(name)
            },
            ValueSetValue::ObjectValue(ObjectValue::Literal(lit)) => {
                return Err(ShEx2UmlError::not_implemented(
                    format!("value_set2value_constraint with literal value: {lit:?}").as_str(),
                ));
            },
            _ => {
                return Err(ShEx2UmlError::not_implemented(
                    format!("value_set2value_constraint with value: {value:?}").as_str(),
                ));
            },
        }
    }
    Ok(result)
}

fn iri_ref2name(
    iri_ref: &IriRef,
    _config: &ShEx2UmlConfig,
    maybe_label: &Option<String>,
    prefixmap: &PrefixMap,
) -> Result<Name, ShEx2UmlError> {
    let mut name = match iri_ref {
        IriRef::Iri(iri) => Name::new(prefixmap.qualify(iri).as_str(), Some(iri.as_str())),
        IriRef::Prefixed { prefix, local } => {
            let iri = prefixmap.resolve_prefix_local(prefix, local)?;
            Name::new(format!("{prefix}:{local}").as_str(), Some(iri.as_str()))
        },
    };
    if let Some(label) = maybe_label {
        name.add_label(label)
    };
    Ok(name)
}

fn mk_card(min: &Option<i32>, max: &Option<i32>) -> Result<UmlCardinality, ShEx2UmlError> {
    let min = if let Some(n) = min { *n } else { 1 };
    let max = if let Some(n) = max { *n } else { 1 };
    match (min, max) {
        (1, 1) => Ok(UmlCardinality::OneOne),
        (0, -1) => Ok(UmlCardinality::Star),
        (0, 1) => Ok(UmlCardinality::Optional),
        (1, -1) => Ok(UmlCardinality::Plus),
        (m, n) if m >= 0 && n > m => Ok(UmlCardinality::Fixed(m)),
        (m, n) if m >= 0 && n > m => Ok(UmlCardinality::Range(m, n)),
        _ => Err(ShEx2UmlError::WrongCardinality { min, max }),
    }
}

fn mk_name(
    iri: &IriRef,
    annotations: &Option<Vec<Annotation>>,
    config: &ShEx2UmlConfig,
    prefixmap: &PrefixMap,
) -> Result<Name, ShEx2UmlError> {
    let label = get_label(annotations, prefixmap, config)?;
    let name = iri_ref2name(iri, config, &label, prefixmap)?;
    Ok(name)
}

fn get_label(
    annotations: &Option<Vec<Annotation>>,
    prefixmap: &PrefixMap,
    config: &ShEx2UmlConfig,
) -> Result<Option<String>, PrefixMapError> {
    for label in config.annotation_label.iter() {
        if let Some(value) = find_annotation(annotations, label, prefixmap)? {
            return Ok(Some(object_value2string(&value)));
        }
    }
    Ok(None)
}

fn cnv_nodekind(maybe_nk: Option<NodeKind>) -> Result<Option<ValueConstraint>, ShEx2UmlError> {
    if let Some(nk) = maybe_nk {
        let name = match nk {
            NodeKind::Iri => Name::new("iri", None),
            NodeKind::BNode => Name::new("bnode", None),
            NodeKind::NonLiteral => Name::new("nonliteral", None),
            NodeKind::Literal => Name::new("literal", None),
        };
        Ok(Some(ValueConstraint::Kind(name)))
    } else {
        Ok(None)
    }
}

fn cnv_datatype(maybe_dt: Option<IriRef>, prefixmap: &PrefixMap) -> Result<Option<ValueConstraint>, ShEx2UmlError> {
    if let Some(dt) = maybe_dt {
        let name = iri_ref2name(&dt, &ShEx2UmlConfig::default(), &None, prefixmap)?;
        Ok(Some(ValueConstraint::datatype(name)))
    } else {
        Ok(None)
    }
}

fn cnv_facets(maybe_facets: Option<Vec<XsFacet>>) -> Result<Option<ValueConstraint>, ShEx2UmlError> {
    if let Some(facets) = maybe_facets {
        let mut facet_names = Vec::new();
        for facet in facets {
            let name = facet2name(&facet, &mut facet_names)?;
            facet_names.push(name);
        }
        Ok(Some(ValueConstraint::Facet(facet_names)))
    } else {
        Ok(None)
    }
}

fn cnv_values(maybe_values: Option<Vec<ValueSetValue>>) -> Result<Option<ValueConstraint>, ShEx2UmlError> {
    if let Some(values) = maybe_values {
        let value_set_constraint = value_set2value_constraint(&values, &ShEx2UmlConfig::default(), &PrefixMap::new())?;
        Ok(Some(ValueConstraint::ValueSet(value_set_constraint)))
    } else {
        Ok(None)
    }
}

impl UmlConverter for ShEx2Uml {
    fn as_plantuml<W: Write>(&self, writer: &mut W, mode: &UmlGenerationMode) -> Result<(), UmlConverterError> {
        self.as_plantuml(writer, mode)
    }
}

fn facet2name(facet: &XsFacet, _names: &mut [Name]) -> Result<Name, ShEx2UmlError> {
    match facet {
        XsFacet::StringFacet(sf) => string_facet2name(sf),
        XsFacet::NumericFacet(nf) => numeric_facet2name(nf),
    }
}

fn string_facet2name(facet: &shex_ast::ast::xs_facet::StringFacet) -> Result<Name, ShEx2UmlError> {
    match facet {
        shex_ast::ast::xs_facet::StringFacet::Length(n) => Ok(Name::new(format!("length={n}").as_str(), None)),
        shex_ast::ast::xs_facet::StringFacet::MinLength(n) => Ok(Name::new(format!("minLength={n}").as_str(), None)),
        shex_ast::ast::xs_facet::StringFacet::MaxLength(n) => Ok(Name::new(format!("maxLength={n}").as_str(), None)),
        shex_ast::ast::xs_facet::StringFacet::Pattern(p) => {
            if let Some(flags) = &p.flags {
                Ok(Name::new(format!("pattern={} (flags={})", p.str, flags).as_str(), None))
            } else {
                Ok(Name::new(format!("pattern={}", p.str).as_str(), None))
            }
        },
    }
}

fn numeric_facet2name(facet: &shex_ast::ast::xs_facet::NumericFacet) -> Result<Name, ShEx2UmlError> {
    match facet {
        shex_ast::ast::xs_facet::NumericFacet::MinInclusive(n) => {
            Ok(Name::new(format!("minInclusive={}", n).as_str(), None))
        },
        shex_ast::ast::xs_facet::NumericFacet::MinExclusive(n) => {
            Ok(Name::new(format!("minExclusive={}", n).as_str(), None))
        },
        shex_ast::ast::xs_facet::NumericFacet::MaxInclusive(n) => {
            Ok(Name::new(format!("maxInclusive={}", n).as_str(), None))
        },
        shex_ast::ast::xs_facet::NumericFacet::MaxExclusive(n) => {
            Ok(Name::new(format!("maxExclusive={}", n).as_str(), None))
        },
        shex_ast::ast::xs_facet::NumericFacet::TotalDigits(n) => {
            Ok(Name::new(format!("totalDigits={}", n).as_str(), None))
        },
        shex_ast::ast::xs_facet::NumericFacet::FractionDigits(n) => {
            Ok(Name::new(format!("fractionDigits={}", n).as_str(), None))
        },
    }
}

fn mk_and(values: Vec<Option<ValueConstraint>>) -> ValueConstraint {
    let mut vcs = Vec::new();
    for vc in values.into_iter().flatten() {
        vcs.push(vc);
    }
    if vcs.is_empty() {
        ValueConstraint::Any
    } else if vcs.len() == 1 {
        vcs.into_iter().next().unwrap()
    } else {
        ValueConstraint::And { values: vcs }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use iri_s::iri;
    use shex_ast::ShExParser;
    use tracing_test::traced_test;

    #[test]
    fn test_simple() {
        let shex_str = "\
    prefix : <http://example.org/>
    prefix xsd: <http://www.w3.org/2001/XMLSchema#>

    :Person {
      :name xsd:string ;
    }";

        let mut expected_uml = Uml::new();

        let mut person = UmlClass::new(Name::new(":Person", Some("http://example.org/Person")));
        person.add_entry(UmlEntry::new(
            Name::new(":name", Some("http://example.org/name")),
            ValueConstraint::datatype(Name::new("xsd:string", Some("http://www.w3.org/2001/XMLSchema#string"))),
            UmlCardinality::OneOne,
        ));
        let (person_node, _found) = expected_uml.get_node_adding_label(&UmlLabel::Class(":Person".to_string()));
        expected_uml
            .add_component(person_node, UmlComponent::class(person))
            .unwrap();
        let shex = ShExParser::parse(shex_str, None, &iri!("http://example.org/")).unwrap();
        let mut converter = ShEx2Uml::new(&ShEx2UmlConfig::default());
        converter.convert(&shex).unwrap();
        let converted_uml = converter.current_uml;
        assert_eq!(converted_uml, expected_uml);
    }

    #[test]
    fn test_simple_recursive() {
        let shex_str = "\
    prefix : <http://example.org/>
    prefix xsd: <http://www.w3.org/2001/XMLSchema#>

    :Person {
      :name xsd:string  ;
      :knows @:Person * ;
    }";

        let mut expected_uml = Uml::new();

        let mut person = UmlClass::new(Name::new(":Person", Some("http://example.org/Person")));
        person.add_entry(UmlEntry::new(
            Name::new(":name", Some("http://example.org/name")),
            ValueConstraint::datatype(Name::new("xsd:string", Some("http://www.w3.org/2001/XMLSchema#string"))),
            UmlCardinality::OneOne,
        ));
        let (person_node, _found) = expected_uml.get_node_adding_label(&UmlLabel::Class(":Person".to_string()));
        expected_uml
            .add_component(person_node, UmlComponent::class(person))
            .unwrap();
        expected_uml.make_link(
            person_node,
            person_node,
            Name::new(":knows", Some("http://example.org/knows")),
            UmlCardinality::Star,
        );
        let shex = ShExParser::parse(shex_str, None, &iri!("http://example.org/")).unwrap();
        let mut converter = ShEx2Uml::new(&ShEx2UmlConfig::default());
        converter.convert(&shex).unwrap();
        let converted_uml = converter.current_uml;
        assert_eq!(converted_uml, expected_uml);
    }

    #[test]
    fn test_two_shapes() {
        let shex_str = "\
    prefix : <http://example.org/>
    prefix xsd: <http://www.w3.org/2001/XMLSchema#>

    :Person {
      :name xsd:string  ;
      :worksFor @:Company * ;
    }
    
    :Company {
      :name xsd:string ;
    }";

        let mut expected_uml = Uml::new();

        let mut person = UmlClass::new(Name::new(":Person", Some("http://example.org/Person")));
        person.add_entry(UmlEntry::new(
            Name::new(":name", Some("http://example.org/name")),
            ValueConstraint::datatype(Name::new("xsd:string", Some("http://www.w3.org/2001/XMLSchema#string"))),
            UmlCardinality::OneOne,
        ));
        let mut company = UmlClass::new(Name::new(":Company", Some("http://example.org/Company")));
        company.add_entry(UmlEntry::new(
            Name::new(":name", Some("http://example.org/name")),
            ValueConstraint::datatype(Name::new("xsd:string", Some("http://www.w3.org/2001/XMLSchema#string"))),
            UmlCardinality::OneOne,
        ));
        let (person_node, _found) = expected_uml.get_node_adding_label(&UmlLabel::Class(":Person".to_string()));
        let (company_node, _found) = expected_uml.get_node_adding_label(&UmlLabel::Class(":Company".to_string()));

        expected_uml
            .add_component(person_node, UmlComponent::class(person))
            .unwrap();
        expected_uml
            .add_component(company_node, UmlComponent::class(company))
            .unwrap();

        expected_uml.make_link(
            person_node,
            company_node,
            Name::new(":worksFor", Some("http://example.org/worksFor")),
            UmlCardinality::Star,
        );
        let shex = ShExParser::parse(shex_str, None, &iri!("http://example.org/")).unwrap();
        let mut converter = ShEx2Uml::new(&ShEx2UmlConfig::default());
        converter.convert(&shex).unwrap();
        let converted_uml = converter.current_uml;
        assert_eq!(converted_uml, expected_uml);
    }

    #[test]
    fn test_disjunction_datatypes() {
        let shex_str = "\
    prefix : <http://example.org/>
    prefix xsd: <http://www.w3.org/2001/XMLSchema#>

    :A {
      :code xsd:string OR xsd:integer ;
    }";

        let mut expected_uml = Uml::new();

        let mut a = UmlClass::new(Name::new(":A", Some("http://example.org/A")));
        a.add_entry(UmlEntry::new(
            Name::new(":code", Some("http://example.org/code")),
            ValueConstraint::Or {
                values: vec![
                    ValueConstraint::datatype(Name::new("xsd:string", Some("http://www.w3.org/2001/XMLSchema#string"))),
                    ValueConstraint::datatype(Name::new(
                        "xsd:integer",
                        Some("http://www.w3.org/2001/XMLSchema#integer"),
                    )),
                ],
            },
            UmlCardinality::OneOne,
        ));
        let (a_node, _found) = expected_uml.get_node_adding_label(&UmlLabel::Class(":A".to_string()));

        expected_uml.add_component(a_node, UmlComponent::class(a)).unwrap();

        let shex = ShExParser::parse(shex_str, None, &iri!("http://example.org/")).unwrap();
        let mut converter = ShEx2Uml::new(&ShEx2UmlConfig::default());
        converter.convert(&shex).unwrap();
        let converted_uml = converter.current_uml;
        assert_eq!(converted_uml, expected_uml);
    }

    #[traced_test]
    #[test]
    fn test_disjunction_shapes() {
        let shex_str = "\
    prefix : <http://example.org/>
    prefix xsd: <http://www.w3.org/2001/XMLSchema#>

    :A {
      :code @:A OR @:B ;
    }";

        let mut expected_uml = Uml::new();

        let a = UmlClass::new(Name::new(":A", Some("http://example.org/A")));
        let (a_node, _found) = expected_uml.get_node_adding_label(&UmlLabel::Class(":A".to_string()));
        let b = UmlClass::new(Name::new(":B", Some("http://example.org/B")));
        let (b_node, _found) = expected_uml.get_node_adding_label(&UmlLabel::Class(":B".to_string()));
        let c = UmlClass::new(Name::new(":C", Some("http://example.org/C")));
        let (c_node, _found) = expected_uml.get_node_adding_label(&UmlLabel::Class(":C".to_string()));
        let ors = BTreeSet::from([b_node, c_node]);
        let or = UmlComponent::or(ors.clone());
        let or_idx = expected_uml.get_logical_component_idx(&ors, &UmlLabelType::Or);
        let (or_node, _found) = expected_uml.get_node_adding_label(&UmlLabel::Or(or_idx));
        expected_uml.add_component(a_node, UmlComponent::class(a)).unwrap();
        expected_uml.add_component(b_node, UmlComponent::class(b)).unwrap();
        expected_uml.add_component(c_node, UmlComponent::class(c)).unwrap();
        expected_uml.add_component(or_node, or).unwrap();
        expected_uml.make_link(
            a_node,
            or_node,
            Name::new(":code", Some("http://example.org/code")),
            UmlCardinality::OneOne,
        );
        expected_uml.make_link(or_node, b_node, Name::new("", None), UmlCardinality::OneOne);
        expected_uml.make_link(or_node, c_node, Name::new("", None), UmlCardinality::OneOne);

        let shex = ShExParser::parse(shex_str, None, &iri!("http://example.org/")).unwrap();
        let mut converter = ShEx2Uml::new(&ShEx2UmlConfig::default());
        converter.convert(&shex).unwrap();
        let converted_uml = converter.current_uml;
        assert_eq!(converted_uml, expected_uml);
    }
}
