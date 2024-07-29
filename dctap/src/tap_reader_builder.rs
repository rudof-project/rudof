use crate::{tap_error::Result, tap_headers::TapHeaders};
use crate::{
    BasicNodeType, DatatypeId, NodeType, PropertyId, ShapeId, TapConfig, TapError, TapReader,
    TapReaderState, TapShape, TapStatement, Value, ValueConstraint, ValueConstraintType,
};
use csv::{Position, Reader, ReaderBuilder, StringRecord, Terminator, Trim};
use std::fs::File;
// use indexmap::IndexSet;
use std::io::{self};
use std::path::Path;

#[derive(Default)]
pub struct TapReaderBuilder {
    reader_builder: ReaderBuilder,
}

impl TapReaderBuilder {
    pub fn new() -> TapReaderBuilder {
        TapReaderBuilder::default()
    }

    // Most of these options are copied from CSV Rust
    pub fn flexible(mut self, yes: bool) -> Self {
        self.reader_builder.flexible(yes);
        self
    }

    pub fn trim(&mut self, trim: Trim) -> &mut TapReaderBuilder {
        self.reader_builder.trim(trim);
        self
    }

    pub fn terminator(&mut self, term: Terminator) -> &mut TapReaderBuilder {
        self.reader_builder.terminator(term);
        self
    }

    pub fn quote(&mut self, quote: u8) -> &mut TapReaderBuilder {
        self.reader_builder.quote(quote);
        self
    }

    pub fn delimiter(&mut self, delimiter: u8) -> &mut TapReaderBuilder {
        self.reader_builder.delimiter(delimiter);
        self
    }

    pub fn from_path<P: AsRef<Path>>(&self, path: P, config: TapConfig) -> Result<TapReader<File>> {
        let mut reader = self.reader_builder.from_path(path)?;
        let rcd_headers = reader.headers()?;
        let headers = TapHeaders::from_record(rcd_headers)?;
        let state = TapReaderState::new().with_headers(headers);
        Ok(TapReader::new(reader, state, config))
    }

    pub fn from_reader<R: io::Read>(&mut self, rdr: R, config: TapConfig) -> Result<TapReader<R>> {
        let mut reader = self.reader_builder.from_reader(rdr);
        let rcd_headers = reader.headers()?;
        let headers = TapHeaders::from_record(rcd_headers)?;
        let state = TapReaderState::new().with_headers(headers);
        Ok(TapReader::new(reader, state, config))
    }
}
