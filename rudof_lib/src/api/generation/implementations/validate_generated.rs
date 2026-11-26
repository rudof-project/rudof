use crate::{
    Result, Rudof, RudofConfig,
    errors::GenerationError,
    formats::{DataFormat, GenerationSchemaFormat, InputSpec, ShExFormat, ShaclFormat, ShapeMapFormat},
};
use std::collections::HashSet;
use std::path::Path;

/// Outcome of validating generated data against the schema it was generated from.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GenerationValidation {
    /// Schema language the data was validated with (`"ShEx"` or `"SHACL"`).
    pub language: String,
    /// Node/shape associations (ShEx) or focus nodes (SHACL) that were checked.
    pub checked: usize,
    /// How many of them conform.
    pub conformant: usize,
    /// ShEx: non-conformant associations. SHACL: validation results reported.
    pub violations: usize,
}

impl GenerationValidation {
    pub fn conforms(&self) -> bool {
        self.violations == 0
    }
}

/// Validate `data` against the source `schema`.
///
/// ShEx validation targets each node at the shape that generated it, as recorded
/// in `shapemap` (written by the generator); SHACL uses the shapes' own targets.
/// The schema is the original file, never the generator's intermediate
/// representation, so the result is independent of the generator's own checks.
pub fn validate_generated(
    config: &RudofConfig,
    schema: &Path,
    schema_format: &GenerationSchemaFormat,
    data: &Path,
    data_format: &DataFormat,
    shapemap: &Path,
) -> Result<GenerationValidation> {
    let mut rudof = Rudof::new(config.clone());
    rudof
        .load_data()
        .with_data(&[InputSpec::path(data)])
        .with_data_format(data_format)
        .execute()?;

    let is_shex = match schema_format {
        GenerationSchemaFormat::ShEx => true,
        GenerationSchemaFormat::Shacl => false,
        GenerationSchemaFormat::Auto => schema.extension().is_some_and(|e| e.eq_ignore_ascii_case("shex")),
    };

    if is_shex {
        rudof
            .load_shex_schema(&InputSpec::path(schema))
            .with_shex_schema_format(&ShExFormat::ShExC)
            .execute()?;
        rudof
            .load_shapemap(&InputSpec::path(shapemap))
            .with_shapemap_format(&ShapeMapFormat::Compact)
            .execute()?;
        rudof.validate_shex().execute()?;
        let results =
            rudof
                .shex_validation_results
                .as_ref()
                .ok_or_else(|| GenerationError::FailedValidatingGeneratedData {
                    error: "the ShEx validator produced no results".to_string(),
                })?;
        let mut outcome = GenerationValidation {
            language: "ShEx".to_string(),
            ..Default::default()
        };
        for (_, _, status) in results.iter() {
            outcome.checked += 1;
            if status.is_conformant() {
                outcome.conformant += 1;
            } else {
                outcome.violations += 1;
            }
        }
        Ok(outcome)
    } else {
        rudof
            .load_shacl_shapes()
            .with_shacl_schema(&InputSpec::path(schema))
            .with_shacl_schema_format(&ShaclFormat::Turtle)
            .execute()?;
        rudof.validate_shacl().execute()?;
        let report =
            rudof
                .shacl_validation_results
                .as_ref()
                .ok_or_else(|| GenerationError::FailedValidatingGeneratedData {
                    error: "the SHACL validator produced no report".to_string(),
                })?;
        // Focus nodes are the generated entities listed in the shapemap.
        let checked = std::fs::read_to_string(shapemap)
            .map(|s| s.split(',').filter(|a| a.contains('@')).count())
            .unwrap_or(0);
        let failing: HashSet<String> = report.results().iter().map(|r| r.focus_node().to_string()).collect();
        Ok(GenerationValidation {
            language: "SHACL".to_string(),
            checked,
            conformant: checked.saturating_sub(failing.len()),
            violations: report.results().len(),
        })
    }
}
