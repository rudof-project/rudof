use srdf::{ok, property_value, FocusRDF, RDFNodeParse, RDFParser};
use std::fmt::Debug;

use crate::{ServiceDescription, ServiceDescriptionError, SD_ENDPOINT, SD_SERVICE};

type Result<A> = std::result::Result<A, ServiceDescriptionError>;

pub struct ServiceDescriptionParser<RDF>
where
    RDF: FocusRDF + Debug,
{
    rdf_parser: RDFParser<RDF>,
}

impl<RDF> ServiceDescriptionParser<RDF>
where
    RDF: FocusRDF + Debug,
{
    pub fn new(rdf: RDF) -> ServiceDescriptionParser<RDF> {
        ServiceDescriptionParser {
            rdf_parser: RDFParser::new(rdf),
        }
    }

    pub fn parse(&mut self) -> Result<ServiceDescription> {
        let service_node = self.rdf_parser.instance_of(&Self::sd_service())?;
        let term = RDF::subject_as_term(&service_node);
        self.rdf_parser.rdf.set_focus(&term);
        let service = Self::service_description().parse_impl(&mut self.rdf_parser.rdf)?;
        Ok(service)
    }

    pub fn service_description<'a>() -> impl RDFNodeParse<RDF, Output = ServiceDescription> + 'a
    where
        RDF: FocusRDF + 'a,
    {
        property_value(&SD_ENDPOINT).then(move |term: RDF::Term| {
            if let Some(iri) = RDF::term_as_iri(&term) {
                let iri_s = RDF::iri2iri_s(&iri);
                ok(&ServiceDescription::new(iri_s))
            } else {
                todo!()
            }
        })
    }

    fn sd_service() -> RDF::Term {
        RDF::iri_s2term(&SD_SERVICE)
    }
}
