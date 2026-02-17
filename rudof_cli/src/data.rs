use anyhow::Result;
use iri_s::IriS;
use rudof_lib::{InputSpec, Rudof, RudofConfig, data::get_data_rudof, data_format::DataFormat};
use rdf::rdf_core::{
    RDFFormat,
    visualizer::{VisualRDFGraph, uml_converter::{UmlConverter, ImageFormat, UmlGenerationMode}}
};
use rdf::rdf_impl::ReaderMode;
use std::path::PathBuf;

use crate::result_data_format::ResultDataFormat;
use crate::writer::get_writer;

#[allow(clippy::too_many_arguments)]
pub fn run_data(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    base: &Option<IriS>,
    debug: u8,
    output: &Option<PathBuf>,
    result_format: &ResultDataFormat,
    force_overwrite: bool,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config)?;
    if debug > 0 {
        println!("Config: {config:?}")
    }

    get_data_rudof(&mut rudof, data, data_format, base, &None, reader_mode, config, false)?;

    match check_result_format(result_format) {
        CheckResultFormat::RDFFormat(rdf_format) => {
            rudof.get_rdf_data().serialize(&rdf_format, &mut writer)?;
        },
        CheckResultFormat::VisualFormat(VisualFormat::PlantUML) => {
            rudof.data2plant_uml(&mut writer)?;
        },
        CheckResultFormat::VisualFormat(VisualFormat::SVG) | CheckResultFormat::VisualFormat(VisualFormat::PNG) => {
            let rdf = rudof.get_rdf_data();
            let uml_converter = VisualRDFGraph::from_rdf(rdf, config.rdf_data_config().rdf_visualization_config())?;
            let format = match result_format {
                ResultDataFormat::Svg => ImageFormat::SVG,
                ResultDataFormat::Png => ImageFormat::PNG,
                _ => unreachable!(),
            };
            uml_converter.as_image(&mut writer, format, &UmlGenerationMode::all(), config.plantuml_path())?;
        },
    }
    Ok(())
}

#[allow(clippy::upper_case_acronyms)]
enum VisualFormat {
    PlantUML,
    SVG,
    PNG,
}

enum CheckResultFormat {
    RDFFormat(RDFFormat),
    VisualFormat(VisualFormat),
}

fn check_result_format(format: &ResultDataFormat) -> CheckResultFormat {
    match format {
        ResultDataFormat::Turtle => CheckResultFormat::RDFFormat(RDFFormat::Turtle),
        ResultDataFormat::N3 => CheckResultFormat::RDFFormat(RDFFormat::N3),
        ResultDataFormat::NTriples => CheckResultFormat::RDFFormat(RDFFormat::NTriples),
        ResultDataFormat::RdfXml => CheckResultFormat::RDFFormat(RDFFormat::RDFXML),
        ResultDataFormat::TriG => CheckResultFormat::RDFFormat(RDFFormat::TriG),
        ResultDataFormat::NQuads => CheckResultFormat::RDFFormat(RDFFormat::NQuads),
        ResultDataFormat::PlantUML => CheckResultFormat::VisualFormat(VisualFormat::PlantUML),
        ResultDataFormat::Svg => CheckResultFormat::VisualFormat(VisualFormat::SVG),
        ResultDataFormat::Png => CheckResultFormat::VisualFormat(VisualFormat::PNG),
        _ => todo!(),
    }
}
