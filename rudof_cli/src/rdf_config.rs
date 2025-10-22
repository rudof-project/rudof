use crate::writer::get_writer;
use clap::ValueEnum;
use rudof_lib::{InputSpec, Rudof, RudofConfig};
use std::fmt::Display;
use std::path::PathBuf;

pub fn run_rdf_config(
    input: &InputSpec,
    _format: &RdfConfigFormat,
    output: &Option<PathBuf>,
    result_format: &RdfConfigResultFormat,
    config: &RudofConfig,
    force_overwrite: bool,
) -> anyhow::Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config)?;
    let reader = input.open_read(None, "rdf-config")?;
    rudof.read_rdf_config(reader, input.to_string())?;
    if let Some(rdf_config) = rudof.get_rdf_config() {
        rdf_config.serialize(cnv_rdf_config_format(result_format), &mut writer)?;
    } else {
        writeln!(writer, "{{\"error\": \"No RDF Config read\"}}")?;
    }
    Ok(())
}

fn cnv_rdf_config_format(format: &RdfConfigResultFormat) -> &rdf_config::RdfConfigFormat {
    match format {
        RdfConfigResultFormat::Yaml => &rdf_config::RdfConfigFormat::Yaml,
        RdfConfigResultFormat::Internal => &rdf_config::RdfConfigFormat::Yaml,
    }
}

#[derive(Clone, Debug, Default, PartialEq, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum RdfConfigFormat {
    #[default]
    Yaml,
}

impl Display for RdfConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdfConfigFormat::Yaml => write!(f, "yaml"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum RdfConfigResultFormat {
    #[default]
    Internal,
    Yaml,
}

impl Display for RdfConfigResultFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdfConfigResultFormat::Yaml => write!(f, "yaml"),
            RdfConfigResultFormat::Internal => write!(f, "internal"),
        }
    }
}
