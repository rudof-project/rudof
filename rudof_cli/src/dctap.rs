use crate::DCTapResultFormat;
use crate::dctap_format::DCTapFormat as CliDCTapFormat;
use crate::writer::get_writer;
use anyhow::{Context, Result, bail};
use dctap::DCTAPFormat;
use rudof_lib::InputSpec;
use rudof_lib::Rudof;
use rudof_lib::RudofConfig;
use std::path::PathBuf;

pub fn run_dctap(
    input: &InputSpec,
    format: &CliDCTapFormat,
    result_format: &DCTapResultFormat,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config)?;
    parse_dctap(&mut rudof, input, format)?;
    if let Some(dctap) = rudof.get_dctap() {
        match result_format {
            DCTapResultFormat::Internal => {
                writeln!(writer, "{dctap}")?;
                Ok(())
            },
            DCTapResultFormat::Json => {
                let str = serde_json::to_string_pretty(&dctap).context("Error converting DCTap to JSON: {dctap}")?;
                writeln!(writer, "{str}")?;
                Ok(())
            },
        }
    } else {
        bail!("Internal error: No DCTAP read")
    }
}

pub fn parse_dctap(rudof: &mut Rudof, input: &InputSpec, format: &CliDCTapFormat) -> Result<()> {
    let dctap_format = match format {
        CliDCTapFormat::Csv => DCTAPFormat::Csv,
        CliDCTapFormat::Xlsx => DCTAPFormat::Xlsx,
        CliDCTapFormat::Xlsb => DCTAPFormat::Xlsb,
        CliDCTapFormat::Xlsm => DCTAPFormat::Xlsm,
        CliDCTapFormat::Xls => DCTAPFormat::Xls,
    };
    match format {
        CliDCTapFormat::Csv => {
            let reader = input.open_read(None, "DCTAP")?;
            rudof.read_dctap(reader, &dctap_format)?;
            Ok(())
        },
        _ => match input {
            InputSpec::Path(path_buf) => {
                rudof.read_dctap_path(path_buf, &dctap_format)?;
                Ok(())
            },
            InputSpec::Stdin => bail!("Can not read Excel file from stdin"),
            InputSpec::Url(_) => bail!("Not implemented reading Excel files from URIs yet"),
            InputSpec::Str(_) => {
                bail!("Not implemented reading Excel files from strings yet")
            },
        },
    }
}
