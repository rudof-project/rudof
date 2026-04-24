use rudof_iri::MimeType;

/// SHACL Formats supported
// Mostly RDF formats
// In the future, we could also support SHACL Compact format
// Maybe unify RDF formats in an enum in rudof_rdf and this could be replaced
#[derive(Debug, Clone, Default)]
pub enum ShaclFormat {
    Internal,
    #[default]
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    JsonLd,
}

impl MimeType for ShaclFormat {
    fn mime_type(&self) -> &'static str {
        match &self {
            ShaclFormat::Internal => "application/shacl+json",
            ShaclFormat::Turtle => "text/turtle",
            ShaclFormat::NTriples => "application/n-triples",
            ShaclFormat::RdfXml => "application/rdf+xml",
            ShaclFormat::TriG => "application/trig",
            ShaclFormat::N3 => "text/n3",
            ShaclFormat::NQuads => "application/n-quads",
            ShaclFormat::JsonLd => "application/ld+json",
        }
    }
}
