use crate::shapemap_state::*;
use indexmap::IndexMap;
use shex_ast::Node;
use shex_ast::ShapeExprLabel;

pub struct QueryShapeMap {
    node_label_map: IndexMap<Node, IndexMap<ShapeExprLabel, ShapeMapState>>,
}
