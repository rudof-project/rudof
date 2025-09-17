#!/bin/bash

echo "=== RUNNING ALL DATA GENERATOR TESTS ==="
echo

echo "1. Basic Passthrough Tests"
cargo test --test basic_passthrough_tests --quiet
echo

echo "2. Constraint Passthrough Tests"
cargo test --test constraint_passthrough_tests --quiet
echo

echo "3. SHACL Integration Tests"
cargo test --test shacl_integration_test --quiet
echo

echo "4. Debug Tests"
cargo test --test debug_datatype_test --quiet
cargo test --test debug_failing_test --quiet
echo

echo "5. Configuration Tests"
cargo test --test configuration_tests --quiet
echo

echo "6. Constraint Validation Tests"
cargo test --test constraint_validation_tests --quiet
echo

echo "7. Specific Constraint Tests"
cargo test --test specific_constraint_tests --quiet
echo

echo "8. Focused Constraint Tests"
cargo test --test focused_constraint_tests --quiet
echo

echo "=== SUMMARY ==="
