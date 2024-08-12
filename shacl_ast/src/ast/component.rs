use crate::{
    node_kind::NodeKind, value::Value, SH_AND_STR, SH_CLASS_STR, SH_CLOSED_STR, SH_DATATYPE_STR,
    SH_DISJOINT_STR, SH_EQUALS_STR, SH_FLAGS_STR, SH_HAS_VALUE_STR, SH_IGNORED_PROPERTIES_STR,
    SH_IN_STR, SH_IRI_STR, SH_LANGUAGE_IN_STR, SH_LESS_THAN_OR_EQUALS_STR, SH_LESS_THAN_STR,
    SH_MAX_COUNT_STR, SH_MAX_EXCLUSIVE_STR, SH_MAX_INCLUSIVE_STR, SH_MAX_LENGTH_STR,
    SH_MIN_COUNT_STR, SH_MIN_EXCLUSIVE_STR, SH_MIN_INCLUSIVE_STR, SH_MIN_LENGTH_STR, SH_NODE_STR,
    SH_NOT_STR, SH_OR_STR, SH_PATTERN_STR, SH_QUALIFIED_MAX_COUNT_STR, SH_QUALIFIED_MIN_COUNT_STR,
    SH_QUALIFIED_VALUE_SHAPE_STR, SH_UNIQUE_LANG_STR, SH_XONE_STR,
};
use iri_s::{iri, IriS};
use itertools::Itertools;
use oxrdf::{Literal as OxLiteral, NamedNode, Term as OxTerm};
use prefixmap::IriRef;
use srdf::{lang::Lang, literal::Literal, RDFNode, SRDFBuilder, XSD_INTEGER_STR};
use std::fmt::Display;

#[derive(Debug, Clone)]
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
            Component::Class(_) => IriS::new_unchecked(SH_CLASS_STR),
            Component::Datatype(_) => IriS::new_unchecked(SH_DATATYPE_STR),
            Component::NodeKind(_) => IriS::new_unchecked(SH_IRI_STR),
            Component::MinCount(_) => IriS::new_unchecked(SH_MIN_COUNT_STR),
            Component::MaxCount(_) => IriS::new_unchecked(SH_MAX_COUNT_STR),
            Component::MinExclusive(_) => IriS::new_unchecked(SH_MIN_EXCLUSIVE_STR),
            Component::MaxExclusive(_) => IriS::new_unchecked(SH_MAX_EXCLUSIVE_STR),
            Component::MinInclusive(_) => IriS::new_unchecked(SH_MIN_INCLUSIVE_STR),
            Component::MaxInclusive(_) => IriS::new_unchecked(SH_MAX_INCLUSIVE_STR),
            Component::MinLength(_) => IriS::new_unchecked(SH_MIN_LENGTH_STR),
            Component::MaxLength(_) => IriS::new_unchecked(SH_MAX_LENGTH_STR),
            Component::Pattern { .. } => IriS::new_unchecked(SH_PATTERN_STR),
            Component::UniqueLang(_) => IriS::new_unchecked(SH_UNIQUE_LANG_STR),
            Component::LanguageIn { .. } => IriS::new_unchecked(SH_LANGUAGE_IN_STR),
            Component::Equals(_) => IriS::new_unchecked(SH_EQUALS_STR),
            Component::Disjoint(_) => IriS::new_unchecked(SH_DISJOINT_STR),
            Component::LessThan(_) => IriS::new_unchecked(SH_LESS_THAN_STR),
            Component::LessThanOrEquals(_) => IriS::new_unchecked(SH_LESS_THAN_OR_EQUALS_STR),
            Component::Or { .. } => IriS::new_unchecked(SH_OR_STR),
            Component::And { .. } => IriS::new_unchecked(SH_AND_STR),
            Component::Not { .. } => IriS::new_unchecked(SH_NOT_STR),
            Component::Xone { .. } => IriS::new_unchecked(SH_XONE_STR),
            Component::Closed { .. } => IriS::new_unchecked(SH_CLOSED_STR),
            Component::Node { .. } => IriS::new_unchecked(SH_NODE_STR),
            Component::HasValue { .. } => IriS::new_unchecked(SH_HAS_VALUE_STR),
            Component::In { .. } => IriS::new_unchecked(SH_IN_STR),
            Component::QualifiedValueShape { .. } => {
                IriS::new_unchecked(SH_QUALIFIED_VALUE_SHAPE_STR)
            }
        }
    }
}
