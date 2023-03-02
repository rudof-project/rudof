use srdf::*;
use srdf::iri::{IRI};

pub struct Schema<'a> {
  pub id: Option<Box<dyn IRI<'a>>>,
  pub base: Option<Box<dyn IRI<'a>>>,
  pub prefixes: Option<PrefixMap<'a>>
}


impl <'a> Schema<'a> {
    pub fn base<'b>(&'b self) -> &'b Option<Box<dyn IRI<'a>>> { &self.base }
}

pub struct SchemaBuilder<'a> {
    id: Option<Box<dyn IRI<'a>>>,
    base: Option<Box<dyn IRI<'a>>>,
    prefixes: PrefixMap<'a>,
    shapes_counter: u32
}

impl <'a> SchemaBuilder<'a> {
    pub fn new() -> SchemaBuilder<'a> {
        SchemaBuilder { 
            id :None, 
            base: None, 
            prefixes: PrefixMap::new(), 
            shapes_counter: 0 
        }
    }

    pub fn addPrefix<I: IRI<'a> + 'static>(mut self, alias: Alias, iri: I) -> SchemaBuilder<'a> {
        self.prefixes.insert(alias, iri);
        self
    }

    pub fn set_base<I: IRI<'a> + 'static>(mut self, base: I) -> SchemaBuilder<'a> {
        self.base = Some(Box::new(base));
        self
    }

    pub fn build(self) -> Schema<'a> {
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
        base: Some(Box::new(SIRI::from("hi"))),
        prefixes: Some(PrefixMap::new())
    };
    let foo_from_builder = 
        SchemaBuilder::new()
                     .set_base(SIRI::from("hi"))
                     .build();
    assert_eq!(
        foo.base.unwrap().to_str(),
        foo_from_builder.base.unwrap().to_str());
}