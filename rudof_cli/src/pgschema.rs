use crate::writer::get_writer;
use anyhow::{Context, Result, bail};
use clap::ValueEnum;
use pgschema::{
    parser::{map_builder::MapBuilder, pg_builder::PgBuilder, pgs_builder::PgsBuilder},
    pg::PropertyGraph,
};
use rudof_lib::{
    InputSpec, RudofConfig, data_format::DataFormat,
    shapemap_format::ShapeMapFormat as CliShapeMapFormat,
};
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

#[allow(clippy::too_many_arguments)]
pub fn run_validate_pgschema(
    schema: &Option<InputSpec>,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    shapemap: &Option<InputSpec>,
    _shapemap_format: &CliShapeMapFormat,
    output: &Option<PathBuf>,
    _config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    // let mut rudof = Rudof::new(config)?;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;

    let mut graph = PropertyGraph::new();
    for data in data {
        let mut data_reader = data.open_read(None, "PG data")?;
        let new_graph = get_pg_data(&mut data_reader, data_format)?;
        graph.merge(&new_graph);
    }
    let schema = match schema {
        Some(schema) => schema,
        None => {
            bail!("Schema must be provided for PGSchema validation");
        }
    };
    let mut schema_reader = schema.open_read(None, "PGSchema")?;
    let schema = get_schema(&mut schema_reader)?;
    let shapemap = match shapemap {
        Some(shapemap) => shapemap,
        None => {
            bail!("Type map must be provided for PGSchema validation");
        }
    };
    let mut map_reader = shapemap.open_read(None, "type map")?;
    let type_map = get_map(&mut map_reader)?;
    let result = type_map.validate(&schema, &graph)?;
    write!(writer, "{}", result)?;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Default)]
pub enum PgSchemaResultFormat {
    #[default]
    Internal,
}

impl Display for PgSchemaResultFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PgSchemaResultFormat::Internal => "internal",
        };
        write!(f, "{}", s)
    }
}

fn get_schema<R: Read>(reader: &mut R) -> Result<pgschema::pgs::PropertyGraphSchema> {
    let mut schema_content = String::new();
    reader.read_to_string(&mut schema_content)?;
    let schema: pgschema::pgs::PropertyGraphSchema =
        match PgsBuilder::new().parse_pgs(schema_content.as_str()) {
            Ok(schema) => schema,
            Err(e) => {
                bail!("Failed to parse schema: {}", e);
            }
        };
    Ok(schema)
}

fn get_pg_data<R: Read>(reader: &mut R, _data_format: &DataFormat) -> Result<PropertyGraph> {
    let mut data_content = String::new();
    reader.read_to_string(&mut data_content)?;
    let graph = match PgBuilder::new().parse_pg(data_content.as_str()) {
        Ok(graph) => graph,
        Err(e) => {
            bail!("Failed to parse graph: {}", e);
        }
    };
    Ok(graph)
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
enum PGSchemaValidationFormat {
    #[default]
    Summary,
    Detailed,
}

fn get_map<R: Read>(reader: &mut R) -> Result<pgschema::type_map::TypeMap> {
    let mut map_content = String::new();
    reader
        .read_to_string(&mut map_content)
        .with_context(|| "Failed to read type map content")?;
    let map: pgschema::type_map::TypeMap = match MapBuilder::new().parse_map(map_content.as_str()) {
        Ok(map) => map,
        Err(e) => {
            bail!("Failed to parse type map: {}", e);
        }
    };
    Ok(map)
}
