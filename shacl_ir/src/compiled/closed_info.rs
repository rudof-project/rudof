use iri_s::IriS;
use shacl_ast::node_shape::NodeShape;
use srdf::Rdf;

#[derive(Debug, Clone)]
pub enum ClosedInfo {
    Yes { ignored_properties: Vec<IriS> },
    No,
}

impl ClosedInfo {
    pub fn is_closed(&self) -> bool {
        matches!(self, ClosedInfo::Yes { .. })
    }

    pub fn ignored_properties(&self) -> Option<&Vec<IriS>> {
        match self {
            ClosedInfo::Yes { ignored_properties } => Some(ignored_properties),
            ClosedInfo::No => None,
        }
    }

    pub fn get_closed_info_node_shape<R: Rdf>(shape: &NodeShape<R>) -> Self {
        let (is_closed, ignored_properties) = shape.closed_component();
        if is_closed {
            let ignored = ignored_properties
                .into_iter()
                .map(|iri| iri.into())
                .collect();
            ClosedInfo::Yes {
                ignored_properties: ignored,
            }
        } else {
            ClosedInfo::No
        }
    }
}

impl Default for ClosedInfo {
    fn default() -> Self {
        ClosedInfo::No
    }
}
