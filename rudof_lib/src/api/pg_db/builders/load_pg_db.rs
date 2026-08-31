use crate::{
    Result, Rudof,
    api::pg_db::PgDbOperations,
    formats::{DataFormat, DataReaderMode, InputSpec, ShaclFormat},
};
use std::io;
use std::path::Path;

/// Builder for the `load_pg_db` operation.
pub struct LoadPgDbBuilder<'a, W: io::Write> {
    rudof: &'a mut Rudof,
    data: &'a [InputSpec],
    writer: &'a mut W,
    db_path: Option<&'a Path>,
    db_read_only: bool,
    shapes: Option<&'a InputSpec>,
    shapes_format: Option<&'a ShaclFormat>,
    base_shapes: Option<&'a str>,
    skip_validation: bool,
    data_format: Option<&'a DataFormat>,
    base_data: Option<&'a str>,
    reader_mode: Option<&'a DataReaderMode>,
}

impl<'a, W: io::Write> LoadPgDbBuilder<'a, W> {
    /// Creates a new builder.
    ///
    /// Internal helper called by `Rudof::load_pg_db()`; not intended for
    /// public construction by callers.
    pub(crate) fn new(rudof: &'a mut Rudof, data: &'a [InputSpec], writer: &'a mut W) -> Self {
        Self {
            rudof,
            data,
            writer,
            db_path: None,
            db_read_only: false,
            shapes: None,
            shapes_format: None,
            base_shapes: None,
            skip_validation: false,
            data_format: None,
            base_data: None,
            reader_mode: None,
        }
    }

    /// Override the database to load into (otherwise the connection info
    /// stored by a prior `connect_pg_db` call is used).
    pub fn with_db(mut self, path: &'a Path, read_only: bool) -> Self {
        self.db_path = Some(path);
        self.db_read_only = read_only;
        self
    }

    /// Set the SHACL shapes to validate against (otherwise shapes embedded
    /// in the data itself are used).
    pub fn with_shapes(mut self, shapes: &'a InputSpec) -> Self {
        self.shapes = Some(shapes);
        self
    }

    /// Set the SHACL shapes format.
    pub fn with_shapes_format(mut self, shapes_format: &'a ShaclFormat) -> Self {
        self.shapes_format = Some(shapes_format);
        self
    }

    /// Set the base IRI for the shapes.
    pub fn with_base_shapes(mut self, base_shapes: &'a str) -> Self {
        self.base_shapes = Some(base_shapes);
        self
    }

    /// Skip SHACL validation and just copy the data.
    pub fn with_skip_validation(mut self, skip_validation: bool) -> Self {
        self.skip_validation = skip_validation;
        self
    }

    /// Set the RDF data format.
    pub fn with_data_format(mut self, data_format: &'a DataFormat) -> Self {
        self.data_format = Some(data_format);
        self
    }

    /// Set the base IRI for the data.
    pub fn with_base_data(mut self, base_data: &'a str) -> Self {
        self.base_data = Some(base_data);
        self
    }

    /// Set the RDF reader mode.
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Execute the `load_pg_db` operation with the configured parameters,
    /// returning the (node count, relationship count) inserted.
    pub fn execute(self) -> Result<(usize, usize)> {
        <Rudof as PgDbOperations>::load_pg_db(
            self.rudof,
            self.data,
            self.db_path,
            self.db_read_only,
            self.shapes,
            self.shapes_format,
            self.base_shapes,
            self.skip_validation,
            self.data_format,
            self.base_data,
            self.reader_mode,
            self.writer,
        )
    }
}
