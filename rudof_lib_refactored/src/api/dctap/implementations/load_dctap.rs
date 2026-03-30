use dctap::DCTap;

use crate::{
    Result, Rudof,
    errors::DCTapError,
    formats::{DCTapFormat, InputSpec},
};

pub fn load_dctap(rudof: &mut Rudof, dctap: &InputSpec, dctap_format: Option<&DCTapFormat>) -> Result<()> {
    let dctap_format = dctap_format.copied().unwrap_or_default();

    match dctap_format {
        DCTapFormat::Csv => read_dctap_csv(rudof, dctap),
        _ => read_dctap_excel_formats(rudof, dctap),
    }
}

fn read_dctap_csv(rudof: &mut Rudof, dctap: &InputSpec) -> Result<()> {
    let dctap_reader = dctap
        .open_read(None, "PG data")
        .map_err(|error| DCTapError::DataSourceSpec {
            message: format!("Failed to open data source '{}': {error}", dctap.source_name()),
        })?;

    let dctap =
        DCTap::from_reader(dctap_reader, &rudof.config.dctap_config()).map_err(|error| DCTapError::DataSourceSpec {
            message: format!("Failed to read data source '{}': {error}", dctap.source_name()),
        })?;

    rudof.dctap = Some(dctap);

    Ok(())
}

fn read_dctap_excel_formats(rudof: &mut Rudof, dctap: &InputSpec) -> Result<()> {
    match dctap {
        InputSpec::Path(path_buf) => {
            let dctap = DCTap::from_excel(path_buf, None, &rudof.config.dctap_config()).map_err(|error| {
                DCTapError::DataSourceSpec {
                    message: format!("Failed to read data source '{}': {error}", dctap.source_name()),
                }
            })?;

            rudof.dctap = Some(dctap);
        },
        _ => Err(DCTapError::DataSourceSpec {
            message: format!(
                "Failed to read data source '{}'. Excel formats require a file path.",
                dctap.source_name()
            ),
        })?,
    }

    Ok(())
}
