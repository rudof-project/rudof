use iri_s::IriS;
use prefixmap::IriRef;
use rbe::Value;
use serde::Serialize;
use srdf::Object;
use srdf::RDFError;
use srdf::SLiteral;
use srdf::numeric_literal::NumericLiteral;
use std::fmt::Display;
use std::str::FromStr;
use tracing::trace;

use crate::SchemaJsonError;

impl Value for Node {}

#[derive(PartialEq, Eq, Hash, Debug, Default, Clone)]
pub struct Node {
    node: Object,
}

impl Node {
    /// Creates a node from an [`ÌriS`]
    pub fn iri(iri: IriS) -> Node {
        Node {
            node: Object::iri(iri),
        }
    }

    /// Returns the length of the RDF Node
    pub fn length(&self) -> usize {
        self.node.length()
    }

    /// Returns the numeric value of a node if it is a numeric literal
    pub fn numeric_value(&self) -> Option<NumericLiteral> {
        self.node.numeric_value()
    }

    pub fn as_object(&self) -> &Object {
        &self.node
    }

    /// Returns the inner Object but checks if it is well-formed
    /// This is important in the case of literals like `"hi"^^xsd:integer` which can declare that they are of type integers
    /// but have a lexical form that is not an integer
    /// In that case, this function will return a WrongDatatypeLiteral
    pub fn as_checked_object(&self) -> Result<Object, SchemaJsonError> {
        trace!("as_checked_object: {:?}", self.node);
        match &self.node {
            Object::Literal(sliteral) => match sliteral {
                SLiteral::DatatypeLiteral {
                    lexical_form,
                    datatype,
                } => {
                    let obj = check_literal_datatype(lexical_form, datatype)?;
                    Ok(obj)
                }
                _ => Ok(self.node.clone()),
            },
            _ => Ok(self.node.clone()),
        }
    }

    pub fn literal(lit: SLiteral) -> Node {
        Node {
            node: Object::literal(lit),
        }
    }

    pub fn datatype(&self) -> Option<IriRef> {
        self.node.datatype()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.node)
    }
}

impl From<Object> for Node {
    fn from(node: Object) -> Self {
        Node { node }
    }
}

impl From<IriS> for Node {
    fn from(iri: IriS) -> Self {
        Node { node: iri.into() }
    }
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.node.serialize(serializer)
    }
}

fn check_literal_datatype(
    lexical_form: &str,
    datatype: &IriRef,
) -> Result<Object, SchemaJsonError> {
    trace!("check_literal_datatype: {lexical_form}^^{datatype}");
    let iri = datatype
        .get_iri()
        .map_err(|_e| SchemaJsonError::IriRefError {
            iri_ref: datatype.clone(),
        })?;
    match iri.as_str() {
        "http://www.w3.org/2001/XMLSchema#integer" => match SLiteral::parse_integer(lexical_form) {
            Ok(n) => Ok(Object::Literal(SLiteral::integer(n))),
            Err(err) => Ok(Object::Literal(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            })),
        },
        "http://www.w3.org/2001/XMLSchema#long" => match SLiteral::parse_long(lexical_form) {
            Ok(n) => Ok(Object::Literal(SLiteral::long(n))),
            Err(err) => Ok(Object::Literal(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            })),
        },
        "http://www.w3.org/2001/XMLSchema#double" => match SLiteral::parse_double(lexical_form) {
            Ok(d) => Ok(Object::Literal(SLiteral::double(d))),
            Err(err) => Ok(Object::Literal(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            })),
        },
        "http://www.w3.org/2001/XMLSchema#boolean" => match SLiteral::parse_bool(lexical_form) {
            Ok(b) => Ok(Object::Literal(SLiteral::boolean(b))),
            Err(err) => Ok(Object::Literal(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            })),
        },
        "http://www.w3.org/2001/XMLSchema#float" => match SLiteral::parse_float(lexical_form) {
            Ok(d) => Ok(Object::Literal(SLiteral::float(d))),
            Err(err) => Ok(Object::Literal(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            })),
        },
        "http://www.w3.org/2001/XMLSchema#decimal" => match SLiteral::parse_decimal(lexical_form) {
            Ok(d) => Ok(Object::Literal(SLiteral::decimal(d))),
            Err(err) => Ok(Object::Literal(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            })),
        },
        _ => {
            // For other datatypes, we do not check the lexical form
            // We assume it is correct
            // This includes rdf:langString
            trace!("Not checking datatype {iri}");
            Ok(Object::Literal(SLiteral::DatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
            }))
        }
    }
}

impl FromStr for Node {
    type Err = RDFError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let obj = Object::from_str(s)?;
        Ok(Node { node: obj })
    }
}
