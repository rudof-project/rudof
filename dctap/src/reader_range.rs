use std::marker::PhantomData;

use calamine::{Data, DataType, Range};
use csv::{Position, StringRecord};
use tracing::debug;

pub struct ReaderRange<R> {
    range: Range<Data>,
    marker: PhantomData<R>,
    current_line: usize,
    position: Position,
}

impl<R> ReaderRange<R> {
    pub fn new(range: Range<Data>) -> ReaderRange<R> {
        ReaderRange {
            range,
            marker: PhantomData,
            position: Position::new(),
            current_line: 0,
        }
    }

    pub fn next_record(&mut self) -> Option<StringRecord> {
        if self.current_line == self.range.height() {
            None
        } else {
            let mut rcd = StringRecord::new();
            for column in 0..self.range.width() {
                let row = self.current_line as u32;
                if let Some(data) = self.range.get_value((row, column as u32)) {
                    if let Some(str) = data.as_string() {
                        rcd.push_field(&str);
                    } else {
                        debug!("Processing excel, data can not converted to string: {data:?} at ({row},{column})");
                        rcd.push_field("");
                    }
                } else {
                    rcd.push_field("");
                }
            }
            self.current_line += 1;
            Some(rcd)
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}
