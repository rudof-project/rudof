// #![deny(rust_2018_idioms)]
pub mod schema;

pub mod compiled_schema;
pub mod compiled_schema_error;
pub mod node;
pub mod pred;
pub mod schema_builder;
pub mod schema_json;
pub mod schema_json_compiler;
pub mod shape_label;
pub mod shape_label_idx;
pub mod value_set;
pub mod value_set_value;


pub use compiled_schema_error::*;
use iri_s::IriS;
pub use node::*;
pub use pred::*;
use rbe::MatchCond;
pub use schema::*;
pub use schema_builder::*;
pub use schema_json_compiler::*;
pub use shape_label::*;
pub use shape_label_idx::*;
pub use value_set::*;
pub use value_set_value::*;

use srdf::Object;

type CResult<T> = Result<T, CompiledSchemaError>;
type Cond = MatchCond<Pred, Node, ShapeLabelIdx>;

#[cfg(test)]
mod tests {

    #[derive(PartialEq, Debug)]
    enum SE {
        And { es: Vec<SE> },
        Not { e: Box<SE> },
        S { v: String },
    }

    #[derive(PartialEq, Debug)]
    enum SE1 {
        And { es: Vec<SE1> },
        Not { e: Box<SE1> },
        S { v: i32 },
    }

    #[derive(PartialEq, Debug)]
    enum SErr {
        Cnv { msg: String },
    }

    fn cnv(se: &SE) -> Result<SE1, SErr> {
        match se {
            SE::And { es } => {
                let vs: Vec<Result<SE1, SErr>> = es
                    .iter()
                    .map(|se| {
                        let r = cnv(se);
                        r
                    })
                    .collect();
                let r: Result<Vec<SE1>, SErr> = vs.into_iter().collect();
                let es = r?;
                Ok(SE1::And { es: es })
            }
            SE::Not { e } => {
                let e = cnv(e)?;
                Ok(SE1::Not { e: Box::new(e) })
            }
            SE::S { v } => match v.parse::<i32>() {
                Ok(n) => Ok(SE1::S { v: n }),
                Err(e) => Err(SErr::Cnv {
                    msg: format!("Error converting {v} to i32: {e}"),
                }),
            },
        }
    }

    #[test]
    fn test_se_conversion() {
        let se = SE::And {
            es: vec![
                SE::Not {
                    e: Box::new(SE::S {
                        v: "23".to_string(),
                    }),
                },
                SE::S {
                    v: "43".to_string(),
                },
            ],
        };
        let expected = SE1::And {
            es: vec![
                SE1::Not {
                    e: Box::new(SE1::S { v: 23 }),
                },
                SE1::S { v: 43 },
            ],
        };
        assert_eq!(cnv(&se), Ok(expected))
    }

    #[test]
    fn test_se_conversion_err() {
        let se = SE::And {
            es: vec![
                SE::Not {
                    e: Box::new(SE::S {
                        v: "foo".to_string(),
                    }),
                },
                SE::S {
                    v: "43".to_string(),
                },
            ],
        };
        assert!(cnv(&se).is_err())
    }

    /*     use super::*;
    use srdf::*;
    use prefix_map::PrefixMap;

     #[test]
    fn schema_build_test() {
        let foo = Schema {
            id: None,
            base: Some(Box::new(IriS::from_str("hi"))),
            prefixes: Some(PrefixMap::new())
        };
        let mut builder = SchemaBuilder::new();
        builder.set_base(IriS::from_str("hi"));
        let foo_from_builder = builder.build();
        assert_eq!(foo.base.unwrap(),foo_from_builder.base.unwrap());
    } */
}
