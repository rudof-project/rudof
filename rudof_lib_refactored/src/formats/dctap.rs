use dctap::dctap_format::DCTAPFormat;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::errors::DCTapError;

/// DC-TAP (Dublin Core Tabular Application Profiles) formats supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum DCTapFormat {
    /// CSV - Comma-Separated Values format (default)
    #[default]
    Csv,
    /// XLSX - Excel 2007+ format (Open XML)
    Xlsx,
    /// XLSB - Excel Binary Workbook format
    Xlsb,
    /// XLSM - Excel Macro-Enabled Workbook format
    Xlsm,
    /// XLS - Excel 97-2003 format (legacy)
    Xls,
}

/// Output formats for DC-TAP processing results supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ResultDCTapFormat {
    /// Internal format - internal representation for processing (default)
    #[default]
    Internal,
    /// JSON format - machine-readable JSON serialization
    Json,
}

// ============================================================================
// DCTAPFormat
// ============================================================================

impl From<DCTapFormat> for DCTAPFormat {
    fn from(format: DCTapFormat) -> Self {
        match format {
            DCTapFormat::Csv => DCTAPFormat::Csv,
            DCTapFormat::Xlsx => DCTAPFormat::Xlsx,
            DCTapFormat::Xlsb => DCTAPFormat::Xlsb,
            DCTapFormat::Xlsm => DCTAPFormat::Xlsm,
            DCTapFormat::Xls => DCTAPFormat::Xls,
        }
    }
}

impl Display for DCTapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapFormat::Csv => write!(dest, "csv"),
            DCTapFormat::Xlsx => write!(dest, "xlsx"),
            DCTapFormat::Xlsb => write!(dest, "xlsb"),
            DCTapFormat::Xlsm => write!(dest, "xlsm"),
            DCTapFormat::Xls => write!(dest, "xls"),
        }
    }
}

impl FromStr for DCTapFormat {
    type Err = DCTapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(DCTapFormat::Csv),
            "xlsx" => Ok(DCTapFormat::Xlsx),
            "xlsb" => Ok(DCTapFormat::Xlsb),
            "xlsm" => Ok(DCTapFormat::Xlsm),
            "xls" => Ok(DCTapFormat::Xls),
            other => Err(DCTapError::UnsupportedDCTapFormat {
                format: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// ResultDCTapFormat
// ============================================================================

impl Display for ResultDCTapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultDCTapFormat::Internal => write!(dest, "internal"),
            ResultDCTapFormat::Json => write!(dest, "json"),
        }
    }
}

impl FromStr for ResultDCTapFormat {
    type Err = DCTapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ResultDCTapFormat::Internal),
            "json" => Ok(ResultDCTapFormat::Json),
            other => Err(DCTapError::UnsupportedResultDCTapFormat {
                format: other.to_string(),
            }),
        }
    }
}
