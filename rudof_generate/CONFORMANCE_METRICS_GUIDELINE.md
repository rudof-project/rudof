# Conformance Metrics Implementation Guideline

## Objective
Implement conformance metrics in `rudof_generate` to quantify how well generated data satisfies the original constraints represented in the Unified Constraint Model.

This is intended to support the research claim that ShEx and SHACL can be repurposed for data generation (not only validation), with measurable and reproducible evidence.

This guideline is restricted to exactly two metrics.

## Scope
The first iteration will compute metrics directly against the `UnifiedConstraintModel` already produced by:
- `src/converters/shex_to_unified.rs`
- `src/converters/shacl_to_unified.rs`

The metrics will evaluate generated RDF data from `DataGenerator::generate()` and report only:
- percentage of valid triples
- percentage of schema-to-unified translation loss

## Metrics To Implement

### 1. Triple Validity Percentage
Percentage of generated triples that are valid with respect to the unified constraints derived from the original ShEx or SHACL schema.

Definition of valid triple in this implementation:
- A triple `(s, p, o)` is valid if `p` exists in the shape constraints of the type assigned to `s` and `o` satisfies the property constraints (`Datatype`, `NodeKind`, `Pattern`, `In`, range constraints, etc.).
- `rdf:type` triples are considered valid when they point to a known shape in the unified model.

Formula:

$$
	ext{TripleValidity\%} = 100 \times \frac{\text{ValidTriples}}{\text{TotalGeneratedTriples}}
$$

### 2. Shape Translation Loss Percentage
Percentage of original shape constraints that are lost when translating ShEx/SHACL to the Unified Constraint Model.

Formula:

$$
	ext{ShapeTranslationLoss\%} = 100 \times \left(1 - \frac{\text{RepresentedConstraintsInUnified}}{\text{OriginalSchemaConstraints}}\right)
$$

Interpretation:
- `0%` means no loss during translation.
- Higher values mean more information from original schemas is not represented in unified form.

## Data Structures
Create a new module:
- `src/conformance_metrics.rs`

Primary report structs:
- `ConformanceReport`
- `ShapeConformanceReport`

`ConformanceReport` should include:
- `total_generated_triples`
- `valid_triples`
- `triple_validity_percentage`
- `original_schema_constraints`
- `represented_constraints_in_unified`
- `shape_translation_loss_percentage`
- derived rates:
   - `triple_validity_percentage`
   - `shape_translation_loss_percentage`

## Integration Plan

### Phase 1: Core Metric Engine
Implement:
- `ConformanceReport::from_graph_and_model(graph, model)`

Algorithm outline:
1. Traverse generated triples once.
2. Build:
   - typed focus nodes map from `rdf:type`
   - property-value map per subject
3. For each generated triple, mark valid/invalid according to unified constraints.
4. Count original schema constraints and represented unified constraints.
5. Compute the two percentages.

### Phase 2: Pipeline Hook
In `DataGenerator` (`src/lib.rs`):
1. Add `last_conformance_report: Option<ConformanceReport>` field.
2. In `generate()`, after graph generation and before write:
   - if unified model is loaded, compute report and store it.
3. Add accessor:
   - `pub fn last_conformance_report(&self) -> Option<&ConformanceReport>`
4. Add lightweight `tracing::info!` summary with rates.

### Phase 3: Public API
Re-export report types from `src/lib.rs` for consumers:
- `pub use conformance_metrics::{ConformanceReport, ShapeConformanceReport};`

## Constraint Coverage Used For Triple Validation

### Implemented in v1
- `Datatype`
- `ShapeReference`
- `NodeKind`
- `Pattern` (regex)
- `MinInclusive`, `MaxInclusive`
- `MinExclusive`, `MaxExclusive`
- `MinLength`, `MaxLength`
- `In`
- `HasValue`
- property `min_cardinality`, `max_cardinality`
- shape `closed`

### Notes
- Cardinality contributes indirectly to triple validity by validating occurrences of each property per subject.
- Invalid regex patterns should mark corresponding checks as invalid for the metric.

## ShEx And SHACL Coverage Matrix

This matrix summarizes what the current generator can translate into the unified model and therefore use for generation and metric computation.

| Feature | ShEx | SHACL | Status | Notes |
| --- | --- | --- | --- | --- |
| Shape declarations | Yes | Yes | Supported | Core shape IDs are preserved |
| Property constraints | Yes | Yes | Supported | Property IRI and associated constraints are extracted |
| Cardinality | Yes | Yes | Supported | `min`/`max` in ShEx, `sh:minCount`/`sh:maxCount` in SHACL |
| Datatype | Yes | Yes | Supported | Mapped to unified datatype constraints |
| Node kind | Yes | Yes | Supported | Basic node kinds are mapped |
| Pattern | Yes | Yes | Supported | Regex patterns are extracted when present |
| Length facets | Yes | Yes | Partially supported | `minLength`/`maxLength` supported; exact length is not translated |
| Numeric range facets | Yes | Yes | Supported | Inclusive/exclusive bounds are extracted |
| Value set / enumeration | Yes | Yes | Supported | `in` and `hasValue` are represented |
| Shape references | Yes | Yes | Supported | Recursive or referenced shapes are mapped |
| Closed shapes | Partial | Partial | Partially supported | Present in the model, but not fully extracted from schemas |
| Triple expression composition | Yes | Partial | Partially supported | `EachOf` / `OneOf` are handled; some advanced references are not |
| Logical combinators | Limited | No | Not fully supported | SHACL `sh:or`, `sh:and`, `sh:not`, `sh:xone` are not fully translated |
| Qualified value shapes | Limited | No | Not fully supported | Not mapped in the unified model |
| SPARQL-based constraints | No | No | Not supported | Outside current generator scope |
| Semantic actions / imports | Limited | Limited | Not supported | Not preserved in the unified model |

### Paper Interpretation
- The generator does not implement the full ShEx or SHACL semantics.
- It implements a generation-oriented subset sufficient for controlled synthetic data production.
- The translation-loss metric quantifies how much of the original schema is not represented after conversion to the unified model.

## Test Plan
Create new integration tests in:
- `tests/conformance_metrics_tests.rs`

Minimum tests:
1. `triple_validity_is_high_when_required_constraints_are_generated`
   - SHACL schema with required datatype property.
   - Expected high `TripleValidity%`.
2. `triple_validity_detects_missing_required_properties`
   - Force missing required properties using generation config.
   - Expected lower `TripleValidity%`.
3. `shape_translation_loss_is_measurable`
   - Use a schema with constraints known to be unsupported in unified translation.
   - Expected `ShapeTranslationLoss% > 0`.

Optional extensions:
4. Pattern violation impact on `TripleValidity%`.
5. Compare ShEx vs SHACL loss percentages.

## Acceptance Criteria
- New module compiles and is wired in `DataGenerator`.
- Report is available after generation via public accessor.
- At least 3 integration tests pass (including loss metric).
- Logs include both percentages.
- No regressions in existing `rudof_generate` tests.

## Commands To Validate
From repository root:

```bash
cargo test -p rudof_generate conformance_metrics -- --nocapture
cargo test -p rudof_generate
```

## Out of Scope (Current Iteration)
- Full SHACL report generation using `shacl_validation` in this path.
- Full ShEx validator orchestration (`shex_validation`) in this path.
- Per-constraint probabilistic calibration metrics.
- CLI flags for exporting metrics to JSON/CSV.

## Next Iteration (After v1)
1. Add optional export file for report (`JSON`).
2. Add CLI toggle to enable/disable conformance computation.
3. Add paper-ready summaries per schema family (ShEx/SHACL).
4. Add comparison baseline with random generation.
