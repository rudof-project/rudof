use std::result;

use serde::{Serialize, Serializer};

use crate::{ValueSetValueWrapper, XsFacet, NodeKind, IriRef, StringFacet, Pattern};
use super::ValueSetValue;
use serde::ser::SerializeMap;


#[derive(Debug, PartialEq)]
pub struct NodeConstraint {
    // #[serde(default, rename = "nodeKind", skip_serializing_if = "Option::is_none")]
    node_kind: Option<NodeKind>,

    // #[serde(default, skip_serializing_if = "Option::is_none")]
    datatype: Option<IriRef>,

    // #[serde(default, rename = "xsFacet", skip_serializing_if = "Option::is_none"]
    xs_facet: Option<Vec<XsFacet>>,

    // #[serde(default, skip_serializing_if = "Option::is_none")]
    values: Option<Vec<ValueSetValueWrapper>>,
}

impl NodeConstraint {
    pub fn new() -> NodeConstraint {
        NodeConstraint {
            node_kind: None,
            datatype: None, 
            xs_facet: None, 
            values: None
        }
    }

    pub fn with_datatype(mut self, datatype: IriRef) -> Self {
        self.datatype = Some(datatype);
        self
    }

    pub fn datatype(&self) -> Option<IriRef> {
       self.datatype.clone()
    }

    pub fn with_node_kind(mut self, node_kind: NodeKind) -> Self {
        self.node_kind = Some(node_kind);
        self
    }

    pub fn node_kind(&self) -> Option<NodeKind> {
        self.node_kind.clone()
    }

    pub fn with_xsfacets(mut self, facets: Vec<XsFacet>) -> Self {
        self.xs_facet = Some(facets);
        self
    }

    pub fn xs_facet(&self) -> Option<Vec<XsFacet>> {
        self.xs_facet.clone()
    }

    pub fn with_values(mut self, values: Vec<ValueSetValue>) -> Self {
        let mut vs: Vec<ValueSetValueWrapper> = Vec::with_capacity(values.len());
        for v in values {
           vs.push(ValueSetValueWrapper::new(v));
        }
        self.values = Some(vs);
        self
    }

    pub fn values(&self) -> Option<Vec<ValueSetValue>> {
        match &self.values {
            None => None,
            Some(values) => {
                let mut vs: Vec<ValueSetValue> = Vec::with_capacity(values.len());
                for v in values {
                    vs.push(v.value());
                }
                Some(vs)        
            }
        }
    }
}

impl Serialize for NodeConstraint {

    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error> 
    where S: Serializer {
      match self {
       NodeConstraint { node_kind, datatype, xs_facet, values } => {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", "NodeConstraint")?;
        match node_kind {
            None => (),
            Some(nk) => {
                map.serialize_entry("nodekind", &format!("{nk}"))?;
            }
        }
        match datatype {
            None => (),
            Some(dt) => {
                map.serialize_entry("datatype", &format!("{dt}"))?;
            }
        }
        match xs_facet {
            None => (),
            Some(facets) => {
                for f in facets {
                    match f {
                        XsFacet::StringFacet(sf) => {
                            match sf {
                                StringFacet::Length(_) => todo!(),
                                StringFacet::MinLength(_) => todo!(),
                                StringFacet::MaxLength(_) => todo!(),
                                StringFacet::Pattern(Pattern { str, flags: None}) => {
                                  map.serialize_entry("pattern", str)?;
                                },
                                StringFacet::Pattern(Pattern { str, flags: Some(fs)}) => {
                                    map.serialize_entry("pattern", str)?;
                                    map.serialize_entry("flags", fs)?;
                                  },
                            }
                        },
                        XsFacet::NumericFacet(_) => todo!(),
                    }
                }
            }
        }
        map.end()
      }
    }
}

}
