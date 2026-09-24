use crate::{
    api::PyRudof,
    error::Result,
    formats::{
        PyConversionFormat, PyConversionMode, PyReaderMode, PyResultConversionFormat,
        PyResultConversionMode,
    },
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::formats::{
    ComparisonFormat, ComparisonMode, ConversionFormat, ConversionMode, DataReaderMode,
    ResultConversionFormat, ResultConversionMode,
};
use std::{path::PathBuf, str::FromStr};

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Compares two schemas for structural equivalence.
    ///
    /// Converts both schemas to the Common Shapes Model and performs a structural comparison.
    ///
    /// Args:
    ///     schema1 (str | os.PathLike): First schema — inline, file path or URL.
    ///     schema2 (str | os.PathLike): Second schema — inline, file path or URL.
    ///     mode1 (str): First schema type, e.g. ``"shex"``, ``"shacl"``, ``"dctap"``, ``"service"``.
    ///     mode2 (str): Second schema type.
    ///     format1 (str): First schema format, e.g. ``"shexc"``, ``"turtle"``.
    ///     format2 (str): Second schema format.
    ///     base1 (str, optional): Base IRI for the first schema.
    ///     base2 (str, optional): Base IRI for the second schema.
    ///     label1 (str, optional): Shape label to compare in the first schema.
    ///     label2 (str, optional): Shape label to compare in the second schema.
    ///     reader_mode (ReaderMode, optional): Error handling. Defaults to ``ReaderMode.Lax``.
    ///
    /// Returns:
    ///     str: Comparison result showing the differences.
    ///
    /// Raises:
    ///     ComparisonError: If a mode or format name is not recognized, or the comparison fails.
    ///     InputError: If either schema cannot be resolved.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (schema1, schema2, mode1, mode2, format1, format2, base1 = None,
                        base2 = None, label1 = None, label2 = None, reader_mode = None))]
    fn compare_schemas(
        &mut self,
        py: Python<'_>,
        schema1: InputArg,
        schema2: InputArg,
        mode1: &str,
        mode2: &str,
        format1: &str,
        format2: &str,
        base1: Option<&str>,
        base2: Option<&str>,
        label1: Option<&str>,
        label2: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
    ) -> Result<String> {
        let InputArg(schema1) = schema1;
        let InputArg(schema2) = schema2;

        let format1 = ComparisonFormat::from_str(format1)?;
        let format2 = ComparisonFormat::from_str(format2)?;
        let mode1 = ComparisonMode::from_str(mode1)?;
        let mode2 = ComparisonMode::from_str(mode2)?;
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let base1 = base1.map(str::to_owned);
        let base2 = base2.map(str::to_owned);
        let label1 = label1.map(str::to_owned);
        let label2 = label2.map(str::to_owned);

        output::capture_string_detached(py, move |w| {
            let mut c = self.inner.show_schema_comparison(
                &schema1, &schema2, &format1, &format2, &mode1, &mode2, w,
            );
            if let Some(m) = &reader_mode {
                c = c.with_reader_mode(m);
            }
            if let Some(b) = &base1 {
                c = c.with_base1(b);
            }
            if let Some(b) = &base2 {
                c = c.with_base2(b);
            }
            if let Some(l) = &label1 {
                c = c.with_shape1(l);
            }
            if let Some(l) = &label2 {
                c = c.with_shape2(l);
            }
            c.execute()
        })
    }

    /// Converts a schema from one language to another.
    ///
    /// Args:
    ///     schema (str | os.PathLike): Schema to convert — inline, file path or URL.
    ///     input_mode (ConversionMode): The language of the input schema.
    ///     output_mode (ResultConversionMode): The target language.
    ///     input_format (ConversionFormat): The serialization of the input schema.
    ///     output_format (ResultConversionFormat): The serialization of the output.
    ///     base (str, optional): Base IRI for resolving relative IRIs.
    ///     reader_mode (ReaderMode, optional): Error handling. Defaults to ``ReaderMode.Lax``.
    ///     shape (str, optional): Restrict the conversion to a single shape.
    ///     templates_folder (str | os.PathLike, optional): Folder of templates used by the HTML output mode.
    ///     output_folder (str | os.PathLike, optional): Folder to write multi-file output into.
    ///
    /// Returns:
    ///     str: The converted schema.
    ///
    /// Raises:
    ///     InputError: If the schema cannot be resolved.
    ///     ConversionError: If the conversion fails.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (schema, input_mode, output_mode, input_format, output_format,
                        base = None, reader_mode = None, shape = None,
                        templates_folder = None, output_folder = None))]
    fn convert_schemas(
        &mut self,
        py: Python<'_>,
        schema: InputArg,
        input_mode: &PyConversionMode,
        output_mode: &PyResultConversionMode,
        input_format: &PyConversionFormat,
        output_format: &PyResultConversionFormat,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
        shape: Option<&str>,
        templates_folder: Option<PathBuf>,
        output_folder: Option<PathBuf>,
    ) -> Result<String> {
        let InputArg(schema) = schema;
        let input_mode: ConversionMode = input_mode.into();
        let output_mode: ResultConversionMode = output_mode.into();
        let input_format: ConversionFormat = input_format.into();
        let output_format: ResultConversionFormat = output_format.into();
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let base = base.map(str::to_owned);
        let shape = shape.map(str::to_owned);

        output::capture_string_detached(py, move |w| {
            let mut c = self.inner.show_schema_conversion(
                &schema,
                &input_mode,
                &output_mode,
                &input_format,
                &output_format,
                w,
            );
            if let Some(b) = &base {
                c = c.with_base(b);
            }
            if let Some(m) = &reader_mode {
                c = c.with_reader_mode(m);
            }
            if let Some(s) = &shape {
                c = c.with_shape(s);
            }
            if let Some(f) = &templates_folder {
                c = c.with_templates_folder(f);
            }
            if let Some(f) = &output_folder {
                c = c.with_output_folder(f);
            }
            c.execute()
        })
    }
}
