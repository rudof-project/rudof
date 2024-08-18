use std::fs::OpenOptions;

use crate::{find_annotation, object_value2string, ShEx2HtmlError};
use minijinja::Template;
use minijinja::{path_loader, Environment};
use prefixmap::{IriRef, PrefixMap, PrefixMapError};
use shex_ast::{Annotation, Schema, Shape, ShapeExpr, ShapeExprLabel, TripleExpr};
use tracing::debug;

use super::{
    Cardinality, Entry, HtmlSchema, HtmlShape, Name, NodeId, ShEx2HtmlConfig, ValueConstraint,
};

pub struct ShEx2Html {
    config: ShEx2HtmlConfig,
    current_html: HtmlSchema,
}

impl ShEx2Html {
    pub fn new(config: ShEx2HtmlConfig) -> ShEx2Html {
        ShEx2Html {
            config,
            current_html: HtmlSchema::new(),
        }
    }

    pub fn current_html(&self) -> &HtmlSchema {
        &self.current_html
    }

    pub fn convert(&mut self, shex: &Schema) -> Result<(), ShEx2HtmlError> {
        let prefixmap = shex
            .prefixmap()
            .unwrap_or_default()
            .without_rich_qualifying();
        if let Some(shapes) = shex.shapes() {
            for shape_decl in shapes {
                let mut name = self.shape_label2name(&shape_decl.id, &prefixmap)?;
                let (node_id, _found) = self.current_html.get_node_adding_label(&name.name());
                let component = self.shape_expr2htmlshape(
                    &mut name,
                    &shape_decl.shape_expr,
                    &prefixmap,
                    &node_id,
                )?;
                self.current_html.add_component(node_id, component)?;
            }
        }
        Ok(())
    }

    pub fn export_schema(&self) -> Result<(), ShEx2HtmlError> {
        let environment = create_env();
        let landing_page = self.config.landing_page();
        let template = environment.get_template(self.config.landing_page_name.as_str())?;
        let landing_page_name = landing_page.to_string_lossy().to_string();
        let out = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(landing_page)
            .map_err(|e| ShEx2HtmlError::ErrorCreatingLandingPage {
                name: landing_page_name,
                error: e,
            })?;
        let _state = template
            .render_to_write(self.current_html.to_landing_html_schema(&self.config), out)?;

        let shape_template = environment.get_template(self.config.shape_template_name.as_str())?;
        for shape in self.current_html.shapes() {
            generate_shape_page(shape, &shape_template, &self.config)?;
        }
        Ok(())
    }

    fn shape_label2name(
        &mut self,
        label: &ShapeExprLabel,
        prefixmap: &PrefixMap,
    ) -> Result<Name, ShEx2HtmlError> {
        match label {
            ShapeExprLabel::IriRef { value } => iri_ref2name(value, &self.config, &None, prefixmap),
            ShapeExprLabel::BNode { value: _ } => todo!(),
            ShapeExprLabel::Start => todo!(),
        }
    }

    fn shape_expr2htmlshape(
        &mut self,
        name: &mut Name,
        shape_expr: &ShapeExpr,
        prefixmap: &PrefixMap,
        current_node_id: &NodeId,
    ) -> Result<HtmlShape, ShEx2HtmlError> {
        match shape_expr {
            ShapeExpr::Shape(shape) => {
                self.shape2htmlshape(name, shape, prefixmap, current_node_id)
            }
            _ => Err(ShEx2HtmlError::NotImplemented {
                msg: "Complex shape expressions are not implemented yet".to_string(),
            }),
        }
    }

    fn shape2htmlshape(
        &mut self,
        name: &mut Name,
        shape: &Shape,
        prefixmap: &PrefixMap,
        current_node_id: &NodeId,
    ) -> Result<HtmlShape, ShEx2HtmlError> {
        if let Some(label) = get_label(&shape.annotations, prefixmap, &self.config)? {
            name.add_label(label.as_str())
        }
        let mut html_shape = HtmlShape::new(name.clone());
        if let Some(extends) = &shape.extends {
            for e in extends.iter() {
                let extended_name = self.shape_label2name(e, prefixmap)?;
                let (extended_node, found) = self
                    .current_html
                    .get_node_adding_label(&extended_name.name());
                html_shape.add_extends(&extended_name);
                if !found {
                    self.current_html
                        .add_component(extended_node, HtmlShape::new(extended_name))?;
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
                                let pred_name =
                                    mk_name(predicate, annotations, &self.config, prefixmap)?;
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
                                        let entry = Entry::new(pred_name, value_constraint, card);
                                        html_shape.add_entry(entry)
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
                    let pred_name = mk_name(predicate, annotations, &self.config, prefixmap)?;
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
                            let entry = Entry::new(pred_name, value_constraint, card);
                            html_shape.add_entry(entry)
                        }
                    }
                }
                TripleExpr::TripleExprRef(_) => todo!(),
            }
            Ok(html_shape)
        } else {
            Ok(html_shape)
        }
    }

    fn value_expr2value_constraint(
        &mut self,
        value_expr: &ShapeExpr,
        prefixmap: &PrefixMap,
        _current_node_id: &NodeId,
        _current_predicate: &Name,
        _current_card: &Cardinality,
    ) -> Result<ValueConstraint, ShEx2HtmlError> {
        match value_expr {
            ShapeExpr::ShapeOr { shape_exprs: _ } => todo!(),
            ShapeExpr::ShapeAnd { shape_exprs: _ } => todo!(),
            ShapeExpr::ShapeNot { shape_expr: _ } => todo!(),
            ShapeExpr::NodeConstraint(nc) => {
                if let Some(datatype) = nc.datatype() {
                    let name = iri_ref2name(&datatype, &self.config, &None, prefixmap)?;
                    Ok(ValueConstraint::datatype(name))
                } else {
                    todo!()
                }
            }
            ShapeExpr::Shape(_) => todo!(),
            ShapeExpr::External => todo!(),
            ShapeExpr::Ref(r) => match &r {
                ShapeExprLabel::IriRef { value } => {
                    let ref_name = iri_ref2name(value, &self.config, &None, prefixmap)?;
                    let (node, found) = self
                        .current_html
                        .get_node_adding_label(ref_name.name().as_str());
                    if !found {
                        self.current_html
                            .add_component(node, HtmlShape::new(ref_name.clone()))?
                    }

                    Ok(ValueConstraint::Ref(ref_name))
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
    config: &ShEx2HtmlConfig,
    maybe_label: &Option<String>,
    prefixmap: &PrefixMap,
) -> Result<Name, ShEx2HtmlError> {
    let mut name = match iri_ref {
        IriRef::Iri(iri) => Name::new(
            prefixmap.qualify(iri).as_str(),
            Some(iri.as_str()),
            config.target_folder().as_path(),
        ),
        IriRef::Prefixed { prefix, local } => {
            let iri = prefixmap.resolve_prefix_local(prefix, local)?;
            Name::new(
                format!("{prefix}:{local}").as_str(),
                Some(iri.as_str()),
                config.target_folder().as_path(),
            )
        }
    };
    if let Some(label) = maybe_label {
        name.add_label(label)
    };
    Ok(name)
}

pub fn create_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(path_loader("shapes_converter/default_templates"));
    env
}

fn mk_card(min: &Option<i32>, max: &Option<i32>) -> Result<Cardinality, ShEx2HtmlError> {
    let min = if let Some(n) = min { *n } else { 1 };
    let max = if let Some(n) = max { *n } else { 1 };
    match (min, max) {
        (1, 1) => Ok(Cardinality::OneOne),
        (0, -1) => Ok(Cardinality::Star),
        (0, 1) => Ok(Cardinality::Optional),
        (1, -1) => Ok(Cardinality::Plus),
        (m, n) if m >= 0 && n > m => Ok(Cardinality::Fixed(m)),
        (m, n) if m >= 0 && n > m => Ok(Cardinality::Range(m, n)),
        _ => Err(ShEx2HtmlError::WrongCardinality { min, max }),
    }
}

/*
fn generate_css_file(name: &str, config: &ShEx2HtmlConfig) -> Result<(), ShEx2HtmlError> {
    let css_path = config.target_folder.join(name);
    let css_path_name = css_path.display().to_string();
    debug!("Generating css file: {css_path_name}");
    let mut out_css = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(css_path)
        .map_err(|e| ShEx2HtmlError::ErrorCreatingLandingPage {
            name: css_path_name,
            error: e,
        })?;
    if let Some(color) = &config.color_property_name {
        writeln!(out_css, ".property_name {{ {color} }}")?;
    }
    Ok(())
}*/

fn generate_shape_page(
    shape: &HtmlShape,
    template: &Template,
    _config: &ShEx2HtmlConfig,
) -> Result<(), ShEx2HtmlError> {
    let name = shape.name();
    if let Some((path, _local_name)) = name.get_path_localname() {
        let file_name = path.as_path().display().to_string();
        let out_shape = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .map_err(|e| ShEx2HtmlError::ErrorCreatingShapesFile {
                name: file_name,
                error: e,
            })?;
        let state = template.render_to_write(shape, out_shape)?;
        debug!("Generated state: {state:?}");
        Ok(())
    } else {
        // It doesn't generate local page because name doesn't have a local ref
        Ok(())
    }
}

fn mk_name(
    iri: &IriRef,
    annotations: &Option<Vec<Annotation>>,
    config: &ShEx2HtmlConfig,
    prefixmap: &PrefixMap,
) -> Result<Name, ShEx2HtmlError> {
    let label = get_label(annotations, prefixmap, config)?;
    let name = iri_ref2name(iri, config, &label, prefixmap)?;
    Ok(name)
}

fn get_label(
    annotations: &Option<Vec<Annotation>>,
    prefixmap: &PrefixMap,
    config: &ShEx2HtmlConfig,
) -> Result<Option<String>, PrefixMapError> {
    for label in config.annotation_label.iter() {
        if let Some(value) = find_annotation(annotations, label, prefixmap)? {
            return Ok(Some(object_value2string(&value)));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use shex_compact::ShExParser;

    #[test]
    fn test_minininja() {
        use minijinja::{context, Environment};

        let mut env = Environment::new();
        env.add_template("hello", "Hello {{ name }}!").unwrap();
        let tmpl = env.get_template("hello").unwrap();
        assert_eq!(
            tmpl.render(context!(name => "John")).unwrap(),
            "Hello John!".to_string()
        )
    }

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
