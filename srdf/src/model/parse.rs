use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use super::rdf::Rdf;
use super::RdfFormat;

/// Reader mode when parsing RDF data files
#[derive(Debug, PartialEq, Clone, Default)]
pub enum ReaderMode {
    /// Stops when there is an error
    #[default]
    Strict,
    /// Emits a warning and continues processing
    Lax,
}

impl ReaderMode {
    pub fn is_strict(&self) -> bool {
        matches!(self, ReaderMode::Strict)
    }
}

pub trait RdfReader: Rdf + Sized + Default {
    type ReaderError: From<std::io::Error>;

    fn merge_from_reader<R: Read>(
        &mut self,
        read: R,
        format: RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), Self::ReaderError>;

    fn from_reader<R: Read>(
        read: R,
        format: RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<Self, Self::ReaderError> {
        let mut graph = Self::default();
        graph.merge_from_reader(read, format, base, reader_mode)?;
        Ok(graph)
    }

    fn merge_from_path<P: AsRef<Path>>(
        &mut self,
        path: P,
        format: RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), Self::ReaderError> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);
        Self::merge_from_reader(self, reader, format, base, reader_mode)?;
        Ok(())
    }

    fn from_path<P: AsRef<Path>>(
        path: P,
        format: RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<Self, Self::ReaderError> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);
        Self::from_reader(reader, format, base, reader_mode)
    }

    fn from_str(
        data: &str,
        format: RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<Self, Self::ReaderError> {
        Self::from_reader(Cursor::new(&data), format, base, reader_mode)
    }

    fn parse_data(
        data: &String,
        format: RdfFormat,
        base: &Path,
        reader_mode: &ReaderMode,
    ) -> Result<Self, Self::ReaderError> {
        let mut attempt = PathBuf::from(base);
        attempt.push(data);
        let base = Some("base:://");
        let data_path = &attempt;
        let graph = Self::from_path(data_path, format, base, reader_mode)?;
        Ok(graph)
    }
}
