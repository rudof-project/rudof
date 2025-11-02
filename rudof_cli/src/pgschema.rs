use crate::writer::get_writer;
use anyhow::{Result, bail};
use clap::ValueEnum;
use iri_s::IriS;
use pgschema::parser::pgs_builder::PgsBuilder;
use rudof_lib::{
    InputSpec, Rudof, RudofConfig, data::get_data_rudof, data_format::DataFormat,
    query::execute_query, query_result_format::ResultQueryFormat as CliResultQueryFormat,
    query_type::QueryType,
};
use srdf::ReaderMode;
use std::{fmt::Display, io::Read, path::PathBuf};

#[allow(clippy::too_many_arguments)]
pub fn run_pgschema(
    schema: &InputSpec,
    _schema_format: &PgSchemaFormat,
    output: &Option<PathBuf>,
    _result_format: &PgSchemaFormat,
    force_overwrite: bool,
    _config: &RudofConfig,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    // let mut rudof = Rudof::new(config)?;
    let mut reader = schema.open_read(None, "PGSchema")?;
    let schema = get_schema(&mut reader)?;
    write!(writer, "{schema}")?;
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Default)]
pub enum PgSchemaFormat {
    #[default]
    PgSchemaC,
}

impl Display for PgSchemaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PgSchemaFormat::PgSchemaC => "compact",
        };
        write!(f, "{}", s)
    }
}

fn get_schema<R: Read>(reader: &mut R) -> Result<pgschema::pgs::PropertyGraphSchema> {
    let mut schema_content = String::new();
    reader.read_to_string(&mut schema_content);
    let schema: pgschema::pgs::PropertyGraphSchema =
        match PgsBuilder::new().parse_pgs(schema_content.as_str()) {
            Ok(schema) => schema,
            Err(e) => {
                bail!("Failed to parse schema: {}", e);
            }
        };
    Ok(schema)
}
