use shex_ast::ast::ShapeDecl;
use std::collections::HashMap;

/// Represents a dependency graph of shapes, where each shape can depend on other shapes.
pub struct DependencyGraph {
    /// Map from shape label to the shape declaration
    pub shapes: HashMap<String, ShapeDecl>,
    /// For each shape, stores a list of (target shape, property, min, max) tuples for related shapes.
    pub dependencies: HashMap<String, Vec<(String, String, Option<i32>, Option<i32>)>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Load shapes and build the dependency graph, including related shapes and cardinalities
    pub fn load_shapes(&mut self, shapes: Vec<ShapeDecl>) {
        for shape in &shapes {
            let label = shape.id.to_string();
            self.shapes.insert(label.clone(), shape.clone());
            let mut related = Vec::new();
            if let shex_ast::ast::ShapeExpr::Shape(s) = &shape.shape_expr {
                if let Some(expr) = &s.expression {
                    Self::collect_dependencies_and_related(&expr.te, &mut related);
                }
            }
            self.dependencies.insert(label, related);
        }
    }

    /// Recursively collect dependencies and related shapes (with cardinality) from a triple expression
    fn collect_dependencies_and_related(
        expr: &shex_ast::ast::TripleExpr,
        related: &mut Vec<(String, String, Option<i32>, Option<i32>)>,
    ) {
        use shex_ast::ast::TripleExpr;
        match expr {
            TripleExpr::EachOf { expressions, .. } | TripleExpr::OneOf { expressions, .. } => {
                for e in expressions {
                    Self::collect_dependencies_and_related(&e.te, related);
                }
            }
            TripleExpr::TripleConstraint { predicate, value_expr, min, max, .. } => {
                if let Some(val_expr) = value_expr {
                    if let shex_ast::ast::ShapeExpr::Ref(ref_to) = &**val_expr {
                        // Fix: treat None as exactly one (min=1, max=1)
                        let (min_val, max_val) = match (*min, *max) {
                            (None, None) => (Some(1), Some(1)),
                            (min, max) => (min, max),
                        };
                        related.push((ref_to.to_string(), predicate.to_string(), min_val, max_val));
                    }
                }
            }
            TripleExpr::TripleExprRef(_) => {}
        }
    }

    /// Get the dependencies for a given shape label, including cardinality and property
    pub fn get_dependencies(&self, label: &str) -> Option<&Vec<(String, String, Option<i32>, Option<i32>)>> {
        self.dependencies.get(label)
    }

    /// Get the shape declaration for a given label
    pub fn get_shape(&self, label: &str) -> Option<&ShapeDecl> {
        self.shapes.get(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::extract_shapes_from_shex_file;

    #[test]
    fn test_cardinality_parsing() {
        // Try both possible relative paths for robustness
        let paths = [
            "../generator/examples/cardinalities/cardinality_test.shex",
            "examples/cardinalities/cardinality_test.shex",
        ];
        let mut shapes = None;
        for path in &paths {
            if let Ok(s) = extract_shapes_from_shex_file(path) {
                if !s.is_empty() {
                    shapes = Some(s);
                    break;
                }
            }
        }
        let shapes = shapes.expect("Could not load test schema");
        let mut dep_graph = DependencyGraph::new();
        dep_graph.load_shapes(shapes);
        println!("Dependency graph keys: {:?}", dep_graph.dependencies.keys().collect::<Vec<_>>());
        // Use the full IRI as the key, matching the schema
        let person_key = "http://example.org/Person";
        let deps = dep_graph.dependencies.get(person_key).expect("No dependencies for Person");
        let enrolled_in = deps.iter().find(|(_, prop, _, _)| prop == "http://example.org/enrolledIn");
        assert!(enrolled_in.is_some(), "No :enrolledIn property found");
        let (_, _, min, max) = enrolled_in.unwrap();
        assert_eq!(*min, Some(0)); // '*' means min=0
        assert_eq!(*max, Some(-1));    // '*' means max=Some(-1) (unbounded)
    }

    #[test]
    fn test_cardinality_star() {
        let deps = load_person_deps();
        let enrolled_in = deps.iter().find(|(target, prop, _, _)| target == "http://example.org/Course" && prop == "http://example.org/enrolledIn");
        assert!(enrolled_in.is_some(), "No :enrolledIn property found");
        let (_, _, min, max) = enrolled_in.unwrap();
        assert_eq!(*min, Some(0)); // '*' means min=0
        assert_eq!(*max, Some(-1)); // '*' means max=-1 (unbounded)
    }

    #[test]
    fn test_cardinality_exactly_one() {
        let deps = load_person_deps();
        let teaches = deps.iter().find(|(target, prop, _, _)| target == "http://example.org/Course" && prop == "http://example.org/teaches");
        assert!(teaches.is_some(), "No :teaches property found");
        let (_, _, min, max) = teaches.unwrap();
        assert_eq!(*min, Some(1)); // exactly one
        assert_eq!(*max, Some(1));
    }

    #[test]
    fn test_cardinality_plus() {
        let deps = load_person_deps();
        let advises = deps.iter().find(|(target, prop, _, _)| target == "http://example.org/Course" && prop == "http://example.org/advises");
        assert!(advises.is_some(), "No :advises property found");
        let (_, _, min, max) = advises.unwrap();
        assert_eq!(*min, Some(1)); // '+' means min=1
        assert_eq!(*max, Some(-1)); // '+' means max=-1 (unbounded)
    }

    #[test]
    fn test_cardinality_optional() {
        let deps = load_person_deps();
        let member_of = deps.iter().find(|(target, prop, _, _)| target == "http://example.org/Group" && prop == "http://example.org/memberOf");
        assert!(member_of.is_some(), "No :memberOf property found");
        let (_, _, min, max) = member_of.unwrap();
        assert_eq!(*min, Some(0)); // '?' means min=0
        assert_eq!(*max, Some(1)); // '?' means max=1
    }

    #[test]
    fn test_cardinality_range() {
        let deps = load_person_deps();
        let has_badge = deps.iter().find(|(target, prop, _, _)| target == "http://example.org/Badge" && prop == "http://example.org/hasBadge");
        assert!(has_badge.is_some(), "No :hasBadge property found");
        let (_, _, min, max) = has_badge.unwrap();
        assert_eq!(*min, Some(2)); // {2,4} means min=2
        assert_eq!(*max, Some(4)); // {2,4} means max=4
    }

    fn load_person_deps() -> Vec<(String, String, Option<i32>, Option<i32>)> {
        let paths = [
            "../generator/examples/cardinalities/cardinality_test.shex",
            "examples/cardinalities/cardinality_test.shex",
        ];
        let mut shapes = None;
        for path in &paths {
            if let Ok(s) = extract_shapes_from_shex_file(path) {
                if !s.is_empty() {
                    shapes = Some(s);
                    break;
                }
            }
        }
        let shapes = shapes.expect("Could not load test schema");
        let mut dep_graph = DependencyGraph::new();
        dep_graph.load_shapes(shapes);
        let person_key = "http://example.org/Person";
        dep_graph.dependencies.get(person_key).cloned().expect("No dependencies for Person")
    }
}
