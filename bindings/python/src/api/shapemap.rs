use crate::{api::PyRudof, error::Result, formats::PyShapeMapFormat, guard, input::InputArg, output};
use pyo3::prelude::*;
use rudof_lib::formats::ShapeMapFormat;

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Loads a ShapeMap from a string, file path or URL.
    ///
    /// Args:
    ///     input (str | os.PathLike): Inline ShapeMap, file path or URL.
    ///     format (ShapeMapFormat, optional): Format. Defaults to ``ShapeMapFormat.Compact``.
    ///     base_nodes (str, optional): Base IRI for resolving node IRIs.
    ///     base_shapes (str, optional): Base IRI for resolving shape IRIs.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     ShapeMapError: If the ShapeMap is malformed.
    #[pyo3(signature = (input, format = None, base_nodes = None, base_shapes = None))]
    fn read_shapemap(
        &mut self,
        py: Python<'_>,
        input: InputArg,
        format: Option<&PyShapeMapFormat>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
    ) -> Result<()> {
        let InputArg(input) = input;
        let format: Option<ShapeMapFormat> = format.map(Into::into);
        let base_nodes = base_nodes.map(str::to_owned);
        let base_shapes = base_shapes.map(str::to_owned);

        guard::detached(py, move || {
            let mut b = self.inner.load_shapemap(&input);
            if let Some(f) = &format {
                b = b.with_shapemap_format(f);
            }
            if let Some(n) = &base_nodes {
                b = b.with_base_nodes(n);
            }
            if let Some(s) = &base_shapes {
                b = b.with_base_shapes(s);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Serializes the current ShapeMap to a string.
    ///
    /// Args:
    ///     format (ShapeMapFormat, optional): Output format. Defaults to ``ShapeMapFormat.Compact``.
    ///
    /// Returns:
    ///     str: Serialized ShapeMap.
    ///
    /// Raises:
    ///     ShapeMapError: If serialization fails.
    #[pyo3(signature = (format = None))]
    fn serialize_shapemap(&self, py: Python<'_>, format: Option<&PyShapeMapFormat>) -> Result<String> {
        let format: Option<ShapeMapFormat> = format.map(Into::into);
        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_shapemap(w);
            if let Some(f) = &format {
                s = s.with_result_shapemap_format(f);
            }
            s.execute()
        })
    }
}
