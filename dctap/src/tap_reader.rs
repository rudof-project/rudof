use crate::tap_error::Result;
use crate::TapShape;
use csv::{Reader, StringRecord};
use std::io::{self};

pub struct TapReader<R> {
    reader: Reader<R>,
    state: TapReaderState,
}

impl<R: io::Read> TapReader<R> {
    pub fn from_reader(reader: Reader<R>) -> TapReader<R> {
        TapReader {
            reader,
            state: TapReaderState::new(),
        }
    }

    pub fn shapes(&mut self) -> ShapesIter<R> {
        ShapesIter::new(self)
    }

    pub fn read_shape(&mut self) -> Result<bool> {
        let mut record = StringRecord::new();
        if self.reader.read_record(&mut record)? {
            self.state.current_shape.from_record(record)?;
            Ok(self.reader.is_done())
        } else {
            todo!() // Err())
        }
    }
}

#[derive(Debug)]
struct TapReaderState {
    current_shape: TapShape,
}

impl TapReaderState {
    pub fn new() -> TapReaderState {
        TapReaderState {
            current_shape: TapShape::new(),
        }
    }
}
/// A borrowed iterator over Shapes
///
/// The lifetime parameter `'r` refers to the lifetime of the underlying `TapReader`.
pub struct ShapesIter<'r, R: 'r> {
    rdr: &'r mut TapReader<R>,
    shape: TapShape,
}

impl<'r, R: io::Read> ShapesIter<'r, R> {
    fn new(rdr: &'r mut TapReader<R>) -> ShapesIter<'r, R> {
        ShapesIter {
            rdr,
            shape: TapShape::new(),
        }
    }

    /// Return a reference to the underlying `TapReader`.
    pub fn reader(&self) -> &TapReader<R> {
        &self.rdr
    }

    /// Return a mutable reference to the underlying `TapReader`.
    pub fn reader_mut(&mut self) -> &mut TapReader<R> {
        &mut self.rdr
    }
}

impl<'r, R: io::Read> Iterator for ShapesIter<'r, R> {
    type Item = Result<TapShape>;

    fn next(&mut self) -> Option<Result<TapShape>> {
        match self.rdr.read_shape() {
            Err(err) => Some(Err(err)),
            Ok(true) => Some(Ok(self.shape.clone())),
            Ok(false) => None,
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
city;country;pop
Boston;United States;4628910
";
        let mut reader = TapReader::from_reader(Reader::from_reader(data.as_bytes()));
        let expected_shape = TapShape::new();
        let next_shape = reader.shapes().next().unwrap().unwrap();
        assert_eq!(next_shape, expected_shape);
    }
}
