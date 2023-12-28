use prefixmap::IriRef;
use srdf::{RDFNode, literal::Literal, lang::Lang};

use crate::{node_kind::NodeKind, value::Value};

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
    Pattern{ pattern: String, flags: Option<String> },
    UniqueLang(bool),
    LanguageIn{ langs: Vec<Lang> },
    Equals(IriRef),
    Disjoint(IriRef),
    LessThan(IriRef),
    LessThanOrEquals(IriRef),
    Or { shapes: Vec<RDFNode> },
    And { shapes: Vec<RDFNode> },
    Not { shape: RDFNode },
    Xone { shapes: Vec<RDFNode> },
    Closed { is_closed: bool, ignored_properties: Vec<IriRef> },
    Node { shape: RDFNode },
    HasValue { value: Value },
    In { values: Vec<Value> },
    QualifiedValueShape { shape: RDFNode, qualified_min_count: Option<isize>, qualified_max_count: Option<isize>, qualified_value_shapes_disjoint: Option<bool>}
}