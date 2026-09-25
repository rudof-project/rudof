use rudof_lib::formats::{DCTapFormat, ResultDCTapFormat};

pyenum! {
    /// DCTAP input formats.
    "DCTapFormat": PyDCTapFormat => DCTapFormat, default = Csv, from_str {
        /// CSV - comma-separated values.
        Csv  => Csv,
        /// Excel workbook (`.xlsx`).
        Xlsx => Xlsx,
        /// Excel binary workbook (`.xlsb`).
        Xlsb => Xlsb,
        /// Excel macro-enabled workbook (`.xlsm`).
        Xlsm => Xlsm,
        /// Legacy Excel workbook (`.xls`).
        Xls  => Xls,
    }
}

pyenum! {
    /// DCTAP output formats.
    "ResultDCTapFormat": PyResultDCTapFormat => ResultDCTapFormat, default = Internal, from_str {
        /// Internal representation used for processing.
        Internal => Internal,
        /// JSON - machine-readable JSON serialization.
        Json     => Json,
    }
}
