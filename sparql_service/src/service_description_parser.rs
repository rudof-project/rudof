use crate::{
    Dataset, Feature, GraphCollection, GraphDescription, NamedGraphDescription,
    SD_AVAILABLE_GRAPHS, SD_BASIC_FEDERATED_QUERY_STR, SD_DEFAULT_DATASET, SD_DEFAULT_GRAPH,
    SD_DEREFERENCES_URIS_STR, SD_EMPTY_GRAPHS_STR, SD_ENDPOINT, SD_FEATURE, SD_GRAPH, SD_NAME,
    SD_NAMED_GRAPH, SD_REQUIRES_DATASET_STR, SD_RESULT_FORMAT, SD_SERVICE, SD_SPARQL10_QUERY_STR,
    SD_SPARQL11_QUERY_STR, SD_SPARQL11_UPDATE_STR, SD_SUPPORTED_LANGUAGE,
    SD_UNION_DEFAULT_GRAPH_STR, ServiceDescription, ServiceDescriptionError, SparqlResultFormat,
    SupportedLanguage, VOID_CLASSES, VOID_TRIPLES,
};
use iri_s::{IriS, iri};
use oxrdf::Graph;
use srdf::{
    FnOpaque, FocusRDF, IriOrBlankNode, Object, PResult, RDFNodeParse, RDFParser, Rdf, get_focus,
    get_focus_iri_or_bnode, numeric_literal::NumericLiteral, object, ok, opaque, optional,
    parse_property_values, property_integer, property_iri, property_iri_or_bnode, property_number,
    property_values_iri, property_values_iri_or_bnode, set_focus, set_focus_iri_or_bnode,
};
use std::{collections::HashSet, fmt::Debug};
use tracing::debug;

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
        get_focus_iri_or_bnode().then(|focus| {
            let focus = focus.clone();
            endpoint().then(move |maybe_iri| {
                let focus = focus.clone();
                supported_language().then(move |supported_language| {
                    result_format().then({
                        let focus = focus.clone();
                        let iri = maybe_iri.clone();
                        move |result_format| {
                            feature().then({
                                let focus = focus.clone();
                                let sl = supported_language.clone();
                                let iri = iri.clone();
                                move |feature| {
                                    optional(default_dataset(&focus)).then({
                                        // TODO: There is something ugly here with so many clone()'s...refactor!!
                                        let focus = focus.clone();
                                        let iri = iri.clone();
                                        let sl = sl.clone();
                                        let result_format = result_format.clone();
                                        move |default_ds| {
                                            let focus = focus.clone();
                                            let iri = iri.clone();
                                            let sl = sl.clone();
                                            let result_format = result_format.clone();
                                            let feature = feature.clone();
                                            available_graphs(&focus).then({
                                                move |ags| {
                                                    let mut sd = ServiceDescription::new()
                                                        .with_endpoint(iri.clone())
                                                        .with_available_graphs(ags)
                                                        .with_default_dataset(default_ds.clone());
                                                    sd.add_supported_languages(sl.clone());
                                                    sd.add_features(feature.clone());
                                                    sd.add_result_formats(result_format.clone());
                                                    ok(&sd)
                                                }
                                            })
                                        }
                                    })
                                }
                            })
                        }
                    })
                })
            })
        })
    }

    fn sd_service() -> RDF::Term {
        SD_SERVICE.clone().into()
    }
}

pub fn endpoint<RDF>() -> impl RDFNodeParse<RDF, Output = Option<IriS>>
where
    RDF: FocusRDF + 'static,
{
    optional(property_iri(&SD_ENDPOINT))
}

pub fn feature<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<Feature>>
where
    RDF: FocusRDF,
{
    property_values_iri(&SD_FEATURE).flat_map(|ref iris| {
        let features = get_features(iris)?;
        Ok(features)
    })
}

pub fn result_format<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<SparqlResultFormat>>
where
    RDF: FocusRDF,
{
    property_values_iri(&SD_RESULT_FORMAT).flat_map(|ref iris| {
        let result_format = get_result_formats(iris)?;
        Ok(result_format)
    })
}

pub fn supported_language<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<SupportedLanguage>>
where
    RDF: FocusRDF,
{
    property_values_iri(&SD_SUPPORTED_LANGUAGE).flat_map(|ref iris| {
        let langs = get_supported_languages(iris)?;
        Ok(langs)
    })
}

fn get_supported_languages(iris: &HashSet<IriS>) -> PResult<HashSet<SupportedLanguage>> {
    let mut res = HashSet::new();
    for i in iris {
        let supported_language = supported_language_iri(i)?;
        res.insert(supported_language);
    }
    Ok(res)
}

fn get_features(iris: &HashSet<IriS>) -> PResult<HashSet<Feature>> {
    let mut res = HashSet::new();
    for i in iris {
        let feature = feature_iri(i)?;
        res.insert(feature);
    }
    Ok(res)
}

fn get_result_formats(iris: &HashSet<IriS>) -> PResult<HashSet<SparqlResultFormat>> {
    let mut res = HashSet::new();
    for i in iris {
        let res_format = result_format_iri(i)?;
        res.insert(res_format);
    }
    Ok(res)
}

fn supported_language_iri(iri: &IriS) -> PResult<SupportedLanguage> {
    match iri.as_str() {
        SD_SPARQL10_QUERY_STR => Ok(SupportedLanguage::SPARQL10Query),
        SD_SPARQL11_QUERY_STR => Ok(SupportedLanguage::SPARQL11Query),
        SD_SPARQL11_UPDATE_STR => Ok(SupportedLanguage::SPARQL11Update),
        _ => Err(srdf::RDFParseError::Custom {
            msg: format!("Unexpected value for supported language: {iri}"),
        }),
    }
}

fn result_format_iri(iri: &IriS) -> PResult<SparqlResultFormat> {
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

fn feature_iri(iri: &IriS) -> PResult<Feature> {
    match iri.as_str() {
        SD_BASIC_FEDERATED_QUERY_STR => Ok(Feature::BasicFederatedQuery),
        SD_UNION_DEFAULT_GRAPH_STR => Ok(Feature::UnionDefaultGraph),
        SD_EMPTY_GRAPHS_STR => Ok(Feature::EmptyGraphs),
        SD_REQUIRES_DATASET_STR => Ok(Feature::RequiresDataset),
        SD_DEREFERENCES_URIS_STR => Ok(Feature::DereferencesURIs),
        _ => Ok(Feature::Other(iri.clone())),
    }
}

pub fn available_graphs<RDF>(
    node: &IriOrBlankNode,
) -> impl RDFNodeParse<RDF, Output = Vec<GraphCollection>>
where
    RDF: FocusRDF,
{
    set_focus_iri_or_bnode(&node).with(parse_property_values(
        &SD_AVAILABLE_GRAPHS,
        available_graph(),
    ))
}

pub fn available_graph<RDF>() -> impl RDFNodeParse<RDF, Output = GraphCollection>
where
    RDF: FocusRDF,
{
    object().then(
        |node| match <Object as TryInto<IriOrBlankNode>>::try_into(node) {
            Ok(ib) => ok(&GraphCollection::new(&ib)),
            Err(_) => todo!(),
        },
    )
}

pub fn default_dataset<RDF>(node: &IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Dataset>
where
    RDF: FocusRDF + 'static,
{
    set_focus_iri_or_bnode(node)
        .with(property_iri_or_bnode(&SD_DEFAULT_DATASET).then(|node_ds| dataset(node_ds)))
}

pub fn dataset<RDF>(node_ds: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Dataset>
where
    RDF: FocusRDF + 'static,
{
    set_focus_iri_or_bnode(&node_ds).with(
        get_focus_iri_or_bnode()
            .and(optional(default_graph(&node_ds)))
            .and(named_graphs(&node_ds))
            .then(|((focus, dg), named_gs)| {
                ok(&Dataset::new(&focus)
                    .with_default_graph(dg)
                    .with_named_graphs(named_gs))
            }),
    )
}

pub fn default_graph<RDF>(
    focus: &IriOrBlankNode,
) -> impl RDFNodeParse<RDF, Output = GraphDescription>
where
    RDF: FocusRDF + 'static,
{
    debug!("default_graph: focus={focus}");
    set_focus_iri_or_bnode(focus)
        .with(property_iri_or_bnode(&SD_DEFAULT_GRAPH).then(|node| graph_description(&node)))
}

pub fn graph_description<RDF>(
    node: &IriOrBlankNode,
) -> impl RDFNodeParse<RDF, Output = GraphDescription>
where
    RDF: FocusRDF + 'static,
{
    debug!("graph_description: focus={node}");
    set_focus_iri_or_bnode(node).with(
        get_focus_iri_or_bnode()
            .and(void_triples())
            .and(void_classes())
            .map(|((focus, triples), classes)| {
                GraphDescription::new(&focus)
                    .with_triples(triples)
                    .with_classes(classes)
            }),
    )
}

pub fn named_graphs<RDF>(
    focus: &IriOrBlankNode,
) -> impl RDFNodeParse<RDF, Output = Vec<NamedGraphDescription>>
where
    RDF: FocusRDF + 'static,
{
    set_focus_iri_or_bnode(focus).with(parse_property_values(&SD_NAMED_GRAPH, named_graph()))
}

pub fn named_graph<RDF>() -> impl RDFNodeParse<RDF, Output = NamedGraphDescription>
where
    RDF: FocusRDF + 'static,
{
    get_focus_iri_or_bnode().then(|focus| named_graph_description(&focus))
}

fn named_graph_description<RDF>(
    _focus: &IriOrBlankNode,
) -> impl RDFNodeParse<RDF, Output = NamedGraphDescription>
where
    RDF: FocusRDF + 'static,
{
    get_focus_iri_or_bnode()
        .and(name())
        .and(optional(graph()))
        .map(|((focus, name), graph)| {
            NamedGraphDescription::new(Some(focus), name).with_graph(graph)
        })
}

fn name<RDF>() -> impl RDFNodeParse<RDF, Output = IriS>
where
    RDF: FocusRDF + 'static,
{
    property_iri(&SD_NAME)
}

fn graph<RDF>() -> impl RDFNodeParse<RDF, Output = GraphDescription>
where
    RDF: FocusRDF + 'static,
{
    property_iri_or_bnode(&SD_GRAPH).then(|node| graph_description(&node))
}

pub fn void_triples<RDF>() -> FnOpaque<RDF, Option<NumericLiteral>>
where
    RDF: FocusRDF,
{
    opaque!(optional(property_number(&VOID_TRIPLES)))
}

pub fn void_classes<RDF>() -> FnOpaque<RDF, Option<NumericLiteral>>
where
    RDF: FocusRDF,
{
    opaque!(optional(property_number(&VOID_CLASSES)))
}
