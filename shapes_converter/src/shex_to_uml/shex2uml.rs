use prefixmap::{IriRef, PrefixMap};
use shex_ast::{Schema, ShapeExprLabel};

use crate::shex_to_uml::{ShEx2UmlConfig, ShEx2UmlError, Uml};

use super::Name;

pub struct ShEx2Uml {
    config: ShEx2UmlConfig,
}

impl ShEx2Uml {
    pub fn new(config: ShEx2UmlConfig) -> ShEx2Uml {
        ShEx2Uml { config }
    }

    pub fn convert(&self, shex: &Schema) -> Result<Uml, ShEx2UmlError> {
        let prefixmap = shex.prefixmap().unwrap_or_else(|| PrefixMap::new());
        let mut uml = Uml::new();
        if let Some(shapes) = shex.shapes() {
            for shape_decl in shapes {
                let name = shape_label2name(&shape_decl.id, &self.config, &prefixmap)?;
                uml.add_label(name);
            }
        }
        Ok(uml)
    }
}

fn shape_label2name(
    label: &ShapeExprLabel,
    config: &ShEx2UmlConfig,
    prefixmap: &PrefixMap,
) -> Result<Name, ShEx2UmlError> {
    match label {
        ShapeExprLabel::IriRef { value } => iri_ref2name(value, config, prefixmap),
        ShapeExprLabel::BNode { value: _ } => todo!(),
        ShapeExprLabel::Start => todo!(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use shex_compact::ShExParser;

    #[test]
    fn test_simple() {
        let shex_str = "\
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
  :name xsd:string ;
  :knows @:Person
}";
        let mut expected_uml = Uml::new();
        expected_uml.add_label(Name::new(":Person", Some("http:/example.org/Person")));
        let shex = ShExParser::parse(shex_str, None).unwrap();
        let converter = ShEx2Uml::new(ShEx2UmlConfig::default());
        let converted_uml = converter.convert(&shex).unwrap();
        assert_eq!(converted_uml, expected_uml);
    }
}
