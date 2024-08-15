use std::collections::{hash_map::Entry, HashMap};

use crate::TapShape;
use crate::{tap_headers::TapHeaders, TapReaderWarning};
use csv::{Position, StringRecord};

#[derive(Debug)]
pub struct TapReaderState {
    current_shape: TapShape,
    cached_next_record: Option<(StringRecord, Position)>,
    headers: TapHeaders,
    placeholder_ids: HashMap<String, u64>,
    warnings: Vec<TapReaderWarning>,
}

impl TapReaderState {
    pub fn new() -> TapReaderState {
        TapReaderState {
            current_shape: TapShape::new(0),
            cached_next_record: None,
            headers: TapHeaders::new(),
            placeholder_ids: HashMap::new(),
            warnings: Vec::new(),
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

    pub fn set_next_record(&mut self, rcd: &StringRecord, pos: &Position) -> &mut Self {
        self.cached_next_record = Some((rcd.clone(), pos.clone()));
        self
    }

    pub fn reset_next_record(&mut self) -> &mut Self {
        self.cached_next_record = None;
        self
    }

    pub fn get_cached_next_record(&mut self) -> Option<(&StringRecord, &Position)> {
        if let Some((rcd, pos)) = &self.cached_next_record {
            Some((rcd, pos))
        } else {
            None
        }
    }

    // Get a value for placeholder_id and increment its counter
    pub fn placeholder_id(&mut self, str: &str) -> u64 {
        match self.placeholder_ids.entry(str.to_string()) {
            Entry::Occupied(mut r) => {
                let v = r.get_mut();
                *v += 1;
                *v
            }
            Entry::Vacant(v) => {
                let initial = 0;
                v.insert(initial);
                initial
            }
        }
    }

    pub fn warnings(&self) -> impl Iterator<Item = &TapReaderWarning> {
        self.warnings.iter()
    }

    pub fn add_warning(&mut self, warning: TapReaderWarning) {
        self.warnings.push(warning)
    }
}

impl Default for TapReaderState {
    fn default() -> Self {
        Self::new()
    }
}
