use std::{
    fs::File,
    io::{self, Write},
    process::Command,
};

use prefixmap::{IriRef, PrefixMap, PrefixMapError};
use shex_ast::{ObjectValue, Schema, Shape, ShapeExpr, ShapeExprLabel, TripleExpr};
use srdf::literal::Literal;
use tracing::debug;

use crate::shex_to_uml::{ShEx2UmlConfig, ShEx2UmlError, Uml};

use super::{
    Name, NodeId, UmlCardinality, UmlClass, UmlComponent, UmlEntry, ValueConstraint, PLANTUML,
};
use tempfile::TempDir;

pub struct ShEx2Uml {
    config: ShEx2UmlConfig,
    current_uml: Uml,
}

impl ShEx2Uml {
    pub fn new(config: &ShEx2UmlConfig) -> ShEx2Uml {
        ShEx2Uml {
            config: config.clone(),
            current_uml: Uml::new(),
        }
    }

    pub fn as_plantuml<W: Write>(&self, writer: &mut W) -> Result<(), ShEx2UmlError> {
        self.current_uml.as_plantuml(writer)?;
        Ok(())
    }

    pub fn as_image<W: Write>(
        &self,
        writer: &mut W,
        image_format: ImageFormat,
    ) -> Result<(), ShEx2UmlError> {
        let tempdir = TempDir::new().map_err(|e| ShEx2UmlError::TempFileError { err: e })?;
        let tempdir_path = tempdir.path();
        let tempfile_path = tempdir_path.join("temp.uml");
        let tempfile_name = tempfile_path.display().to_string();
        let mut tempfile =
            File::create(tempfile_path).map_err(|e| ShEx2UmlError::CreatingTempUMLFile {
                tempfile_name: tempfile_name.clone(),
                error: e,
            })?;
        self.current_uml.as_plantuml(&mut tempfile)?;
        debug!("ShEx contents stored in temporary file:{}", tempfile_name);

        let (out_param, out_file_name) = match image_format {
            ImageFormat::PNG => ("-png", tempdir_path.join("temp.png")),
            ImageFormat::SVG => ("-svg", tempdir_path.join("temp.svg")),
        };
        if let Some(plantuml_path) = &self.config.plantuml_path {
            let mut command = Command::new("java");
            command
                .arg("-jar")
                .arg(plantuml_path)
                .arg("-o")
                .arg(tempdir_path.to_string_lossy().to_string())
                .arg(out_param)
                .arg(tempfile_name);
            let command_name = format!("{:?}", &command);
            debug!("PLANTUML COMMAND:\n{command_name}");
            let result = command.output();
            match result {
                Ok(_) => {
                    let mut temp_file = File::open(out_file_name.as_path()).map_err(|e| {
                        ShEx2UmlError::CantOpenGeneratedTempFile {
                            generated_name: out_file_name.display().to_string(),
                            error: e,
                        }
                    })?;
                    copy(&mut temp_file, writer).map_err(|e| ShEx2UmlError::CopyingTempFile {
                        temp_name: out_file_name.display().to_string(),
                        error: e,
                    })?;
                    Ok(())
                }
                Err(e) => Err(ShEx2UmlError::PlantUMLCommandError {
                    command: command_name,
                    error: e,
                }),
            }
        } else {
            Err(ShEx2UmlError::NoPlantUMLPath {
                env_name: PLANTUML.to_string(),
            })
        }
    }

    pub fn convert(&mut self, shex: &Schema) -> Result<(), ShEx2UmlError> {
        let prefixmap = shex.prefixmap().unwrap_or_default();
        if let Some(shapes) = shex.shapes() {
            for shape_decl in shapes {
                let name = self.shape_label2name(&shape_decl.id, &prefixmap)?;
                let node_id = self.current_uml.add_label(&name);
                let component =
                    self.shape_expr2component(&name, &shape_decl.shape_expr, &prefixmap, &node_id)?;
                self.current_uml.add_component(node_id, component)?;
            }
        }
        Ok(())
    }

    fn shape_label2name(
        &mut self,
        label: &ShapeExprLabel,
        prefixmap: &PrefixMap,
    ) -> Result<Name, ShEx2UmlError> {
        match label {
            ShapeExprLabel::IriRef { value } => iri_ref2name(value, &self.config, prefixmap),
            ShapeExprLabel::BNode { value: _ } => todo!(),
            ShapeExprLabel::Start => todo!(),
        }
    }

    fn shape_expr2component(
        &mut self,
        name: &Name,
        shape_expr: &ShapeExpr,
        prefixmap: &PrefixMap,
        current_node_id: &NodeId,
    ) -> Result<UmlComponent, ShEx2UmlError> {
        match shape_expr {
            ShapeExpr::Shape(shape) => {
                self.shape2component(name, shape, prefixmap, current_node_id)
            }
            _ => Err(ShEx2UmlError::NotImplemented {
                msg: "Complex shape expressions are not implemented yet".to_string(),
            }),
        }
    }

    fn shape2component(
        &mut self,
        name: &Name,
        shape: &Shape,
        prefixmap: &PrefixMap,
        current_node_id: &NodeId,
    ) -> Result<UmlComponent, ShEx2UmlError> {
        let mut name = Name::new(name.name().as_str(), None);
        if let Some(label) = get_label(shape, prefixmap, &self.config)? {
            name.add_href(label.as_str())
        }
        let mut uml_class = UmlClass::new(name.clone());
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
                                annotations: _,
                            } => {
                                let pred_name = iri_ref2name(predicate, &self.config, prefixmap)?;
                                let card = mk_card(min, max)?;
                                let value_constraint = if let Some(se) = value_expr {
                                    self.value_expr2value_constraint(
                                        se,
                                        prefixmap,
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
                    annotations: _,
                } => {
                    let pred_name = iri_ref2name(predicate, &self.config, prefixmap)?;
                    let card = mk_card(min, max)?;
                    let value_constraint = if let Some(se) = value_expr {
                        self.value_expr2value_constraint(
                            se,
                            prefixmap,
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
        prefixmap: &PrefixMap,
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
                    let name = iri_ref2name(&datatype, &self.config, prefixmap)?;
                    Ok(ValueConstraint::datatype(name))
                } else {
                    todo!()
                }
            }
            ShapeExpr::Shape(_) => todo!(),
            ShapeExpr::External => todo!(),
            ShapeExpr::Ref(r) => match &r {
                ShapeExprLabel::IriRef { value } => {
                    let ref_name = iri_ref2name(value, &self.config, prefixmap)?;
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
            }, /*
               // TODO: If we want to embed some references...
               match &r {
                   ShapeExprLabel::IriRef { value } => {
                       let name = iri_ref2name(value, config, prefixmap)?;
                       Ok(ValueConstraint::Ref(name))
                   }
                   ShapeExprLabel::BNode { value: _ } => todo!(),
                   ShapeExprLabel::Start => todo!(),
               }*/
        }
    }
}

fn iri_ref2name(
    iri_ref: &IriRef,
    _config: &ShEx2UmlConfig,
    prefixmap: &PrefixMap,
) -> Result<Name, ShEx2UmlError> {
    match iri_ref {
        IriRef::Iri(iri) => Ok(Name::new(
            prefixmap.qualify(iri).as_str(),
            Some(iri.as_str()),
        )),
        IriRef::Prefixed { prefix: _, local } => {
            // TODO: Check if we could replace href as None by a proper IRI
            Ok(Name::new(local, None))
        }
    }
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

fn copy<W: Write>(file: &mut File, writer: &mut W) -> Result<(), io::Error> {
    io::copy(file, writer)?;
    Ok(())
}

fn get_label(
    shape: &Shape,
    prefixmap: &PrefixMap,
    config: &ShEx2UmlConfig,
) -> Result<Option<String>, PrefixMapError> {
    for label in config.annotation_label.iter() {
        if let Some(value) = shape.find_annotation(label, prefixmap)? {
            return Ok(Some(object_value2string(&value)));
        }
    }
    Ok(None)
}

fn object_value2string(object_value: &ObjectValue) -> String {
    match object_value {
        ObjectValue::IriRef(_) => todo!(),
        ObjectValue::Literal(lit) => lit_2string(lit),
    }
}

fn lit_2string(lit: &Literal) -> String {
    lit.lexical_form()
}

pub enum ImageFormat {
    SVG,
    PNG,
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
