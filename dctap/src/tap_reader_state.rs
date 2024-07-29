use crate::{tap_error::Result, tap_headers::TapHeaders};
use crate::{
    BasicNodeType, DatatypeId, NodeType, PropertyId, ShapeId, TapConfig, TapError, TapShape,
    TapStatement, Value, ValueConstraint, ValueConstraintType,
};
use csv::{Position, Reader, ReaderBuilder, StringRecord, Terminator, Trim};
use std::fs::File;
// use indexmap::IndexSet;
use std::io::{self};
use std::path::Path;

#[derive(Debug)]
pub(crate) struct TapReaderState {
    current_shape: TapShape,
    cached_next_record: Option<StringRecord>,
    headers: TapHeaders,
    _position: Position,
}

impl TapReaderState {
    pub fn new() -> TapReaderState {
        TapReaderState {
            current_shape: TapShape::new(),
            cached_next_record: None,
            headers: TapHeaders::new(),
            _position: Position::new(),
        }
    }

    pub fn current_shape(&mut self) -> &mut TapShape {
        &mut self.current_shape
    }

    pub fn headers(&self) -> &TapHeaders {
        &self.headers
    }

    pub fn with_headers(mut self, headers: TapHeaders) -> Self {
        self.headers = headers;
        self
    }

    pub fn set_next_record(&mut self, rcd: &StringRecord) -> &mut Self {
        self.cached_next_record = Some(rcd.clone());
        self
    }

    pub fn reset_next_record(&mut self) -> &mut Self {
        self.cached_next_record = None;
        self
    }

    pub fn get_cached_next_record(&mut self) -> Option<&StringRecord> {
        if let Some(rcd) = &self.cached_next_record {
            Some(rcd)
        } else {
            None
        }
    }
}
