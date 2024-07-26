use std::fs::OpenOptions;

use crate::ShEx2HtmlError;
use minijinja::Template;
use minijinja::{path_loader, Environment};
use prefixmap::{IriRef, PrefixMap};
use shex_ast::{Schema, Shape, ShapeExpr, ShapeExprLabel, TripleExpr};
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

        let shape_template = environment.get_template("shape.html")?;
        for shape in self.current_html.shapes() {
            generate_shape_page(shape, &shape_template, &self.config)?;
        }

        // Old geeration of shapes
        // generate_html(&self.current_html, &self.config)?;
        /*if let Some(css_file) = &self.config.css_file_name {
            generate_css_file(css_file, &self.config)?;
        }*/
        Ok(())
    }

    pub fn convert(&mut self, shex: &Schema) -> Result<(), ShEx2HtmlError> {
        let prefixmap = shex
            .prefixmap()
            .unwrap_or_default()
            .without_rich_qualifying();
        if let Some(shapes) = shex.shapes() {
            for shape_decl in shapes {
                let name = self.shape_label2name(&shape_decl.id, &prefixmap)?;
                let node_id = self.current_html.add_label(&name);
                let component =
                    self.shape_expr2htmlshape(&name, &shape_decl.shape_expr, &prefixmap, &node_id)?;
                self.current_html.add_component(node_id, component)?;
            }
        }
        Ok(())
    }

    fn shape_label2name(
        &mut self,
        label: &ShapeExprLabel,
        prefixmap: &PrefixMap,
    ) -> Result<Name, ShEx2HtmlError> {
        match label {
            ShapeExprLabel::IriRef { value } => iri_ref2name(value, &self.config, prefixmap),
            ShapeExprLabel::BNode { value: _ } => todo!(),
            ShapeExprLabel::Start => todo!(),
        }
    }

    fn shape_expr2htmlshape(
        &mut self,
        name: &Name,
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
        name: &Name,
        shape: &Shape,
        prefixmap: &PrefixMap,
        current_node_id: &NodeId,
    ) -> Result<HtmlShape, ShEx2HtmlError> {
        let mut html_shape = HtmlShape::new(name.clone());
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
                    let _ref_name = iri_ref2name(value, &self.config, prefixmap)?;
                    /*self.current_uml.add_link(
                        *current_node_id,
                        ref_name,
                        current_predicate.clone(),
                        current_card.clone(),
                    )?; */
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
    config: &ShEx2HtmlConfig,
    prefixmap: &PrefixMap,
) -> Result<Name, ShEx2HtmlError> {
    match iri_ref {
        IriRef::Iri(iri) => Ok(Name::new(
            prefixmap.qualify(iri).as_str(),
            Some(iri.as_str()),
            config.target_folder.as_path(),
        )),
        IriRef::Prefixed { prefix: _, local } => {
            // TODO: Check if we could replace href as None by a proper IRI
            // println!("Is a local name: {local}");
            Ok(Name::new(local, None, config.target_folder.as_path()))
        }
    }
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
    debug!("Generating shape with name: {name:?}");
    if let Some((path, _local_name)) = name.as_local_ref() {
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
        Err(ShEx2HtmlError::NoLocalRefName { name: name.clone() })
    }
}

/*fn generate_html(html_schema: &HtmlSchema, config: &ShEx2HtmlConfig) -> Result<(), ShEx2HtmlError> {
    /* open_html(&mut writer)?;
    header(&mut writer, &config.title, config)?;
    open_tag("body", &mut writer)?;
    tag_txt("h1", config.title.as_str(), &mut writer)?; */
    generate_shapes(html_schema.shapes(), config)?;
    /* close_tag("body", &mut writer)?;
    close_html(&mut writer)?; */
    Ok(())
}

fn generate_shapes<'a>(
    shapes: impl Iterator<Item = &'a HtmlShape>,
    config: &ShEx2HtmlConfig,
) -> Result<(), ShEx2HtmlError> {
    /* open_tag("ul", writer)?; */
    for html_shape in shapes {
        let name = html_shape.name();
        debug!("Generating shape with name: {name:?}");
        if let Some((path, _local_name)) = name.as_local_ref() {
            let file_name = path.as_path().display().to_string();
            let file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path)
                .map_err(|e| ShEx2HtmlError::ErrorCreatingShapesFile {
                    name: file_name,
                    error: e,
                })?;
            // write_li_shape(name.name().as_str(), local_name.as_str(), writer)?;
            write_shape(Box::new(file), html_shape, config)?;
        } else {
            debug!("No local ref for that name");
        }
    }
    // close_tag("ul", writer)?;
    Ok(())
}

fn open_html(writer: &mut Box<dyn Write>) -> Result<(), io::Error> {
    open_tag("html", writer)?;
    Ok(())
}

fn close_html(writer: &mut Box<dyn Write>) -> Result<(), io::Error> {
    close_tag("html", writer)?;
    Ok(())
}

fn open_tag(tag: &str, writer: &mut Box<dyn Write>) -> Result<(), io::Error> {
    write!(writer, "<{tag}>")?;
    Ok(())
}

fn open_tag_attrs(
    tag: &str,
    attrs: Vec<(&str, &str)>,
    writer: &mut Box<dyn Write>,
) -> Result<(), io::Error> {
    write!(writer, "<{tag} ")?;
    for (name, value) in attrs {
        write!(writer, " {name}=\"{value}\"")?;
    }
    Ok(())
}

fn tag_txt(tag: &str, txt: &str, writer: &mut Box<dyn Write>) -> Result<(), io::Error> {
    write!(writer, "<{tag}>{txt}</{tag}>")?;
    Ok(())
}

fn tag_txt_attrs(
    tag: &str,
    attrs: Vec<(&str, &str)>,
    txt: &str,
    writer: &mut Box<dyn Write>,
) -> Result<(), io::Error> {
    write!(writer, "<{tag}")?;
    for (name, value) in attrs {
        write!(writer, " {name}=\"{value}\"")?;
    }
    write!(writer, ">{txt}</{tag}>")?;
    Ok(())
}

fn tag_attrs(
    tag: &str,
    attrs: Vec<(&str, &str)>,
    writer: &mut Box<dyn Write>,
) -> Result<(), io::Error> {
    write!(writer, "<{tag}")?;
    for (name, value) in attrs {
        write!(writer, " {name}=\"{value}\"")?;
    }
    write!(writer, "/>")?;
    Ok(())
}

fn close_tag(tag: &str, writer: &mut Box<dyn Write>) -> Result<(), io::Error> {
    write!(writer, "</{tag}>")?;
    Ok(())
}

fn write_shape(
    mut writer: Box<dyn Write>,
    html_shape: &HtmlShape,
    config: &ShEx2HtmlConfig,
) -> Result<(), ShEx2HtmlError> {
    open_html(&mut writer)?;
    header(&mut writer, html_shape.name().name().as_str(), config)?;
    open_tag("body", &mut writer)?;
    tag_txt("h1", html_shape.name().name().as_str(), &mut writer)?;
    open_tag_attrs("table", vec![("class", "table")], &mut writer)?;
    open_tag("tr", &mut writer)?;
    open_tag("thead", &mut writer)?;
    tag_txt("th", "Property", &mut writer)?;
    tag_txt("th", "Expected value", &mut writer)?;
    tag_txt("th", "Cardinality", &mut writer)?;
    close_tag("tr", &mut writer)?;
    close_tag("thead", &mut writer)?;
    open_tag("tbody", &mut writer)?;
    for entry in html_shape.entries() {
        write_entry(&mut writer, entry)?;
    }
    close_tag("tbody", &mut writer)?;
    close_tag("table", &mut writer)?;
    direct_txt(&mut writer, "<script src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js\" integrity=\"sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz\" crossorigin=\"anonymous\"></script>")?;
    close_tag("body", &mut writer)?;
    close_html(&mut writer)?;
    Ok(())
}

fn write_entry(writer: &mut Box<dyn Write>, entry: &Entry) -> Result<(), ShEx2HtmlError> {
    open_tag("tr", writer)?;
    tag_txt_attrs(
        "td",
        vec![("class", "property_name")],
        name2html(&entry.name).as_str(),
        writer,
    )?;
    tag_txt(
        "td",
        value_constraint2html(&entry.value_constraint).as_str(),
        writer,
    )?;
    tag_txt("td", cardinality2html(&entry.card).as_str(), writer)?;
    close_tag("tr", writer)?;
    Ok(())
}

fn header(
    writer: &mut Box<dyn Write>,
    title: &str,
    config: &ShEx2HtmlConfig,
) -> Result<(), ShEx2HtmlError> {
    open_tag("head", writer)?;
    direct_txt(writer, "<meta charset=\"utf-8\">")?;
    direct_txt(
        writer,
        "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">",
    )?;
    tag_txt("title", title, writer)?;
    if let Some(css_file) = &config.css_file_name {
        tag_attrs(
            "link",
            vec![("href", css_file.as_str()), ("rel", "stylesheet")],
            writer,
        )?;
    }
    direct_txt(writer, "<link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH\" crossorigin=\"anonymous\">")?;
    close_tag("head", writer)?;
    Ok(())
}

fn direct_txt(writer: &mut Box<dyn Write>, str: &str) -> Result<(), ShEx2HtmlError> {
    write!(writer, "{str}")?;
    Ok(())
}

fn name2html(name: &Name) -> String {
    if let Some(href) = name.href() {
        format!("<a href=\"{}\">{}</a>", href, name.name())
    } else {
        name.name()
    }
}

fn ref2html(name: &Name) -> String {
    if let Some(local_ref) = name.as_local_href() {
        format!("<a href=\"{}\">@{}</a>", local_ref, name.name())
    } else {
        name.name()
    }
}

fn value_constraint2html(value_constraint: &ValueConstraint) -> String {
    match value_constraint {
        ValueConstraint::Any => "Any".to_string(),
        ValueConstraint::Datatype(name) => name2html(name),
        ValueConstraint::Ref(r) => ref2html(r),
        ValueConstraint::None => "None".to_string(),
        ValueConstraint::ValueSet(_) => todo!(),
    }
}

fn cardinality2html(card: &Cardinality) -> String {
    match card {
        Cardinality::OneOne => "".to_string(),
        Cardinality::Star => "*".to_string(),
        Cardinality::Plus => "+".to_string(),
        Cardinality::Optional => "?".to_string(),
        Cardinality::Range(m, n) => format!("[{m}-{n}]"),
        Cardinality::Fixed(m) => format!("[{m}]"),
    }
}

fn url_for_name(name: ViaDeserialize<Name>) -> Result<String, Error> {
    Ok(name.name())
}
*/
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
