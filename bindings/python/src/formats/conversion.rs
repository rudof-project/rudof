use rudof_lib::formats::{
    ConversionFormat, ConversionMode, ResultConversionFormat, ResultConversionMode,
};

pyenum! {
    /// Conversion input modes - the kind of schema being converted from.
    "ConversionMode": PyConversionMode => ConversionMode, default = ShEx {
        /// SHACL shapes graph.
        Shacl => Shacl,
        /// ShEx schema.
        ShEx  => ShEx,
        /// DCTAP profile.
        Dctap => Dctap,
    }
}

pyenum! {
    /// Conversion output modes - the kind of artifact being converted to.
    "ResultConversionMode": PyResultConversionMode => ResultConversionMode, default = ShEx {
        /// SPARQL queries.
        Sparql => Sparql,
        /// ShEx schema.
        ShEx   => ShEx,
        /// UML diagram.
        Uml    => Uml,
        /// HTML documentation.
        Html   => Html,
        /// SHACL shapes graph.
        Shacl  => Shacl,
    }
}

pyenum! {
    /// Conversion input formats.
    "ConversionFormat": PyConversionFormat => ConversionFormat, default = ShExC, from_str {
        /// CSV - comma-separated values.
        Csv    => Csv,
        /// ShExC - compact ShEx syntax.
        ShExC  => ShExC,
        /// ShExJ - JSON representation of a ShEx schema.
        ShExJ  => ShExJ,
        /// Turtle - compact, human-readable RDF format.
        Turtle => Turtle,
        /// Excel workbook (`.xlsx`).
        Xlsx   => Xlsx,
    }
}

pyenum! {
    /// Conversion output formats.
    "ResultConversionFormat": PyResultConversionFormat => ResultConversionFormat,
    default = Default, from_str {
        /// The default format for the selected output mode.
        Default  => Default,
        /// Internal representation used for processing.
        Internal => Internal,
        /// JSON - machine-readable JSON serialization.
        Json     => Json,
        /// ShExC - compact ShEx syntax.
        ShExC    => ShExC,
        /// ShExJ - JSON representation of a ShEx schema.
        ShExJ    => ShExJ,
        /// Turtle - compact, human-readable RDF format.
        Turtle   => Turtle,
        /// PlantUML - text-based UML diagram source.
        PlantUML => PlantUML,
        /// HTML documentation.
        Html     => Html,
        /// SVG - Scalable Vector Graphics image.
        Svg      => Svg,
        /// PNG - Portable Network Graphics image.
        Png      => Png,
    }
}
