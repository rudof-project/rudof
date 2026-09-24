use rudof_lib::formats::ResultServiceFormat;

pyenum! {
    /// Service description serialization formats.
    "ServiceDescriptionFormat": PyServiceDescriptionFormat => ResultServiceFormat,
    default = Internal, from_str {
        /// Internal representation used for processing.
        Internal => Internal,
        /// MIE - machine-readable interface description.
        Mie      => Mie,
        /// JSON - machine-readable JSON serialization.
        Json     => Json,
    }
}
