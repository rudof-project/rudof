use prefix_map::PrefixMap;
use srdf::iri::IriS;

#[derive(Debug)]
pub struct Schema<'a> {
  pub id: Option<Box<IriS>>,
  pub base: Option<Box<IriS>>,
  pub prefixes: Option<PrefixMap<'a>>
}


impl <'a> Schema<'a> {
    pub fn base(&self) -> &Option<Box<IriS>> { &self.base }
}

