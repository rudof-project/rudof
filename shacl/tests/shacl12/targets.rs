#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/shacl12-test-suite/tests/core/targets/",
        multiple_targets_001 => "multipleTargets-001",
        #[ignore = "sh:shape in the data graph as a target of the shape it points to is not implemented"]
        shape_001 => "shape-001",
        target_class_001 => "targetClass-001",
        target_class_implicit_001 => "targetClassImplicit-001",
        #[ignore = "sh:ShapeClass as an implicit class target is not implemented"]
        target_class_implicit_002 => "targetClassImplicit-002",
        target_node_001 => "targetNode-001",
        target_objects_of_001 => "targetObjectsOf-001",
        target_subjects_of_001 => "targetSubjectsOf-001",
        target_subjects_of_002 => "targetSubjectsOf-002",
        target_where_001 => "targetWhere-001",
    }
}
