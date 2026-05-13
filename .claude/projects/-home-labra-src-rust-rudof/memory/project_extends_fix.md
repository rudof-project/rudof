---
name: extends-validation-fix
description: Fix for ShEx EXTENDS validation in check_node_extends_main_shape — NC recursive check + exhaustive triple expr check with shape-ref guard
metadata:
  type: project
---

In shex_validation/src/engine.rs, `check_node_extends_main_shape` was refactored in commit 551bbc1b to check parent shapes using `check_node_shape`, which incorrectly validated triple expressions against the full node neighborhood (not the partition). This caused 6 regressions (pass/t33 tests incorrectly failing, vitals tests incorrectly failing).

**Root causes fixed:**
1. `check_node_shape` was called for parent's triple expr, pulling ALL triples (incl. those belonging to child's partition) → q-triple issue for I's {q:5} when node also has q:6
2. For CLOSED empty extends shapes (e.g. `EXTENDS @<Vital> CLOSED {}`), `check_node_shape` incorrectly failed the closed check because the node's triples appear as "remainder"

**Fix applied:**
In `check_node_extends_main_shape`:
1. Removed `check_node_shape` call entirely
2. Added **recursive NC checking** through the ancestor extends chain (handles cases like A's regex `/sA.../` failing for `sxBCDEFGHIJx`)
3. Added **exhaustive triple expr check** with pre-filtering: for shapes with their own triple expression AND no shape-reference value conditions (MatchCond::Ref), pre-filter triples to those matching any component's value condition, then check cardinality. This correctly handles open shapes where extra values should be ignored.

The `cond_has_ref` helper function checks recursively if a MatchCond contains a Ref variant, which would require the partition algorithm to handle correctly.

**Why: ** Without the recursive NC check, t24-t27 incorrectly passed (NCs like A's regex weren't checked). Without the exhaustive triple check, t30-t32 incorrectly passed (E's {p:[2]}, F's {p:[3]}, G's {p:[4]} weren't checked). The shape-ref guard on the exhaustive check prevents false failures for shapes with nested shape references (like Posture's fhir:component constraint).

**Net result vs pre-fix HEAD:**
- Fixed all 6 regressions from commit 551bbc1b
- Also fixed AND3G-fail_G2pattern, ExtendAND3G-fail_G3pattern, vitals-pass_sit-PostureVital
- Total baseline reduced from 46 to 40 failing tests
