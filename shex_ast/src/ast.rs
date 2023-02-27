use srdf::*;
use srdf::iri::{IRI};

#[derive(Debug, PartialEq)]
pub struct Schema {
  pub id: Option<IRI>,
  pub base: Option<IRI>,
  pub prefixes: Option<PrefixMap>
}


impl Schema {
    pub fn base<'a>(&'a self) -> &'a Option<IRI> { &self.base }
}

#[derive(Debug, PartialEq)]
pub struct SchemaBuilder {
    id: Option<IRI>,
    base: Option<IRI>,
    prefixes: PrefixMap,
    shapes_counter: u32
}

impl SchemaBuilder {
    pub fn new() -> SchemaBuilder {
        SchemaBuilder { 
            id :None, 
            base: None, 
            prefixes: PrefixMap::new(), 
            shapes_counter: 0 
        }
    }

    pub fn addPrefix(mut self, alias: Alias, iri: IRI) -> SchemaBuilder {
        self.prefixes.insert(alias, iri);
        self
    }

    pub fn setBase(mut self, base: IRI) -> SchemaBuilder {
        self.base = Some(base);
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
        base: Some(String::from("hi")),
        prefixes: Some(PrefixMap::new())
    };
    let foo_from_builder = 
        SchemaBuilder::new()
                     .setBase(IRI::from("hi"))
                     .build();
    assert_eq!(foo_from_builder,foo);
}