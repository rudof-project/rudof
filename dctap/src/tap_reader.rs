use crate::TapShape;
use crate::{tap_error::Result, tap_headers::TapHeaders};
use csv::{Reader, StringRecord};
use std::io::{self};

pub struct TapReader<'r, R> {
    reader: &'r mut Reader<R>,
    state: TapReaderState,
}

impl<'r, R: io::Read> TapReader<'r, R> {
    pub fn from_reader(reader: &mut Reader<R>) -> Result<TapReader<R>> {
        let rcd_headers = reader.headers()?;
        let headers = TapHeaders::from_record(rcd_headers)?;
        println!("Headers: {headers:?}");
        Ok(TapReader {
            reader,
            state: TapReaderState::new().with_headers(headers),
        })
    }

    pub fn shapes(&'r mut self) -> ShapesIter<'r, R> {
        ShapesIter::new(self)
    }

    pub fn read_shape(&mut self) -> Result<bool> {
        let mut record = StringRecord::new();
        if self.reader.read_record(&mut record)? {
            self.state
                .current_shape
                .from_record(record, &self.state.headers)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug)]
struct TapReaderState {
    current_shape: TapShape,
    headers: TapHeaders,
}

impl TapReaderState {
    pub fn new() -> TapReaderState {
        TapReaderState {
            current_shape: TapShape::new(),
            headers: TapHeaders::new(),
        }
    }

    pub fn with_headers(mut self, headers: TapHeaders) -> Self {
        self.headers = headers;
        self
    }
}
/// A borrowed iterator over Shapes
///
/// The lifetime parameter `'r` refers to the lifetime of the underlying `TapReader`.
pub struct ShapesIter<'r, R: 'r> {
    reader: &'r mut TapReader<'r, R>,
}

impl<'r, R: io::Read> ShapesIter<'r, R> {
    fn new(reader: &'r mut TapReader<'r, R>) -> ShapesIter<'r, R> {
        ShapesIter { reader }
    }

    /// Return a reference to the underlying `TapReader`.
    pub fn reader(&self) -> &TapReader<R> {
        &self.reader
    }

    /// Return a mutable reference to the underlying `TapReader`.
    pub fn reader_mut(&mut self) -> &mut TapReader<'r, R> {
        &mut self.reader
    }
}

impl<'r, R: io::Read> Iterator for ShapesIter<'r, R> {
    type Item = Result<TapShape>;

    fn next(&mut self) -> Option<Result<TapShape>> {
        match self.reader.read_shape() {
            Err(err) => Some(Err(err)),
            Ok(true) => {
                println!("Next shape with true");
                Some(Ok(self.reader.state.current_shape.clone()))
            }
            Ok(false) => {
                println!("Next shape with false...");
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TapShape;

    use super::*;

    #[test]
    fn test_simple() {
        let data = "\
shapeId,shapeLabel,propertyId,propertyLabel
Person,PersonLabel,knows,KnowsLabel
";
        let mut reader = Reader::from_reader(data.as_bytes());
        let mut tap_reader = TapReader::from_reader(&mut reader).unwrap();
        let expected_shape = TapShape::new().with_shape_id("Person");
        let next_shape = tap_reader.shapes().next().unwrap().unwrap();
        println!("next_shape: {next_shape:?}");
        assert_eq!(next_shape, expected_shape);
    }
}
