use crate::{tap_error::Result, tap_headers::TapHeaders};
use crate::{ReaderRange, TapConfig, TapError, TapReader, TapReaderState};
use calamine::{open_workbook, Data, DataType, Range, Reader as XlsxReader, Xls, Xlsx};
use csv::{ReaderBuilder, StringRecord};
use std::fs::File;
// use indexmap::IndexSet;
use std::io::{self, BufReader};
use std::path::Path;

#[derive(Default)]
pub struct TapReaderBuilder {
    _reader_builder: ReaderBuilder,
}

impl TapReaderBuilder {
    pub fn new() -> TapReaderBuilder {
        TapReaderBuilder::default()
    }
    /*
        // Most of these options are copied from CSV Rust
        pub fn _flexible(mut self, yes: bool) -> Self {
            self.reader_builder.flexible(yes);
            self
        }

        pub fn _trim(&mut self, trim: Trim) -> &mut TapReaderBuilder {
            self.reader_builder.trim(trim);
            self
        }

        pub fn _terminator(&mut self, term: Terminator) -> &mut TapReaderBuilder {
            self.reader_builder.terminator(term);
            self
        }

        pub fn _quote(&mut self, quote: u8) -> &mut TapReaderBuilder {
            self.reader_builder.quote(quote);
            self
        }

        pub fn _delimiter(&mut self, delimiter: u8) -> &mut TapReaderBuilder {
            self.reader_builder.delimiter(delimiter);
            self
        }
    */
    /// Build a TapReader from a path and a `TapConfig`
    ///
    /// # Example
    /// ```no_run
    /// use dctap::TapReaderBuilder;
    /// use dctap::TapConfig;
    /// use std::error::Error;
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Box<dyn Error>> {
    ///     let mut tap = TapReaderBuilder::from_path("foo.csv", &TapConfig::default())?;
    ///     for result in tap.shapes() {
    ///         let shape = result?;
    ///         println!("{:?}", shape);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P, config: &TapConfig) -> Result<TapReader<File>> {
        let mut reader = ReaderBuilder::new()
            .delimiter(config.delimiter())
            .quote(config.quote())
            .flexible(config.flexible())
            .from_path(path)?;
        let rcd_headers = reader.headers()?;
        let headers = TapHeaders::from_record(rcd_headers)?;
        let state = TapReaderState::new().with_headers(headers);
        Ok(TapReader::new_csv_reader(reader, state, config))
    }

    pub fn from_reader<R: io::Read>(rdr: R, config: &TapConfig) -> Result<TapReader<R>> {
        let mut reader = ReaderBuilder::new()
            .delimiter(config.delimiter())
            .quote(config.quote())
            .flexible(config.flexible())
            .from_reader(rdr);
        let rcd_headers = reader.headers()?;
        let headers = TapHeaders::from_record(rcd_headers)?;
        let state = TapReaderState::new().with_headers(headers);
        Ok(TapReader::new_csv_reader(reader, state, config))
    }

    pub fn from_excel<R: io::Read, P: AsRef<Path>>(
        path: P,
        sheet_name: Option<&str>,
        config: &TapConfig,
    ) -> Result<TapReader<R>> {
        let path_name = path.as_ref().to_string_lossy().to_string();
        let mut excel: Xlsx<_> = match open_workbook(path) {
            Ok(xls) => Ok::<calamine::Xlsx<BufReader<File>>, TapError>(xls),
            Err(e) => todo!(),
        }?;
        let mut range = match sheet_name {
            None => match excel.worksheet_range_at(0) {
                Some(range) => range,
                None => todo!(),
            },
            Some(name) => excel.worksheet_range(name),
        }?;
        let reader_range: ReaderRange<io::Empty> = ReaderRange::new(range);
        if let Some(rcd) = range_to_record_headers(reader_range) {
            let headers = TapHeaders::from_record(&rcd)?;
            println!("Headers: {headers:?}");
            let state = TapReaderState::new().with_headers(headers);
            Ok(TapReader::new_range_reader(range, state, config))
        } else {
            Err(TapError::NoHeadersExcel { path: path_name })
        }
    }
}

fn range_to_record_headers<R>(range: &mut ReaderRange<R>) -> Option<StringRecord> {
    if let Some(row) = range.rows().next() {
        let mut rcd = StringRecord::new();
        for cell in row {
            if let Some(str) = cell.as_string() {
                rcd.push_field(&str);
            } else {
                rcd.push_field("");
            }
        }
        Some(rcd)
    } else {
        None
    }
}
