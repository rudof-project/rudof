use rudof_lib::formats::ShapeMapFormat;

pyenum! {
    /// ShapeMap serialization formats.
    "ShapeMapFormat": PyShapeMapFormat => ShapeMapFormat, default = Compact, from_str {
        /// Compact - human-readable ShapeMap syntax.
        Compact  => Compact,
        /// Internal representation used for processing.
        Internal => Internal,
        /// JSON - machine-readable JSON serialization.
        Json     => Json,
        /// Details - verbose output including validation details.
        Details  => Details,
        /// CSV - comma-separated values for spreadsheet tools.
        Csv      => Csv,
    }
}
