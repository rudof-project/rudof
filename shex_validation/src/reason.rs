use crate::ValidatorErrors;
use prefixmap::{PrefixMap, PrefixMapError};
use ptree::{TreeBuilder, write_tree};
use serde::{Serialize, ser::SerializeMap};
use shex_ast::{
    Node, ShapeLabelIdx,
    ir::{
        node_constraint::NodeConstraint, schema_ir::SchemaIR, shape::Shape, shape_expr::ShapeExpr,
    },
};
use std::{fmt::Display, io};

/// Reason represents justifications about why a node conforms to some shape
#[derive(Debug, Clone)]
pub enum Reason {
    DescendantShapePassed {
        node: Node,
        shape: ShapeLabelIdx,
        reasons: Reasons,
    },
    ShapeExtendsPassed {
        node: Node,
        shape: Box<Shape>,
        reasons: Reasons,
    },
    NodeConstraintPassed {
        node: Node,
        nc: NodeConstraint,
    },
    ShapeAndPassed {
        node: Node,
        se: Box<ShapeExpr>,
        reasons: Vec<Vec<Reason>>,
    },
    EmptyPassed {
        node: Node,
    },
    ExternalPassed {
        node: Node,
    },
    ShapeOrPassed {
        node: Node,
        shape_expr: ShapeLabelIdx,
        reasons: Reasons,
    },
    ShapeNotPassed {
        node: Node,
        shape_expr: ShapeExpr,

        // Errors that are evidences that the negation passess
        errors_evidences: ValidatorErrors,
    },
    ShapePassed {
        node: Node,
        shape: Box<Shape>,
        idx: ShapeLabelIdx,
    },
    ShapeRefPassed {
        node: Node,
        idx: ShapeLabelIdx,
    },
}

impl Reason {
    fn build_tree(
        &self,
        tb: &mut TreeBuilder,
        _nodes_prefixmap: &PrefixMap,
        _schema: &SchemaIR,
    ) -> Result<(), PrefixMapError> {
        match self {
            Reason::NodeConstraintPassed { .. } => Ok(()),
            Reason::ShapeAndPassed { reasons, .. } => {
                tb.begin_child("reasons".to_string());
                for reason in reasons {
                    for r in reason {
                        r.build_tree(tb, _nodes_prefixmap, _schema)?;
                    }
                }
                tb.end_child();
                Ok(())
            }
            Reason::ShapeOrPassed { reasons, .. } => {
                tb.begin_child("reasons".to_string());
                for reason in reasons.iter() {
                    reason.build_tree(tb, _nodes_prefixmap, _schema)?;
                }
                tb.end_child();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn root_qualified(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
    ) -> Result<String, PrefixMapError> {
        match self {
            Reason::NodeConstraintPassed { node, nc } => Ok(format!(
                "Node constraint passed. Node: {}, Constraint: {nc}",
                node.show_qualified(nodes_prefixmap),
            )),
            Reason::ShapeAndPassed { node, se, .. } => {
                let s = format!(
                    "AND passed. Node {}, and: {}",
                    node.show_qualified(nodes_prefixmap),
                    schema.show_shape_expr(se, width)
                );
                Ok(s)
            }
            Reason::ShapePassed { node, idx, .. } => {
                let se_str = schema.show_shape_idx(idx, width);
                Ok(format!(
                    "Shape passed. Node {}, shape {}: {}",
                    node.show_qualified(nodes_prefixmap),
                    idx,
                    se_str
                ))
            }
            _ => Ok(format!("{self}",)),
        }
    }

    pub fn write_qualified<W: io::Write>(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
        writer: &mut W,
    ) -> Result<(), PrefixMapError> {
        let root_str = self.root_qualified(nodes_prefixmap, schema, width)?;
        let mut tb = TreeBuilder::new(root_str);
        self.build_tree(&mut tb, nodes_prefixmap, schema)?;
        write_tree(&tb.build(), writer).map_err(|e| PrefixMapError::IOError {
            error: e.to_string(),
        })?;
        Ok(())
    }

    pub fn show_qualified(
        &self,
        nodes_prefixmap: &PrefixMap,
        schema: &SchemaIR,
        width: usize,
    ) -> Result<String, PrefixMapError> {
        let mut v = Vec::new();
        self.write_qualified(nodes_prefixmap, schema, width, &mut v)?;
        let s = String::from_utf8(v).map_err(|e| PrefixMapError::IOError {
            error: e.to_string(),
        })?;
        Ok(s)
    }
}

/*impl TreeItem for Reason {
    type Child = Reason;

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        match self {
            Reason::ShapeAndPassed { reasons, .. } => {
                let v: Vec<Reason> = reasons.iter().flatten().cloned().collect();
                std::borrow::Cow::Owned(v)
            }
            Reason::DescendantShapePassed { reasons, .. } => {
                std::borrow::Cow::Owned(reasons.reasons.clone())
            }
            Reason::ShapeExtendsPassed { reasons, .. } => {
                std::borrow::Cow::Owned(reasons.reasons.clone())
            }
            _ => std::borrow::Cow::Borrowed(&[]),
        }
    }

    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        _prefix: &str,
        _last: bool,
    ) -> std::io::Result<()> {
        write!(f, "{}")
    }
}*/

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::NodeConstraintPassed { node, nc } => {
                write!(f, "Node constraint passed. Node: {node}, Constraint: {nc}",)
            }
            Reason::ShapeAndPassed { node, se, reasons } => {
                write!(f, "AND passed. Node {node}, and: {se}, reasons:")?;
                for reason in reasons {
                    write!(f, "[")?;
                    for r in reason {
                        write!(f, "{r}, ")?;
                    }
                    write!(f, "], ")?;
                }
                Ok(())
            }
            Reason::ShapePassed { node, shape, idx } => {
                write!(f, "Shape passed. Node {node}, shape {idx}: {shape}")
            }
            Reason::ShapeOrPassed {
                node,
                shape_expr,
                reasons,
            } => write!(
                f,
                "Shape OR passed. Node {node}, shape: {shape_expr}, reasons: {reasons}"
            ),
            Reason::ShapeNotPassed {
                node,
                shape_expr,
                errors_evidences,
            } => write!(
                f,
                "Shape NOT passed. Node {node}, shape: {shape_expr}, errors: {errors_evidences}"
            ),
            Reason::ExternalPassed { node } => write!(f, "Shape External passed for node {node}"),
            Reason::EmptyPassed { node } => write!(f, "Shape External passed for node {node}"),
            Reason::ShapeRefPassed { node, idx } => {
                write!(f, "ShapeRef passed. Node {node}, idx: {idx}")
            }
            Reason::ShapeExtendsPassed {
                node,
                shape,
                reasons,
            } => write!(
                f,
                "Shape extends passed. Node {node}, shape: {shape}, reasons: {reasons}"
            ),
            Reason::DescendantShapePassed {
                node,
                shape,
                reasons,
            } => write!(
                f,
                "Descendant shapes passed. Node {node}, shape: {shape}, reasons: {reasons}"
            ),
        }
    }
}

impl Reason {
    pub fn as_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

#[derive(Debug, Clone)]
pub struct Reasons {
    reasons: Vec<Reason>,
}

impl Reasons {
    pub fn new(reasons: Vec<Reason>) -> Reasons {
        Reasons { reasons }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Reason> {
        self.reasons.iter()
    }
}

impl Display for Reasons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for reason in self.reasons.iter() {
            writeln!(f, "  {reason}")?;
        }
        Ok(())
    }
}

impl Serialize for Reason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("reason", &self.to_string())?;
        map.end()
    }
}
