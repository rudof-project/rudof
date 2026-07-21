use crate::{SchemaIRError, ir::schema_ir::SchemaIR};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Read, Write};

pub const CACHE_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheReaderMode {
    Strict,
    Lax,
}

impl CacheReaderMode {
    pub fn is_strict(&self) -> bool {
        matches!(self, CacheReaderMode::Strict)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CacheFormat {
    #[default]
    Bincode,
}

impl CacheFormat {
    fn as_str(&self) -> &'static str {
        match self {
            CacheFormat::Bincode => "bincode",
        }
    }

    pub(crate) fn write_to<W: Write>(&self, schema_ir: &SchemaIR, writer: &mut W) -> Result<(), Box<SchemaIRError>> {
        match self {
            CacheFormat::Bincode => {
                let config = bincode::config::standard();
                let bytes = bincode::serde::encode_to_vec(schema_ir, config)
                    .map_err(|e| Box::new(SchemaIRError::CacheWriteError { msg: e.to_string() }))?;
                writer
                    .write_all(&bytes)
                    .map_err(|e| Box::new(SchemaIRError::CacheWriteError { msg: e.to_string() }))?;
            },
        }
        Ok(())
    }

    pub(crate) fn read_from<R: Read>(&self, reader: &mut R) -> Result<SchemaIR, Box<SchemaIRError>> {
        match self {
            CacheFormat::Bincode => {
                let mut body = Vec::new();
                reader
                    .read_to_end(&mut body)
                    .map_err(|e| Box::new(SchemaIRError::CacheReadError { msg: e.to_string() }))?;
                let config = bincode::config::standard();
                let (ir, _consumed): (SchemaIR, usize) = bincode::serde::decode_from_slice(&body, config)
                    .map_err(|e| Box::new(SchemaIRError::CacheReadError { msg: e.to_string() }))?;
                Ok(ir)
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheHeader {
    pub version: u32,
    pub body_format: String,
    pub rudof_version: String,
    pub has_neg_cycle: bool,
}

impl CacheHeader {
    pub fn new(fmt: CacheFormat, has_neg_cycle: bool) -> Self {
        CacheHeader {
            version: CACHE_VERSION,
            body_format: fmt.as_str().to_string(),
            rudof_version: env!("CARGO_PKG_VERSION").to_string(),
            has_neg_cycle,
        }
    }

    pub fn body_format(&self) -> Result<CacheFormat, SchemaIRError> {
        match self.body_format.as_str() {
            "bincode" => Ok(CacheFormat::Bincode),
            _ => Err(SchemaIRError::CacheReadError {
                msg: format!("Unknown cache body format: {}", self.body_format),
            }),
        }
    }

    pub(crate) fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), Box<SchemaIRError>> {
        let line =
            serde_json::to_string(self).map_err(|e| Box::new(SchemaIRError::CacheWriteError { msg: e.to_string() }))?;
        writeln!(writer, "{line}").map_err(|e| Box::new(SchemaIRError::CacheWriteError { msg: e.to_string() }))?;
        Ok(())
    }

    pub(crate) fn read_from<R: BufRead>(reader: &mut R) -> Result<Self, Box<SchemaIRError>> {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| Box::new(SchemaIRError::CacheReadError { msg: e.to_string() }))?;
        serde_json::from_str(line.trim_end()).map_err(|e| {
            Box::new(SchemaIRError::CacheReadError {
                msg: format!("parsing header: {e}"),
            })
        })
    }
}
