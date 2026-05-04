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

    pub fn show_qualified(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
    ) -> Result<String, PrefixMapError> {
        let str = self
            .partitions
            .iter()
            .map(|p| p.show_qualified(nodes_prefixmap, schema, width))
            .collect::<Result<Vec<_>, _>>()? // Collects all results, or returns error early
            .join(", "); // Joins the strings with a comma
        Ok(str)
    }
}

impl Display for PartitionsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for partition in self.partitions.iter() {
            result.push_str(&format!("{}", partition));
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

    pub fn show_qualified(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
    ) -> Result<String, PrefixMapError> {
        let mut result = String::new();
        if let Some(label) = self.maybe_label {
            result.push_str(&format!("Shape {} = ", show_idx(&label, schema)));
        } else {
            result.push_str("Base = ");
        }
        let rbe_str = self
            .rbes
            .iter()
            .map(|rbe| show_rbe_qualified(rbe, nodes_prefixmap, schema, width))
            .try_fold(String::new(), |acc, rbe_str| rbe_str.map(|s| acc + &s))?;
        result.push_str(&format!("Expressions = [{}], ", rbe_str));
        let neighs_str = self
            .neighs
            .iter()
            .map(|(pred, node, _context)| {
                format!(
                    "{} {}",
                    nodes_prefixmap.qualify(pred.iri()),
                    node.show_qualified(nodes_prefixmap)
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        result.push_str(&format!("Neighbors: [{}]", neighs_str));
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
    nodes_prefixmap: &PrefixMap,
    _schema: &SchemaIR,
    width: usize,
) -> Result<String, PrefixMapError> {
    let rbe_str = rbe.show_rbe_table(
        |pred| nodes_prefixmap.qualify(pred.iri()),
        |node| node.show_qualified(nodes_prefixmap),
        width,
    );
    Ok(rbe_str)
}

impl Display for PartitionDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        if let Some(label) = self.maybe_label {
            result.push_str(&format!("Shape {} = ", label));
        } else {
            result.push_str("Base = ");
        }
        let rbe_str = self
            .rbes
            .iter()
            .map(|rbe| rbe.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        result.push_str(&format!(" Expressions [{}]\n", rbe_str));
        let neighs_str = self
            .neighs
            .iter()
            .map(|(pred, node, _context)| format!("{} {}", pred.iri(), node))
            .collect::<Vec<_>>()
            .join(", ");
        result.push_str(&format!(" Neighbors: [{}]\n", neighs_str));
        write!(f, "{}", result)
    }
}
