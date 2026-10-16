#[cfg(test)]
mod tests {
    suite_tests! {
        crate::common::runner::validate_runner,
        "tests/data-shapes/data-shapes-test-suite/tests/core/targets/",
        multiple_targets_001 => "multipleTargets-001",
        target_class_001 => "targetClass-001",
        target_class_implicit_001 => "targetClassImplicit-001",
        target_node_001 => "targetNode-001",
        target_objects_of_001 => "targetObjectsOf-001",
        target_subjects_of_001 => "targetSubjectsOf-001",
        target_subjects_of_002 => "targetSubjectsOf-002",
    }
}
