use crate::writer::get_writer;
use crate::{InputCompareFormat, input_compare_mode::InputCompareMode, result_compare_format::ResultCompareFormat};
use anyhow::{Context, Result, bail};
use iri_s::IriS;
use iri_s::MimeType;
use rdf::rdf_impl::ReaderMode;
use rudof_lib::{InputSpec, Rudof, RudofConfig};
use shapes_comparator::{CoShaMo, CoShaMoConverter, ComparatorConfig};
use shex_ast::Schema;
use std::path::PathBuf;
use tracing::debug;

#[allow(clippy::too_many_arguments)]
pub fn run_compare(
    input1: &InputSpec,
    format1: &InputCompareFormat,
    mode1: &InputCompareMode,
    base1: &Option<IriS>,
    label1: Option<&str>,
    input2: &InputSpec,
    format2: &InputCompareFormat,
    mode2: &InputCompareMode,
    base2: &Option<IriS>,
    label2: Option<&str>,
    reader_mode: &ReaderMode,
    output: &Option<PathBuf>,
    result_format: &ResultCompareFormat,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let mut reader1 = input1.open_read(Some(format1.mime_type()), "Compare1")?;
    let mut reader2 = input2.open_read(Some(format2.mime_type()), "Compare2")?;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config)?;
    let coshamo1 = get_coshamo(
        &mut rudof,
        mode1,
        format1,
        base1,
        label1,
        &mut reader1,
        reader_mode,
        Some(&input1.source_name()),
    )?;
    let coshamo2 = get_coshamo(
        &mut rudof,
        mode2,
        format2,
        base2,
        label2,
        &mut reader2,
        reader_mode,
        Some(&input2.source_name()),
    )?;
    let shaco = coshamo1.compare(&coshamo2);
    match result_format {
        ResultCompareFormat::Internal => {
            writeln!(writer, "{shaco}")?;
            Ok(())
        },
        ResultCompareFormat::Json => {
            let str =
                serde_json::to_string_pretty(&shaco).context(format!("Error converting Result to JSON: {shaco}"))?;
            writeln!(writer, "{str}")?;
            Ok(())
        },
    }
}

#[allow(clippy::too_many_arguments)]
pub fn get_coshamo(
    rudof: &mut Rudof,
    mode: &InputCompareMode,
    format: &InputCompareFormat,
    base: &Option<IriS>,
    label: Option<&str>,
    reader: &mut dyn std::io::Read,
    reader_mode: &ReaderMode,
    source_name: Option<&str>,
) -> Result<CoShaMo> {
    match mode {
        InputCompareMode::Shacl => bail!("Not yet implemented comparison between SHACL schemas"),
        InputCompareMode::ShEx => {
            let shex = read_shex(rudof, format, base, reader, reader_mode, source_name)?;
            let mut converter = CoShaMoConverter::new(&ComparatorConfig::new());
            let coshamo = converter.populate_from_shex(&shex, label)?;
            Ok(coshamo)
        },
        InputCompareMode::Dctap => bail!("Not yet implemented comparison between DCTAP files"),
        InputCompareMode::Service => {
            bail!("Not yet implemented comparison between Service descriptions")
        },
    }
}

pub fn read_shex(
    rudof: &mut Rudof,
    format: &InputCompareFormat,
    base: &Option<IriS>,
    reader: &mut dyn std::io::Read,
    reader_mode: &ReaderMode,
    source_name: Option<&str>,
) -> Result<Schema> {
    let shex_format1 = format
        .to_shex_format()
        .unwrap_or_else(|_| panic!("ShEx format1 {format}"));
    let base = base.as_ref().map(|iri| iri.as_str());
    rudof.read_shex(reader, &shex_format1, base, reader_mode, source_name)?;
    if let Some(schema) = rudof.get_shex() {
        debug!("Schema read: {schema}");
        Ok(schema.clone())
    } else {
        bail!(
            "Error reading ShEx {} with format {format}",
            source_name.unwrap_or("unknown")
        )
    }
}
