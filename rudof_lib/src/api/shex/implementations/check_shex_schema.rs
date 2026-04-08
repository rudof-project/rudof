use crate::{
    Result, Rudof,
    api::shex::implementations::load_shex_schema::load_shex_schema,
    errors::ShExError,
    formats::{DataReaderMode, InputSpec, ShExFormat},
};
use std::io;

pub fn check_shex_schema<W: io::Write>(
    rudof: &Rudof,
    schema: &InputSpec,
    schema_format: Option<&ShExFormat>,
    base_schema: Option<&str>,
    writer: &mut W,
) -> Result<bool> {
    // Step 1: Check well-formedness by attempting to load the schema
    let mut temp_config = rudof.config.clone();
    let mut validator_cfg = temp_config.validator_config();
    validator_cfg.set_check_negation_requirement(false);
    temp_config.shex_validator = Some(validator_cfg);
    let mut temp_rudof = Rudof::new(temp_config);

    let is_schema_well_formed = match load_shex_schema(
        &mut temp_rudof,
        schema,
        schema_format,
        base_schema,
        Some(&DataReaderMode::Lax),
    ) {
        Ok(_) => true,
        Err(e) => {
            writeln!(writer, "Schema is malformed. Error during parsing:\n{}\n", e)
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
            false
        },
    };

    // If schema is malformed, no need to check for negative cycles
    if !is_schema_well_formed {
        return Ok(false);
    }

    // Step 2: Check for negative cycles in the dependency graph
    let shex_schema_ir = temp_rudof.shex_schema_ir.unwrap();

    let neg_cycles = shex_schema_ir.neg_cycles();
    let has_neg_cycles = !neg_cycles.is_empty();

    if has_neg_cycles {
        writeln!(writer, "Schema contains negative cycles in its dependency graph:\n")
            .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;

        for (cycle_idx, cycle_edges) in neg_cycles.iter().enumerate() {
            writeln!(writer, "Negative cycle #{}:", cycle_idx + 1)
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;

            let (shapes, edges) = shex_schema_ir.format_cycle_details(cycle_edges);

            writeln!(writer, "  Shapes involved:")
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;

            for shape in &shapes {
                writeln!(writer, "    - {}", shape)
                    .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
            }

            writeln!(writer, "\n  Negative cycle path:")
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;

            for path in &edges {
                writeln!(writer, "    {}", path).map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
            }

            writeln!(writer).map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
        }

        return Ok(false);
    }

    // Schema is both well-formed and has no negative cycles
    writeln!(writer, "Schema is valid: well-formed and contains no negative cycles.")
        .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
    Ok(true)
}
