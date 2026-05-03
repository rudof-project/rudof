use std::fmt::Display;

use prefixmap::{PrefixMap, PrefixMapError};
use rbe::RbeTable;
use shex_ast::{
    Node, Pred, ShapeLabelIdx,
    ir::{schema_ir::SchemaIR, semantic_action_context::SemanticActionContext},
};

#[derive(Debug, Clone)]
pub struct PartitionsDisplay {
    partitions: Vec<PartitionDisplay>,
}

impl PartitionsDisplay {
    pub fn new(partitions: &[PartitionDisplay]) -> PartitionsDisplay {
        PartitionsDisplay {
            partitions: partitions.to_vec(),
        }
    }

    pub fn show_qualified(&self, nodes_prefixmap: &PrefixMap, schema: &SchemaIR) -> Result<String, PrefixMapError> {
        let mut result = String::new();
        for partition in self.partitions.iter() {
            result.push_str(&partition.show_qualified(nodes_prefixmap, schema)?);
        }
        Ok(result)
    }
}

impl Display for PartitionsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for partition in self.partitions.iter() {
            result.push_str(&format!("{}\n", partition));
        }
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone)]
pub struct PartitionDisplay {
    maybe_label: Option<ShapeLabelIdx>,
    rbes: Vec<RbeTable<Pred, Node, ShapeLabelIdx, SemanticActionContext>>,
    neighs: Vec<(Pred, Node, SemanticActionContext)>,
}

impl PartitionDisplay {
    pub fn new(
        maybe_label: Option<ShapeLabelIdx>,
        rbes: &[RbeTable<Pred, Node, ShapeLabelIdx, SemanticActionContext>],
        neighs: &[(Pred, Node, SemanticActionContext)],
    ) -> PartitionDisplay {
        PartitionDisplay {
            maybe_label,
            rbes: rbes.to_vec(),
            neighs: neighs.to_vec(),
        }
    }

    pub fn show_qualified(&self, nodes_prefixmap: &PrefixMap, schema: &SchemaIR) -> Result<String, PrefixMapError> {
        let mut result = String::new();
        if let Some(label) = self.maybe_label {
            result.push_str(&format!(" Shape {}\n", show_idx(&label, schema)));
        } else {
            result.push_str(" Base\n");
        }
        result.push_str("  Rbes: [");
        for rbe in self.rbes.iter() {
            result.push_str(&format!("{}", show_rbe_qualified(rbe, nodes_prefixmap, schema)?));
        }
        result.push_str("]\n");
        result.push_str("  Neighbors: [");
        for (pred, node, _context) in self.neighs.iter() {
            result.push_str(&format!(
                "{} {}; ",
                nodes_prefixmap.qualify(pred.iri()),
                node.show_qualified(nodes_prefixmap),
            ));
        }
        result.push_str("]\n");

        Ok(result)
    }
}

fn show_idx(idx: &ShapeLabelIdx, schema: &SchemaIR) -> String {
    schema
        .shape_label_from_idx(idx)
        .map(|l| schema.show_label(l))
        .unwrap_or_else(|| idx.to_string())
}

fn show_rbe_qualified(
    rbe: &RbeTable<Pred, Node, ShapeLabelIdx, SemanticActionContext>,
    _nodes_prefixmap: &PrefixMap,
    _schema: &SchemaIR,
) -> Result<String, PrefixMapError> {
    let mut result = String::new();
    result.push_str(&format!("Rbe = {}\n", rbe));
    Ok(result)
}

impl Display for PartitionDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        if let Some(label) = self.maybe_label {
            result.push_str(&format!("Shape {}\n", label));
        } else {
            result.push_str("Base\n");
        }
        result.push_str("  Rbes [");
        for rbe in self.rbes.iter() {
            result.push_str(&format!("{}", rbe));
        }
        result.push_str("]\n");
        result.push_str("  Neighbors: [");
        for (pred, node, _context) in self.neighs.iter() {
            result.push_str(&format!("{} {}", pred.iri(), node));
        }
        result.push_str("]\n");

        write!(f, "{}", result)
    }
}
