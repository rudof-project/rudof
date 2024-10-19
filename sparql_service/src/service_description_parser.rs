use iri_s::IriS;
use srdf::{ok, property_iri, property_value, property_values, FocusRDF, RDFNodeParse, RDFParser};
use std::fmt::Debug;

use crate::{
    ServiceDescription, ServiceDescriptionError, SD_ENDPOINT, SD_SERVICE, SD_SUPPORTED_LANGUAGE,
};

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
        Self::endpoint().flat_map(|iri| Ok(ServiceDescription::new(iri)))
        /*        property_value(&SD_ENDPOINT).then(move |term: RDF::Term| {
            if let Some(sd_iri) = RDF::term_as_iri(&term) {
                let sd_iri_s = RDF::iri2iri_s(&sd_iri);
                property_values(&SD_SUPPORTED_LANGUAGE)
                    .then_mut(move |ts| ok(&ServiceDescription::new(sd_iri_s)))
            } else {
                todo!()
            }
        }) */
    }

    pub fn endpoint() -> impl RDFNodeParse<RDF, Output = IriS>
    where
        RDF: FocusRDF,
    {
        property_iri(&SD_ENDPOINT)
    }

    fn sd_service() -> RDF::Term {
        RDF::iri_s2term(&SD_SERVICE)
    }
}
