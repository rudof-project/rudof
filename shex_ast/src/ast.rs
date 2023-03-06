use prefix_map::{Alias, PrefixMap};
use srdf::iri::IriS;

pub struct Schema<'a> {
  pub id: Option<Box<IriS>>,
  pub base: Option<Box<IriS>>,
  pub prefixes: Option<PrefixMap<'a>>
}


impl <'a> Schema<'a> {
    pub fn base(&self) -> &Option<Box<IriS>> { &self.base }
}

pub struct SchemaBuilder<'a> {
    id: Option<Box<IriS>>,
    base: Option<Box<IriS>>,
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

    pub fn addPrefix(mut self, alias: &'a str, iri: &'a IriS) -> SchemaBuilder<'a> {
        self.prefixes.insert(alias, &iri);
        self
    }

    pub fn set_base(mut self, base: IriS) -> SchemaBuilder<'a> {
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
    use iri_s::IriS;
    let foo = Schema {
        id: None,
        base: Some(Box::new(IriS::from_str("hi"))),
        prefixes: Some(PrefixMap::new())
    };
    let foo_from_builder = 
        SchemaBuilder::new()
                     .set_base(IriS::from_str("hi"))
                     .build();
    assert_eq!(
        foo.base.unwrap(),
        foo_from_builder.base.unwrap());
}