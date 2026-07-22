use crate::{SchemaIRError, ir::schema_ir::SchemaIR};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// Magic bytes identifying a rudof precompiled SchemaIR cache file.
///
/// Written at offset 0 so quick reads can identify the format
/// before decoding anything.
pub const MAGIC: &[u8; 4] = b"RSIR";

/// On-disk envelope version. Bump when the layout of magic + version + header
/// changes in a way older readers cannot understand.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CacheFormat {
    #[default]
    Bincode,
}

impl CacheFormat {
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

/// Header of a precompiled `SchemaIR` cache. Length-prefixed on disk so future
/// versions can grow it without breaking older readers (they can skip it).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheHeader {
    pub body_format: CacheFormat,
    pub rudof_version: String,
    pub has_neg_cycle: bool,
}

impl CacheHeader {
    pub fn new(fmt: CacheFormat, has_neg_cycle: bool) -> Self {
        CacheHeader {
            body_format: fmt,
            rudof_version: env!("CARGO_PKG_VERSION").to_string(),
            has_neg_cycle,
        }
    }

    pub fn body_format(&self) -> CacheFormat {
        self.body_format
    }

    pub(crate) fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), Box<SchemaIRError>> {
        let config = bincode::config::standard();
        let header_bytes = bincode::serde::encode_to_vec(self, config)
            .map_err(|e| Box::new(SchemaIRError::CacheWriteError { msg: e.to_string() }))?;
        let header_len: u32 = header_bytes.len().try_into().map_err(|_| {
            Box::new(SchemaIRError::CacheWriteError {
                msg: "cache header exceeds u32::MAX bytes".to_string(),
            })
        })?;

        let write = |w: &mut W, buf: &[u8]| -> Result<(), Box<SchemaIRError>> {
            w.write_all(buf)
                .map_err(|e| Box::new(SchemaIRError::CacheWriteError { msg: e.to_string() }))
        };

        write(writer, MAGIC)?;
        write(writer, &CACHE_VERSION.to_le_bytes())?;
        write(writer, &header_len.to_le_bytes())?;
        write(writer, &header_bytes)?;
        Ok(())
    }

    pub(crate) fn read_from<R: Read>(reader: &mut R) -> Result<Self, Box<SchemaIRError>> {
        let read_exact = |r: &mut R, buf: &mut [u8], what: &str| -> Result<(), Box<SchemaIRError>> {
            r.read_exact(buf).map_err(|e| {
                Box::new(SchemaIRError::CacheReadError {
                    msg: format!("reading {what}: {e}"),
                })
            })
        };

        let mut magic = [0u8; 4];
        read_exact(reader, &mut magic, "magic bytes")?;
        if &magic != MAGIC {
            return Err(Box::new(SchemaIRError::CacheReadError {
                msg: format!("Not a SchemaIR cache: expected magic {:?}, found {:?}", MAGIC, magic),
            }));
        }

        let mut version_bytes = [0u8; 4];
        read_exact(reader, &mut version_bytes, "cache version")?;
        let version = u32::from_le_bytes(version_bytes);
        if version != CACHE_VERSION {
            return Err(Box::new(SchemaIRError::CacheReadError {
                msg: format!("Incompatible cache version: found {version}, expected {CACHE_VERSION}"),
            }));
        }

        let mut len_bytes = [0u8; 4];
        read_exact(reader, &mut len_bytes, "header length")?;
        let header_len = u32::from_le_bytes(len_bytes) as usize;
        let mut header_bytes = vec![0u8; header_len];
        read_exact(reader, &mut header_bytes, "header body")?;

        let config = bincode::config::standard();
        let (header, _consumed): (CacheHeader, usize) = bincode::serde::decode_from_slice(&header_bytes, config)
            .map_err(|e| {
                Box::new(SchemaIRError::CacheReadError {
                    msg: format!("decoding header: {e}"),
                })
            })?;
        Ok(header)
    }
}
