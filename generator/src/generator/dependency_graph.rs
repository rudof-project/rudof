use shex_ast::ast::ShapeDecl;
use std::collections::{HashMap, HashSet};

/// Represents a dependency graph of shapes, where each shape can depend on other shapes.
pub struct DependencyGraph {
    /// Map from shape label to the shape declaration
    pub shapes: HashMap<String, ShapeDecl>,
    /// Map from shape label to the set of shape labels it depends on
    pub dependencies: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Load shapes and build the dependency graph
    pub fn load_shapes(&mut self, shapes: Vec<ShapeDecl>) {
        for shape in &shapes {
            let label = shape.id.to_string();
            self.shapes.insert(label.clone(), shape.clone());
            let mut deps = HashSet::new();
            if let shex_ast::ast::ShapeExpr::Shape(s) = &shape.shape_expr {
                if let Some(expr) = &s.expression {
                    Self::collect_dependencies(&expr.te, &mut deps);
                }
            }
            self.dependencies.insert(label, deps);
        }
    }

    /// Recursively collect dependencies from a triple expression
    fn collect_dependencies(expr: &shex_ast::ast::TripleExpr, deps: &mut HashSet<String>) {
        use shex_ast::ast::TripleExpr;
        match expr {
            TripleExpr::EachOf { expressions, .. } | TripleExpr::OneOf { expressions, .. } => {
                for e in expressions {
                    Self::collect_dependencies(&e.te, deps);
                }
            }
            TripleExpr::TripleConstraint { value_expr, .. } => {
                if let Some(val_expr) = value_expr {
                    if let shex_ast::ast::ShapeExpr::Ref(ref_to) = &**val_expr {
                        deps.insert(ref_to.to_string());
                    }
                }
            }
            TripleExpr::TripleExprRef(_) => {}
        }
    }

    /// Get the dependencies for a given shape label
    pub fn get_dependencies(&self, label: &str) -> Option<&HashSet<String>> {
        self.dependencies.get(label)
    }

    /// Get the shape declaration for a given label
    pub fn get_shape(&self, label: &str) -> Option<&ShapeDecl> {
        self.shapes.get(label)
    }
}
