//! ShEx Abstract Syntax
//!
//! Ths abstract syntax follows the [ShEx abstract syntax spec](https://shex.io/)
//!

// #![deny(rust_2018_idioms)]
pub mod ast;
pub mod compact;
pub mod ir;
pub mod node;
pub mod pred;
pub mod resolve_method;
pub mod shapemap;
pub mod shex_format;
pub mod shexr;

pub use ast::*;
pub use compact::*;
pub use ir::schema_ir_error::*;
pub use ir::shape_label_idx::*;
pub use node::*;
pub use pred::*;
use rbe::MatchCond;
pub use resolve_method::*;
pub use shex_format::*;
pub use shexr::*;

type CResult<T> = Result<T, Box<SchemaIRError>>;
type Cond = MatchCond<Pred, Node, ShapeLabelIdx>;
pub type Expr = rbe::RbeTable<Pred, Node, ShapeLabelIdx>;

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
                let es: Vec<SE1> = es.iter().map(cnv).collect::<Result<Vec<_>, SErr>>()?;

                Ok(SE1::And { es })
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
