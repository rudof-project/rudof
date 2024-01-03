use std::fmt::Display;

use prefixmap::IriRef;
use srdf::{numeric_literal::NumericLiteral, RDFNode, SHACLPath};

use crate::{component::Component, message_map::MessageMap, severity::Severity, target::Target};

#[derive(Debug, Clone)]
pub struct PropertyShape {
    id: RDFNode,
    path: SHACLPath,
    components: Vec<Component>,
    targets: Vec<Target>,
    property_shapes: Vec<RDFNode>,
    closed: bool,
    ignored_properties: Vec<IriRef>,
    deactivated: bool,
    message: MessageMap,
    severity: Option<Severity>,
    name: MessageMap,
    description: MessageMap,

    // SHACL spec says that the values of sh:order should be decimals but in the examples they use integers. `NumericLiteral` also includes doubles.
    order: Option<NumericLiteral>,

    group: Option<RDFNode>,
    source_iri: Option<IriRef>,
    annotations: Vec<(IriRef, RDFNode)>,
}

impl Display for PropertyShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
