use srdf::*;
use srdf::iri::{IRI};

pub struct Schema {
  pub id: Option<Box<dyn IRI>>,
  pub base: Option<Box<dyn IRI>>,
  pub prefixes: Option<PrefixMap>
}


impl Schema {
    pub fn base<'b>(&'b self) -> &'b Option<Box<dyn IRI>> { &self.base }
}

pub struct SchemaBuilder {
    id: Option<Box<dyn IRI>>,
    base: Option<Box<dyn IRI>>,
    prefixes: PrefixMap,
    shapes_counter: u32
}

impl <'a> SchemaBuilder {
    pub fn new() -> SchemaBuilder {
        SchemaBuilder { 
            id :None, 
            base: None, 
            prefixes: PrefixMap::new(), 
            shapes_counter: 0 
        }
    }

    pub fn addPrefix<I: IRI + 'static>(mut self, alias: Alias, iri: I) -> SchemaBuilder {
        self.prefixes.insert(alias, iri);
        self
    }

    pub fn set_base<I: IRI + 'static>(mut self, base: I) -> SchemaBuilder {
        self.base = Some(Box::new(base));
        self
    }

    pub fn build(self) -> Schema {
        Schema {
            id: self.id,
            base: self.base,
            prefixes: Some(self.prefixes)
        }
    }
}


#[test]
fn builder_test() {
    let foo = Schema {
        id: None,
        base: Some(Box::new(<MyIRI as IRI>::from(String::from("hi")))),
        prefixes: Some(PrefixMap::new())
    };
    let foo_from_builder = 
        SchemaBuilder::new()
                     .set_base(<MyIRI as IRI>::from(String::from("hi")))
                     .build();
    assert_eq!(2,2);
}