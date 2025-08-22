use std::io::Write;

use prefixmap::{IriRef, PrefixMap, PrefixMapError};
use shex_ast::{
    Annotation, ObjectValue, Schema, Shape, ShapeExpr, ShapeExprLabel, TripleExpr, ValueSetValue,
};
use srdf::{UmlConverter, UmlConverterError, UmlGenerationMode};

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

    pub fn as_plantuml<W: Write>(
        &self,
        writer: &mut W,
        mode: &UmlGenerationMode,
    ) -> Result<(), UmlConverterError> {
        match mode {
            UmlGenerationMode::AllNodes => {
                self.current_uml
                    .as_plantuml_all(&self.config, writer)
                    .map_err(|e| UmlConverterError::UmlError {
                        error: e.to_string(),
                    })?;
                Ok(())
            }
            UmlGenerationMode::Neighs(str) => {
                if let Some(node_id) = self.current_uml.get_node(str) {
                    self.current_uml
                        .as_plantuml_neighs(&self.config, writer, &node_id)
                        .map_err(|e| UmlConverterError::UmlError {
                            error: e.to_string(),
                        })?;
                    Ok(())
                } else {
                    Err(UmlConverterError::NotFoundLabel { name: str.clone() })
                }
            }
        }
    }

    pub fn convert(&mut self, shex: &Schema) -> Result<(), ShEx2UmlError> {
        self.current_prefixmap = shex.prefixmap().unwrap_or_default();
        if let Some(shapes) = shex.shapes() {
            for shape_decl in shapes {
                let mut name = self.shape_label2name(&shape_decl.id)?;
                let (node_id, _found) = self.current_uml.get_node_adding_label(&name.name());
                let component =
                    self.shape_expr2component(&mut name, &shape_decl.shape_expr, &node_id)?;
                self.current_uml.update_component(node_id, component)?;
            }
        }
        Ok(())
    }

    fn shape_label2name(&self, label: &ShapeExprLabel) -> Result<Name, ShEx2UmlError> {
        match label {
            ShapeExprLabel::IriRef { value } => {
                iri_ref2name(value, &self.config, &None, &self.current_prefixmap)
            }
            ShapeExprLabel::BNode { value: _ } => todo!(),
            ShapeExprLabel::Start => todo!(),
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
            _ => Err(ShEx2UmlError::NotImplemented {
                msg: "Complex shape expressions are not implemented yet".to_string(),
            }),
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
                let (extended_node, found) = self
                    .current_uml
                    .get_node_adding_label(&extended_name.name());
                self.current_uml
                    .add_extends(current_node_id, &extended_node);
                uml_class.add_extends(&extended_node);
                if !found {
                    self.current_uml.add_component(
                        extended_node,
                        UmlComponent::class(UmlClass::new(extended_name)),
                    )?;
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
                                let pred_name = mk_name(
                                    predicate,
                                    annotations,
                                    &self.config,
                                    &self.current_prefixmap,
                                )?;
                                let card = mk_card(min, max)?;
                                let value_constraint = if let Some(se) = value_expr {
                                    self.value_expr2value_constraint(
                                        se,
                                        current_node_id,
                                        &pred_name,
                                        &card,
                                    )?
                                } else {
                                    ValueConstraint::default()
                                };
                                match value_constraint {
                                    ValueConstraint::None => {}
                                    _ => {
                                        let entry =
                                            UmlEntry::new(pred_name, value_constraint, card);
                                        uml_class.add_entry(entry)
                                    }
                                }
                            }
                            _ => todo!(),
                        }
                    }
                }
                TripleExpr::OneOf {
                    id: _,
                    expressions: _,
                    min: _,
                    max: _,
                    sem_acts: _,
                    annotations: _,
                } => todo!(),
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
                    let pred_name = mk_name(
                        predicate,
                        annotations,
                        &self.config,
                        &self.current_prefixmap,
                    )?;
                    let card = mk_card(min, max)?;
                    let value_constraint = if let Some(se) = value_expr {
                        self.value_expr2value_constraint(se, current_node_id, &pred_name, &card)?
                    } else {
                        ValueConstraint::default()
                    };
                    match value_constraint {
                        ValueConstraint::None => {}
                        _ => {
                            let entry = UmlEntry::new(pred_name, value_constraint, card);
                            uml_class.add_entry(entry)
                        }
                    }
                }
                TripleExpr::TripleExprRef(_) => todo!(),
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
            ShapeExpr::ShapeOr { shape_exprs: _ } => todo!(),
            ShapeExpr::ShapeAnd { shape_exprs: _ } => todo!(),
            ShapeExpr::ShapeNot { shape_expr: _ } => todo!(),
            ShapeExpr::NodeConstraint(nc) => {
                if let Some(datatype) = nc.datatype() {
                    let name =
                        iri_ref2name(&datatype, &self.config, &None, &self.current_prefixmap)?;
                    Ok(ValueConstraint::datatype(name))
                } else if let Some(value_set) = nc.values() {
                    let value_set_constraint = value_set2value_constraint(
                        &value_set,
                        &self.config,
                        &self.current_prefixmap,
                    )?;
                    Ok(ValueConstraint::ValueSet(value_set_constraint))
                } else {
                    todo!()
                }
            }
            ShapeExpr::Shape(_) => todo!(),
            ShapeExpr::External => todo!(),
            ShapeExpr::Ref(r) => match &r {
                ShapeExprLabel::IriRef { value } => {
                    let ref_name =
                        iri_ref2name(value, &self.config, &None, &self.current_prefixmap)?;
                    self.current_uml.add_link(
                        *current_node_id,
                        ref_name,
                        current_predicate.clone(),
                        current_card.clone(),
                    )?;
                    Ok(ValueConstraint::None)
                }
                ShapeExprLabel::BNode { value: _ } => todo!(),
                ShapeExprLabel::Start => todo!(),
            },
        }
    }
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
            }
            ValueSetValue::ObjectValue(ObjectValue::Literal(lit)) => {
                return Err(ShEx2UmlError::not_implemented(
                    format!("value_set2value_constraint with literal value: {lit:?}").as_str(),
                ));
            }
            _ => {
                return Err(ShEx2UmlError::not_implemented(
                    format!("value_set2value_constraint with value: {value:?}").as_str(),
                ));
            }
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
        }
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

impl UmlConverter for ShEx2Uml {
    fn as_plantuml<W: Write>(
        &self,
        writer: &mut W,
        mode: &UmlGenerationMode,
    ) -> Result<(), UmlConverterError> {
        self.as_plantuml(writer, mode)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use shex_compact::ShExParser;

    /*    #[test]
    fn test_simple() {
        let shex_str = "\
    prefix : <http://example.org/>
    prefix xsd: <http://www.w3.org/2001/XMLSchema#>

    :Person {
      :name xsd:string ;
      :knows @:Person  ;
      :works_for @:Course * ;
    }

    :Course {
      :name xsd:string
    }";
        let mut expected_uml = Uml::new();
        expected_uml.add_label(Name::new(":Person", Some("http://example.org/Person")));
        expected_uml.add_label(Name::new(":Course", Some("http://example.org/Course")));
        let shex = ShExParser::parse(shex_str, None).unwrap();
        let converter = ShEx2Uml::new(ShEx2UmlConfig::default());
        let converted_uml = converter.convert(&shex).unwrap();
        assert_eq!(converted_uml, expected_uml);
    } */
}
