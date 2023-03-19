use prefix_map::PrefixMap;
use srdf::iri::IriS;
use std::{error::Error, fmt};
use crate::schema::Schema;
use std::str::FromStr;

pub struct SchemaBuilder<'a> {
       inner: Result<SchemaParts<'a>, ErrorBuildingSchema>
}


#[derive(Debug)]
pub enum ErrorBuildingSchema {
   NotImplemented
}
impl fmt::Display for ErrorBuildingSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match &self {
        ErrorBuildingSchema::NotImplemented => write!(f, "Not implemented!")
      }
    }
  }
impl Error for ErrorBuildingSchema {}

// type Result<T> = Result<T,ErrorBuildingSchema>

struct SchemaParts<'a> {
    id: Option<Box<IriS>>,
    base: Option<Box<IriS>>,
    prefixes: PrefixMap<'a>,
    shapes_counter: u32
}

impl <'a> SchemaBuilder<'a> {
    pub fn new() -> SchemaBuilder<'a> { 
        SchemaBuilder::default()
    }

    pub fn add_prefix(self, alias: &'a str, iri: &'a IriS) -> SchemaBuilder<'a> {
        self.and_then(move |mut schema_parts| {
            schema_parts.prefixes.insert(alias, &iri);
            Ok(schema_parts)
        })
    }

    pub fn set_base(self, base: IriS) -> SchemaBuilder<'a> {
        self.and_then(move |mut schema_parts| {
            schema_parts.base = Some(Box::new(base));
            Ok(schema_parts)
        })
    }

    pub fn add_shape(self) -> SchemaBuilder<'a> {
      self.and_then(move |mut schema_parts| {
        schema_parts.shapes_counter += 1;
        Ok(schema_parts)
      })
    }

    pub fn build(self) -> Result<Schema<'a>, ErrorBuildingSchema> {
        self.inner.and_then(|schema_parts| {
            Ok(Schema {
                id: schema_parts.id,
                base: schema_parts.base,
                prefixes: Some(schema_parts.prefixes)
            })})
    }
    
    // private
    fn and_then<F>(self, func: F) -> Self
    where
        F: FnOnce(SchemaParts<'a>) -> Result<SchemaParts<'a>, ErrorBuildingSchema>
    {
        SchemaBuilder {
            inner: self.inner.and_then(func),
        }
    }

}

impl <'a> Default for SchemaBuilder<'a> {
    fn default() -> SchemaBuilder<'a> {
        SchemaBuilder {
            inner: Ok(
              SchemaParts { 
                id: None, 
                base: None,
                prefixes: PrefixMap::default(),
                shapes_counter: 0
            }),
        }
    }
}


#[cfg(test)]
mod tests {
  use super::*;  



  fn update_base<'a>(sb: SchemaBuilder<'a>, iri: IriS) -> Result<SchemaBuilder<'a>, ErrorBuildingSchema> {
     Ok(sb.set_base(iri))
  }


 #[test]
 fn test_update() {
    let sb = SchemaBuilder::new();
    let iri = IriS::from_str("http://example.org/").unwrap();
    let schema = update_base(sb, iri).unwrap().build().unwrap();
    assert_eq!(schema.base(), &Some(Box::new(IriS::from_str("http://example.org/").unwrap())));
 }

}

