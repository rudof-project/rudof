use iri_s::IriS;
use srdf::{FocusRDF, PResult, RDFNodeParse, RDFParser, ok, property_iri, property_values_iri};
use std::fmt::Debug;

use crate::{
    Dataset, Feature, SD_BASIC_FEDERATED_QUERY_STR, SD_DEFAULT_DATASET, SD_DEREFERENCES_URIS_STR,
    SD_EMPTY_GRAPHS_STR, SD_ENDPOINT, SD_FEATURE, SD_REQUIRES_DATASET_STR, SD_RESULT_FORMAT,
    SD_SERVICE, SD_SPARQL10_QUERY_STR, SD_SPARQL11_QUERY_STR, SD_SPARQL11_UPDATE_STR,
    SD_SUPPORTED_LANGUAGE, SD_UNION_DEFAULT_GRAPH_STR, ServiceDescription, ServiceDescriptionError,
    SparqlResultFormat, SupportedLanguage,
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
    RDF: FocusRDF + Debug + 'static,
{
    pub fn new(rdf: RDF) -> ServiceDescriptionParser<RDF> {
        ServiceDescriptionParser {
            rdf_parser: RDFParser::new(rdf),
        }
    }

    pub fn parse(&mut self) -> Result<ServiceDescription> {
        let service_node = self.rdf_parser.instance_of(&Self::sd_service())?;
        let term = service_node.into();
        self.rdf_parser.rdf.set_focus(&term);
        let service = Self::service_description().parse_impl(&mut self.rdf_parser.rdf)?;
        Ok(service)
    }

    pub fn service_description() -> impl RDFNodeParse<RDF, Output = ServiceDescription>
    where
        RDF: FocusRDF + 'static,
    {
        Self::endpoint().then(|iri| {
            Self::supported_language().then(move |supported_language| {
                Self::result_format().then({
                    let iri = iri.clone();
                    move |result_format| {
                        Self::feature().then({
                            let sl = supported_language.clone();
                            let iri = iri.clone();
                            move |feature| {
                                Self::default_dataset().then({
                                    // TODO: There is something ugly here with so many clone()'s...refactor!!
                                    let iri = iri.clone();
                                    let sl = sl.clone();
                                    let result_format = result_format.clone();
                                    move |default_ds| {
                                        let mut sd = ServiceDescription::new(iri.clone());
                                        sd.add_supported_language(&sl);
                                        sd.add_feature(&feature);
                                        sd.add_result_format(&result_format);
                                        sd.add_default_dataset(&default_ds);
                                        ok(&sd)
                                    }
                                })
                            }
                        })
                    }
                })
            })
        })
    }

    pub fn default_dataset() -> impl RDFNodeParse<RDF, Output = Dataset>
    where
        RDF: FocusRDF + 'static,
    {
        property_iri(&SD_DEFAULT_DATASET).then(move |iri| ok(&Dataset::new(&iri)))
    }

    pub fn endpoint() -> impl RDFNodeParse<RDF, Output = IriS>
    where
        RDF: FocusRDF + 'static,
    {
        property_iri(&SD_ENDPOINT)
    }

    pub fn feature() -> impl RDFNodeParse<RDF, Output = Vec<Feature>>
    where
        RDF: FocusRDF,
    {
        property_values_iri(&SD_FEATURE).flat_map(|ref iris| {
            let features = get_features(iris)?;
            Ok(features)
        })
    }

    pub fn result_format() -> impl RDFNodeParse<RDF, Output = Vec<SparqlResultFormat>>
    where
        RDF: FocusRDF,
    {
        property_values_iri(&SD_RESULT_FORMAT).flat_map(|ref iris| {
            let result_format = get_result_formats(iris)?;
            Ok(result_format)
        })
    }

    pub fn supported_language() -> impl RDFNodeParse<RDF, Output = Vec<SupportedLanguage>>
    where
        RDF: FocusRDF,
    {
        property_values_iri(&SD_SUPPORTED_LANGUAGE).flat_map(|ref iris| {
            let langs = get_supported_languages(iris)?;
            Ok(langs)
        })
    }

    fn sd_service() -> RDF::Term {
        SD_SERVICE.clone().into()
    }
}

fn get_supported_languages(iris: &Vec<IriS>) -> PResult<Vec<SupportedLanguage>> {
    let mut res = Vec::new();
    for i in iris {
        let supported_language = supported_language(i)?;
        res.push(supported_language)
    }
    Ok(res)
}

fn get_features(iris: &Vec<IriS>) -> PResult<Vec<Feature>> {
    let mut res = Vec::new();
    for i in iris {
        let feature = feature(i)?;
        res.push(feature)
    }
    Ok(res)
}

fn get_result_formats(iris: &Vec<IriS>) -> PResult<Vec<SparqlResultFormat>> {
    let mut res = Vec::new();
    for i in iris {
        let res_format = result_format(i)?;
        res.push(res_format)
    }
    Ok(res)
}

fn supported_language(iri: &IriS) -> PResult<SupportedLanguage> {
    match iri.as_str() {
        SD_SPARQL10_QUERY_STR => Ok(SupportedLanguage::SPARQL10Query),
        SD_SPARQL11_QUERY_STR => Ok(SupportedLanguage::SPARQL11Query),
        SD_SPARQL11_UPDATE_STR => Ok(SupportedLanguage::SPARQL11Update),
        _ => Err(srdf::RDFParseError::Custom {
            msg: format!("Unexpected value for supported language: {iri}"),
        }),
    }
}

fn result_format(iri: &IriS) -> PResult<SparqlResultFormat> {
    let rf = match iri.as_str() {
        "http://www.w3.org/ns/formats/SPARQL_Results_XML" => SparqlResultFormat::XML,
        "http://www.w3.org/ns/formats/JSON-LD" => SparqlResultFormat::JsonLD,
        "http://www.w3.org/ns/formats/N-Triples" => SparqlResultFormat::NTriples,
        "http://www.w3.org/ns/formats/SPARQL_Results_CSV" => SparqlResultFormat::CSV,
        "http://www.w3.org/ns/formats/SPARQL_Results_JSON" => SparqlResultFormat::JSON,
        "http://www.w3.org/ns/formats/Turtle" => SparqlResultFormat::Turtle,
        "http://www.w3.org/ns/formats/SPARQL_Results_TSV" => SparqlResultFormat::TSV,
        "http://www.w3.org/ns/formats/RDF_XML" => SparqlResultFormat::RdfXml,
        _ => SparqlResultFormat::Other(iri.clone()),
    };
    Ok(rf)
}

fn feature(iri: &IriS) -> PResult<Feature> {
    match iri.as_str() {
        SD_BASIC_FEDERATED_QUERY_STR => Ok(Feature::BasicFederatedQuery),
        SD_UNION_DEFAULT_GRAPH_STR => Ok(Feature::UnionDefaultGraph),
        SD_EMPTY_GRAPHS_STR => Ok(Feature::EmptyGraphs),
        SD_REQUIRES_DATASET_STR => Ok(Feature::RequiresDataset),
        SD_DEREFERENCES_URIS_STR => Ok(Feature::DereferencesURIs),
        _ => Ok(Feature::Other(iri.clone())),
    }
}
