use prefix_map::PrefixMap;
use srdf::iri::IriS;
use std::{error::Error, fmt};
use crate::schema::Schema;

pub struct SchemaBuilder {
       inner: Result<SchemaParts, ErrorBuildingSchema>
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

struct SchemaParts {
    id: Option<IriS>,
    base: Option<IriS>,
    prefixes: PrefixMap,
    shapes_counter: u32
}

impl SchemaBuilder {
    pub fn new() -> SchemaBuilder { 
        SchemaBuilder::default()
    }

    pub fn add_prefix(self, alias: &str, iri: IriS) -> SchemaBuilder {
        self.and_then(move |mut schema_parts| {
            schema_parts.prefixes.insert(alias, iri);
            Ok(schema_parts)
        })
    }

    pub fn set_base(self, base: IriS) -> SchemaBuilder {
        self.and_then(move |mut schema_parts| {
            schema_parts.base = Some(base);
            Ok(schema_parts)
        })
    }

    pub fn add_shape(self) -> SchemaBuilder {
      self.and_then(move |mut schema_parts| {
        schema_parts.shapes_counter += 1;
        Ok(schema_parts)
      })
    }

    pub fn build(self) -> Result<Schema, ErrorBuildingSchema> {
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
        F: FnOnce(SchemaParts) -> Result<SchemaParts, ErrorBuildingSchema>
    {
        SchemaBuilder {
            inner: self.inner.and_then(func),
        }
    }

}

impl <'a> Default for SchemaBuilder {
    fn default() -> SchemaBuilder {
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
  use std::str::FromStr;



  fn update_base(sb: SchemaBuilder, iri: IriS) -> Result<SchemaBuilder, ErrorBuildingSchema> {
     Ok(sb.set_base(iri))
  }

  fn update_prefix_map(sb: SchemaBuilder, alias: &str, iri: &IriS) -> Result<SchemaBuilder, ErrorBuildingSchema> {
    Ok(sb.add_prefix(alias,iri.to_owned()))
 }

 #[test]
 fn test_update_base() {
    let sb = SchemaBuilder::new();
    let iri = IriS::from_str("http://example.org/").unwrap();
    let schema = update_base(sb, iri).unwrap().build().unwrap();
    assert_eq!(schema.base(), Some(IriS::from_str("http://example.org/").unwrap()));
 }

 #[test]
 fn test_update_prefix_map() {
    let sb = SchemaBuilder::new();
    let iri = IriS::from_str("http://example.org/").unwrap();
    let schema = update_prefix_map(sb, &"ss", &iri).unwrap().build().unwrap();
    assert_eq!(
        schema.resolve(&"ss:foo").unwrap(), 
        Some(IriS::from_str("http://example.org/foo").unwrap())
    );
 }


}

