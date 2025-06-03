// Utility functions for the generator module

use shex_compact::ShExParser;
use shex_ast::ast::ShapeDecl;
use std::path::Path;

/// Loads a ShEx file from the given path and returns its shapes as a Vec<ShapeDecl>.
pub fn extract_shapes_from_shex_file<P: AsRef<Path>>(shex_path: P) -> Result<Vec<ShapeDecl>, String> {
    let schema = ShExParser::parse_buf(shex_path.as_ref(), None)
        .map_err(|e| format!("Failed to parse ShEx: {e}"))?;
    schema.shapes().ok_or_else(|| "No shapes found in schema".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_shapes_from_schema_shex() {
        let shex_path = "../generator/examples/schema.shex";
        let shapes = extract_shapes_from_shex_file(shex_path)
            .expect("Should parse schema.shex and extract shapes");
        // There should be 2 shapes: Person and Course
        assert_eq!(shapes.len(), 2, "Expected 2 shapes in schema.shex");
        let labels: Vec<String> = shapes.iter().map(|s| s.id.to_string()).collect();
        assert!(labels.iter().any(|l| l.contains("Person")), "Should contain Person shape");
        assert!(labels.iter().any(|l| l.contains("Course")), "Should contain Course shape");
    }
}

