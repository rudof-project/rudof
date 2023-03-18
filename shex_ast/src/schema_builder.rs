use prefix_map::PrefixMap;
use srdf::iri::IriS;
use std::{error::Error, fmt};
use crate::schema::Schema;

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
    let schema = 
      update_base(sb, IriS::from_str("http://example.org/")).unwrap().build().unwrap();
    assert_eq!(schema.base(), &Some(Box::new(IriS::from_str("http://example.org/"))));
    /*match update_base(&mut sb, IriS::from_str("http://example.org/")) {
      Ok(sb) => { 
        let s = sb.build();
        assert_eq!(s.base.unwrap(), Box::new(IriS::from_str("http://example.org/")));      
      }, 
      Err(err) => {
        assert_eq!(2+2,4);
      }
    }; */
    // let s = r.build();
    //let r = sb; 

 }

}

/*#[test]
fn builder_test() {
    use iri_s::IriS;
    let foo = Schema {
        id: None,
        base: Some(Box::new(IriS::from_str("hi"))),
        prefixes: Some(PrefixMap::new())
    };
    let mut builder = SchemaBuilder::new();
    builder.set_base(IriS::from_str("hi"));
    let foo_from_builder = builder.build();
    let r1 = foo.base().map(|s| {Some(s)}).unwrap();
    let r2 = foo_from_builder.unwrap().base();
    assert_eq!(r1, *r2);
} */

/*#[test]
fn fn_builder() {
    use iri_s::IriS;
    let ex = IriS::from_str("http://example.org");
    let mut sb = SchemaBuilder::new();
    sb.set_base(IriS::from_str("hi"))
      .add_prefix("rdf", &ex);
    let schema = sb.build();
    let schema_base = schema.unwrap().base;
    assert_eq!(
        schema_base,
        Some(Box::new(IriS::from_str("hi"))));
}*/