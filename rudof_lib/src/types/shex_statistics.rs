use iri_s::IriS;
use shex_ast::ShapeLabelIdx;
use shex_ast::ir::dependency_graph::PosNeg;
use shex_ast::ir::shape_expr::ShapeExpr;
use shex_ast::ir::shape_label::ShapeLabel;

#[derive(Debug, Clone)]
pub struct ShExStatistics {
    /// Count of shapes grouped by number of extends clauses
    #[allow(dead_code)]
    pub extends_count: std::collections::HashMap<usize, usize>,

    /// Number of locally defined shapes
    pub local_shapes_count: usize,

    /// Total number of shapes (including imported)
    pub total_shapes_count: usize,

    /// List of (label, source, expression) tuples for all shapes
    pub shapes: Vec<(ShapeLabel, IriS, ShapeExpr)>,

    /// List of (source, positive/negative, target) dependency tuples
    pub dependencies: Vec<(ShapeLabel, PosNeg, ShapeLabel)>,

    /// Whether the schema has imported schemas
    pub has_imports: bool,

    /// List of negative cycles detected in the schema
    #[allow(dead_code)]
    pub neg_cycles: Vec<Vec<(ShapeLabelIdx, ShapeLabelIdx, Vec<ShapeLabelIdx>)>>,
}
