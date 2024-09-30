use std::marker::PhantomData;

use calamine::{Data, DataType, Range};
use csv::{Position, StringRecord};

pub struct ReaderRange<R> {
    range: Range<Data>,
    marker: PhantomData<R>,
    position: Position,
}

impl<R> ReaderRange<R> {
    pub fn new(range: Range<Data>) -> ReaderRange<R> {
        ReaderRange {
            range,
            marker: PhantomData,
            position: Position::new(),
        }
    }

    pub fn iter(&mut self) -> impl Iterator<Item = StringRecord> + '_ {
        self.range.rows().map(cnv_row)
    }

    pub fn read_record(&mut self, record: &mut StringRecord) -> bool {
        if let Some(row) = self.range.rows().next() {
            record.clear();
            for cell in row {
                if let Some(str) = cell.as_string() {
                    record.push_field(&str);
                } else {
                    record.push_field("");
                }
            }
            true
        } else {
            false
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}

fn cnv_row(row: &[Data]) -> StringRecord {
    let mut rcd = StringRecord::new();
    for cell in row {
        if let Some(str) = cell.as_string() {
            rcd.push_field(&str);
        } else {
            rcd.push_field("");
        }
    }
    rcd
}
