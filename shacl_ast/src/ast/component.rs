use std::fmt::Display;
use std::str::FromStr;

use iri_s::IriS;
use itertools::Itertools;
use prefixmap::IriRef;

use crate::node_kind::NodeKind;
use crate::value::Value;
use crate::vocab::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Component {
    Class(RDFNode),
    Datatype(IriRef),
    NodeKind(NodeKind),
    MinCount(isize),
    MaxCount(isize),
    MinExclusive(Literal),
    MaxExclusive(Literal),
    MinInclusive(Literal),
    MaxInclusive(Literal),
    MinLength(isize),
    MaxLength(isize),
    Pattern {
        pattern: String,
        flags: Option<String>,
    },
    UniqueLang(bool),
    LanguageIn {
        langs: Vec<Lang>,
    },
    Equals(IriRef),
    Disjoint(IriRef),
    LessThan(IriRef),
    LessThanOrEquals(IriRef),
    Or {
        shapes: Vec<RDFNode>,
    },
    And {
        shapes: Vec<RDFNode>,
    },
    Not {
        shape: RDFNode,
    },
    Xone {
        shapes: Vec<RDFNode>,
    },
    Closed {
        is_closed: bool,
        ignored_properties: Vec<IriRef>,
    },
    Node {
        shape: RDFNode,
    },
    HasValue {
        value: Value,
    },
    In {
        values: Vec<Value>,
    },
    QualifiedValueShape {
        shape: RDFNode,
        qualified_min_count: Option<isize>,
        qualified_max_count: Option<isize>,
        qualified_value_shapes_disjoint: Option<bool>,
    },
}

impl Component {
    pub fn write<RDF>(&self, rdf_node: &RDFNode, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        match self {
            Self::Class(rdf_node) => {
                Self::write_term(&RDF::object_as_term(rdf_node), SH_CLASS_STR, rdf_node, rdf)?;
            }
            Self::Datatype(iri) => {
                Self::write_iri(iri, SH_DATATYPE_STR, rdf_node, rdf)?;
            }
            Self::NodeKind(node_kind) => {
                let iri = match &node_kind {
                    NodeKind::Iri => SH_IRI_STR,

                    _ => unimplemented!(),
                };

                Self::write_iri(&IriRef::Iri(iri!(iri)), SH_DATATYPE_STR, rdf_node, rdf)?;
            }
            Self::MinCount(value) => {
                Self::write_integer(*value, SH_MIN_COUNT_STR, rdf_node, rdf)?;
            }
            Self::MaxCount(value) => {
                Self::write_integer(*value, SH_MAX_COUNT_STR, rdf_node, rdf)?;
            }
            Self::MinExclusive(value) => {
                Self::write_literal(value, SH_MIN_EXCLUSIVE_STR, rdf_node, rdf)?;
            }
            Self::MaxExclusive(value) => {
                Self::write_literal(value, SH_MAX_EXCLUSIVE_STR, rdf_node, rdf)?;
            }
            Self::MinInclusive(value) => {
                Self::write_literal(value, SH_MIN_INCLUSIVE_STR, rdf_node, rdf)?;
            }
            Self::MaxInclusive(value) => {
                Self::write_literal(value, SH_MAX_INCLUSIVE_STR, rdf_node, rdf)?;
            }
            Self::MinLength(value) => {
                Self::write_integer(*value, SH_MIN_LENGTH_STR, rdf_node, rdf)?;
            }
            Self::MaxLength(value) => {
                Self::write_integer(*value, SH_MAX_LENGTH_STR, rdf_node, rdf)?;
            }
            Self::Pattern { pattern, flags } => {
                Self::write_literal(&Literal::str(pattern), SH_PATTERN_STR, rdf_node, rdf)?;
                if let Some(flags) = flags {
                    Self::write_literal(&Literal::str(flags), SH_FLAGS_STR, rdf_node, rdf)?;
                }
            }
            Self::UniqueLang(value) => {
                Self::write_boolean(*value, SH_UNIQUE_LANG_STR, rdf_node, rdf)?;
            }
            Self::LanguageIn { langs } => {
                langs.iter().try_for_each(|lang| {
                    Self::write_literal(
                        &Literal::str(&lang.value()),
                        SH_LANGUAGE_IN_STR,
                        rdf_node,
                        rdf,
                    )
                })?;
            }
            Self::Equals(iri) => {
                Self::write_iri(iri, SH_EQUALS_STR, rdf_node, rdf)?;
            }
            Self::Disjoint(iri) => {
                Self::write_iri(iri, SH_DISJOINT_STR, rdf_node, rdf)?;
            }
            Self::LessThan(iri) => {
                Self::write_iri(iri, SH_LESS_THAN_STR, rdf_node, rdf)?;
            }
            Self::LessThanOrEquals(iri) => {
                Self::write_iri(iri, SH_LESS_THAN_OR_EQUALS_STR, rdf_node, rdf)?;
            }
            Self::Or { shapes } => {
                shapes.iter().try_for_each(|shape| {
                    Self::write_term(&RDF::object_as_term(shape), SH_OR_STR, rdf_node, rdf)
                })?;
            }
            Self::And { shapes } => {
                shapes.iter().try_for_each(|shape| {
                    Self::write_term(&RDF::object_as_term(shape), SH_AND_STR, rdf_node, rdf)
                })?;
            }
            Self::Not { shape } => {
                Self::write_term(&RDF::object_as_term(shape), SH_PATTERN_STR, rdf_node, rdf)?;
            }
            Self::Xone { shapes } => {
                shapes.iter().try_for_each(|shape| {
                    Self::write_term(&RDF::object_as_term(shape), SH_XONE_STR, rdf_node, rdf)
                })?;
            }
            Self::Closed {
                is_closed,
                ignored_properties,
            } => {
                Self::write_boolean(*is_closed, SH_CLOSED_STR, rdf_node, rdf)?;

                ignored_properties.iter().try_for_each(|iri| {
                    Self::write_iri(iri, SH_IGNORED_PROPERTIES_STR, rdf_node, rdf)
                })?;
            }
            Self::Node { shape } => {
                Self::write_term(&RDF::object_as_term(shape), SH_NODE_STR, rdf_node, rdf)?;
            }
            Self::HasValue { value } => match value {
                Value::Iri(iri) => {
                    Self::write_iri(iri, SH_HAS_VALUE_STR, rdf_node, rdf)?;
                }
                Value::Literal(literal) => {
                    Self::write_literal(
                        &Literal::str(&literal.to_string()),
                        SH_HAS_VALUE_STR,
                        rdf_node,
                        rdf,
                    )?;
                }
            },
            Self::In { values } => {
                values.iter().try_for_each(|value| match value {
                    Value::Iri(iri) => Self::write_iri(iri, SH_HAS_VALUE_STR, rdf_node, rdf),
                    Value::Literal(literal) => Self::write_literal(
                        &Literal::str(&literal.to_string()),
                        SH_HAS_VALUE_STR,
                        rdf_node,
                        rdf,
                    ),
                })?;
            }
            Self::QualifiedValueShape {
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            } => {
                Self::write_term(
                    &RDF::object_as_term(shape),
                    SH_QUALIFIED_VALUE_SHAPE_STR,
                    rdf_node,
                    rdf,
                )?;

                if let Some(value) = qualified_min_count {
                    Self::write_integer(*value, SH_QUALIFIED_MIN_COUNT_STR, rdf_node, rdf)?;
                }

                if let Some(value) = qualified_max_count {
                    Self::write_integer(*value, SH_QUALIFIED_MAX_COUNT_STR, rdf_node, rdf)?;
                }

                if let Some(value) = qualified_value_shapes_disjoint {
                    Self::write_boolean(*value, SH_QUALIFIED_MAX_COUNT_STR, rdf_node, rdf)?;
                }
            }
        }
        Ok(())
    }

    fn write_integer<RDF>(
        value: isize,
        predicate: &str,
        rdf_node: &RDFNode,
        rdf: &mut RDF,
    ) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        let decimal_type = NamedNode::new(XSD_INTEGER_STR).unwrap();

        let term = OxTerm::Literal(OxLiteral::new_typed_literal(
            value.to_string(),
            decimal_type,
        ));

        Self::write_term(&RDF::term_s2term(&term), predicate, rdf_node, rdf)
    }

    fn write_boolean<RDF>(
        value: bool,
        predicate: &str,
        rdf_node: &RDFNode,
        rdf: &mut RDF,
    ) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        let term = OxTerm::Literal(OxLiteral::from(value));

        Self::write_term(&RDF::term_s2term(&term), predicate, rdf_node, rdf)
    }

    fn write_literal<RDF>(
        value: &Literal,
        predicate: &str,
        rdf_node: &RDFNode,
        rdf: &mut RDF,
    ) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        let term = OxTerm::Literal(OxLiteral::new_simple_literal(value.lexical_form()));

        Self::write_term(&RDF::term_s2term(&term), predicate, rdf_node, rdf)
    }

    fn write_iri<RDF>(
        value: &IriRef,
        predicate: &str,
        rdf_node: &RDFNode,
        rdf: &mut RDF,
    ) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        Self::write_term(
            &RDF::iri_s2term(&value.get_iri().unwrap()),
            predicate,
            rdf_node,
            rdf,
        )
    }

    fn write_term<RDF>(
        value: &RDF::Term,
        predicate: &str,
        rdf_node: &RDFNode,
        rdf: &mut RDF,
    ) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        rdf.add_triple(
            &RDF::object_as_subject(rdf_node).unwrap(),
            &RDF::iri_s2iri(&iri!(predicate)),
            value,
        )
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Component::Class(cls) => write!(f, "class({cls})"),
            Component::Datatype(dt) => write!(f, "datatype({dt})"),
            Component::NodeKind(nk) => write!(f, "nodeKind({nk})"),
            Component::MinCount(mc) => write!(f, "minCount({mc})"),
            Component::MaxCount(mc) => write!(f, "maxCount({mc})"),
            Component::MinExclusive(me) => write!(f, "minExclusive({me})"),
            Component::MaxExclusive(me) => write!(f, "maxExclusive({me})"),
            Component::MinInclusive(mi) => write!(f, "minInclusive({mi})"),
            Component::MaxInclusive(mi) => write!(f, "maxInclusive({mi})"),
            Component::MinLength(ml) => write!(f, "minLength({ml})"),
            Component::MaxLength(ml) => write!(f, "maxLength({ml})"),
            Component::Pattern { pattern, flags } => match flags {
                Some(flags) => write!(f, "pattern({pattern}, {flags})"),
                None => write!(f, "pattern({pattern})"),
            },
            Component::UniqueLang(ul) => write!(f, "uniqueLang({ul})"),
            Component::LanguageIn { .. } => todo!(), // write!(f, "languageIn({langs})"),
            Component::Equals(e) => write!(f, "equals({e})"),
            Component::Disjoint(d) => write!(f, "disjoint({d})"),
            Component::LessThan(lt) => write!(f, "uniqueLang({lt})"),
            Component::LessThanOrEquals(lte) => write!(f, "uniqueLang({lte})"),
            Component::Or { shapes } => {
                let str = shapes.iter().map(|s| s.to_string()).join(" ");
                write!(f, "or [{str}]")
            }
            Component::And { shapes } => {
                let str = shapes.iter().map(|s| s.to_string()).join(" ");
                write!(f, "and [{str}]")
            }
            Component::Not { shape } => {
                write!(f, "not [{shape}]")
            }
            Component::Xone { shapes } => {
                let str = shapes.iter().map(|s| s.to_string()).join(" ");
                write!(f, "xone [{str}]")
            }
            Component::Closed { .. } => todo!(),
            Component::Node { shape } => write!(f, "node({shape})"),
            Component::HasValue { value } => write!(f, "hasValue({value})"),
            Component::In { values } => {
                let str = values.iter().map(|v| v.to_string()).join(" ");
                write!(f, "In [{str}]")
            }
            Component::QualifiedValueShape { .. } => todo!(),
        }
    }
}

impl From<Component> for IriS {
    fn from(value: Component) -> Self {
        match value {
            Component::Class(_) => IriS::from_str(SH_CLASS_STR).unwrap(),
            Component::Datatype(_) => IriS::from_str(SH_DATATYPE_STR).unwrap(),
            Component::NodeKind(_) => IriS::from_str(SH_IRI_STR).unwrap(),
            Component::MinCount(_) => IriS::from_str(SH_MIN_COUNT_STR).unwrap(),
            Component::MaxCount(_) => IriS::from_str(SH_MAX_COUNT_STR).unwrap(),
            Component::MinExclusive(_) => IriS::from_str(SH_MIN_EXCLUSIVE_STR).unwrap(),
            Component::MaxExclusive(_) => IriS::from_str(SH_MAX_EXCLUSIVE_STR).unwrap(),
            Component::MinInclusive(_) => IriS::from_str(SH_MIN_INCLUSIVE_STR).unwrap(),
            Component::MaxInclusive(_) => IriS::from_str(SH_MAX_INCLUSIVE_STR).unwrap(),
            Component::MinLength(_) => IriS::from_str(SH_MIN_LENGTH_STR).unwrap(),
            Component::MaxLength(_) => IriS::from_str(SH_MAX_LENGTH_STR).unwrap(),
            Component::Pattern { .. } => IriS::from_str(SH_PATTERN_STR).unwrap(),
            Component::UniqueLang(_) => IriS::from_str(SH_UNIQUE_LANG_STR).unwrap(),
            Component::LanguageIn { .. } => IriS::from_str(SH_LANGUAGE_IN_STR).unwrap(),
            Component::Equals(_) => IriS::from_str(SH_EQUALS_STR).unwrap(),
            Component::Disjoint(_) => IriS::from_str(SH_DISJOINT_STR).unwrap(),
            Component::LessThan(_) => IriS::from_str(SH_LESS_THAN_STR).unwrap(),
            Component::LessThanOrEquals(_) => IriS::from_str(SH_LESS_THAN_OR_EQUALS_STR).unwrap(),
            Component::Or { .. } => IriS::from_str(SH_OR_STR).unwrap(),
            Component::And { .. } => IriS::from_str(SH_AND_STR).unwrap(),
            Component::Not { .. } => IriS::from_str(SH_NOT_STR).unwrap(),
            Component::Xone { .. } => IriS::from_str(SH_XONE_STR).unwrap(),
            Component::Closed { .. } => IriS::from_str(SH_CLOSED_STR).unwrap(),
            Component::Node { .. } => IriS::from_str(SH_NODE_STR).unwrap(),
            Component::HasValue { .. } => IriS::from_str(SH_HAS_VALUE_STR).unwrap(),
            Component::In { .. } => IriS::from_str(SH_IN_STR).unwrap(),
            Component::QualifiedValueShape { .. } => IriS::from_str(SH_QUALIFIED_VALUE_SHAPE_STR),
        }
    }
}
