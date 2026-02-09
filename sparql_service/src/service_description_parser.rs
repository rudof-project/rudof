use crate::{
    ClassPartition, Dataset, Feature, GraphCollection, GraphDescription, NamedGraphDescription, PropertyPartition,
    SD_BASIC_FEDERATED_QUERY_STR, SD_DEREFERENCES_URIS_STR, SD_EMPTY_GRAPHS_STR, SD_REQUIRES_DATASET_STR,
    SD_SPARQL10_QUERY_STR, SD_SPARQL11_QUERY_STR, SD_SPARQL11_UPDATE_STR, SD_UNION_DEFAULT_GRAPH_STR,
    ServiceDescription, ServiceDescriptionError, SparqlResultFormat, SupportedLanguage, dct_title, sd_available_graphs,
    sd_default_dataset, sd_default_graph, sd_endpoint, sd_feature, sd_graph, sd_name, sd_named_graph, sd_result_format,
    sd_service, sd_supported_language, void_class, void_class_partition, void_classes, void_property,
    void_property_partition, void_triples,
};
use iri_s::IriS;
use srdf::{
    FocusRDF, IriOrBlankNode, PResult, RDFNodeParse, RDFParser, Rdf, get_focus_iri_or_bnode,
    numeric_literal::NumericLiteral, ok, optional, parse_property_values, property_iri, property_iri_or_bnode,
    property_number, property_string, property_values_iri, set_focus_iri_or_bnode,
};
use std::{collections::HashSet, fmt::Debug};
use tracing::trace;

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
        let sd_service: RDF::Term = <RDF as Rdf>::Term::from(sd_service().clone());
        let service_node = self.rdf_parser.instance_of(&sd_service)?;
        let term = service_node.into();
        self.rdf_parser.rdf.set_focus(&term);
        let service = Self::service_description().parse_impl(&mut self.rdf_parser.rdf)?;
        Ok(service.with_prefixmap(self.rdf_parser.prefixmap()))
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
                                    optional(default_dataset(focus.clone())).then({
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
                                            available_graphs(focus.clone()).then({
                                                move |ags| {
                                                    let focus = focus.clone();
                                                    let iri = iri.clone();
                                                    let sl = sl.clone();
                                                    let result_format = result_format.clone();
                                                    let feature = feature.clone();
                                                    let ags = ags.clone();
                                                    let default_ds = default_ds.clone();
                                                    title(focus).then(move |title| {
                                                        let mut sd = ServiceDescription::new()
                                                            .with_endpoint(iri.clone())
                                                            .with_available_graphs(ags.clone())
                                                            .with_default_dataset(default_ds.clone());
                                                        sd.add_title(title.as_deref());
                                                        sd.add_supported_languages(sl.clone());
                                                        sd.add_features(feature.clone());
                                                        sd.add_result_formats(result_format.clone());
                                                        ok(sd)
                                                    })
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
}

pub fn title<RDF>(focus: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Option<String>>
where
    RDF: FocusRDF + 'static,
{
    set_focus_iri_or_bnode(focus).with(optional(property_string(dct_title().clone())))
}

pub fn endpoint<RDF>() -> impl RDFNodeParse<RDF, Output = Option<IriS>>
where
    RDF: FocusRDF + 'static,
{
    optional(property_iri(sd_endpoint().clone()))
}

pub fn feature<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<Feature>>
where
    RDF: FocusRDF,
{
    property_values_iri(sd_feature().clone()).flat_map(|ref iris| {
        let features = get_features(iris)?;
        Ok(features)
    })
}

pub fn result_format<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<SparqlResultFormat>>
where
    RDF: FocusRDF,
{
    property_values_iri(sd_result_format().clone()).flat_map(|ref iris| {
        let result_format = get_result_formats(iris)?;
        Ok(result_format)
    })
}

pub fn supported_language<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<SupportedLanguage>>
where
    RDF: FocusRDF,
{
    property_values_iri(sd_supported_language().clone()).flat_map(|ref iris| {
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

pub fn available_graphs<RDF>(node: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Vec<GraphCollection>>
where
    RDF: FocusRDF + 'static,
{
    set_focus_iri_or_bnode(node).with(parse_property_values(sd_available_graphs(), available_graph()))
}

pub fn available_graph<RDF>() -> impl RDFNodeParse<RDF, Output = GraphCollection>
where
    RDF: FocusRDF + 'static,
{
    get_focus_iri_or_bnode().then(|focus| {
        parse_property_values(sd_named_graph(), named_graph())
            .map(move |named_graphs| GraphCollection::new(&focus.clone()).with_collection(named_graphs.into_iter()))
    })
}

pub fn default_dataset<RDF>(node: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Dataset>
where
    RDF: FocusRDF + 'static,
{
    set_focus_iri_or_bnode(node)
        .with(property_iri_or_bnode(sd_default_dataset().clone()).then(|node_ds| dataset(node_ds)))
}

pub fn dataset<RDF>(node_ds: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Dataset>
where
    RDF: FocusRDF + 'static,
{
    set_focus_iri_or_bnode(node_ds.clone()).with(
        get_focus_iri_or_bnode()
            .and(optional(default_graph(node_ds.clone())))
            .and(named_graphs(node_ds))
            .then(|((focus, dg), named_gs)| {
                ok(Dataset::new(&focus).with_default_graph(dg).with_named_graphs(named_gs))
            }),
    )
}

pub fn default_graph<RDF>(focus: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = GraphDescription>
where
    RDF: FocusRDF + 'static,
{
    trace!("parsing default_graph with focus={focus}");
    set_focus_iri_or_bnode(focus)
        .with(property_iri_or_bnode(sd_default_graph().clone()).then(|node| graph_description(node.clone())))
}

pub fn graph_description<RDF>(node: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = GraphDescription>
where
    RDF: FocusRDF + 'static,
{
    trace!("parsing graph_description: focus={node}");
    set_focus_iri_or_bnode(node.clone()).with(
        get_focus_iri_or_bnode()
            .and(parse_void_triples(node.clone()))
            .and(parse_void_classes(node.clone()))
            .and(parse_void_class_partition(node.clone()))
            .and(parse_void_property_partition(node.clone()))
            .map(|((((focus, triples), classes), class_partition), property_partition)| {
                let d = GraphDescription::new(&focus)
                    .with_triples(triples)
                    .with_classes(classes)
                    .with_class_partition(class_partition)
                    .with_property_partition(property_partition);
                trace!("parsed graph_description: {d}");
                d
            }),
    )
}

pub fn named_graphs<RDF>(focus: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Vec<NamedGraphDescription>>
where
    RDF: FocusRDF + 'static,
{
    trace!("parsing named_graphs with focus={focus}");
    set_focus_iri_or_bnode(focus).with(parse_property_values(sd_named_graph(), named_graph()))
}

pub fn named_graph<RDF>() -> impl RDFNodeParse<RDF, Output = NamedGraphDescription>
where
    RDF: FocusRDF + 'static,
{
    get_focus_iri_or_bnode().then(|focus| named_graph_description(focus))
}

fn named_graph_description<RDF>(focus: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = NamedGraphDescription>
where
    RDF: FocusRDF + 'static,
{
    trace!("parsing named_graph_description with focus={focus}");
    set_focus_iri_or_bnode(focus).with(
        get_focus_iri_or_bnode()
            .and(name())
            .and(parse_property_values(sd_graph(), graph()))
            .map(|((focus, name), graphs)| {
                trace!(
                    "named_graph_description: focus={focus}, name={name}, graphs={}",
                    graphs.len()
                );
                NamedGraphDescription::new(Some(focus), name).with_graphs(graphs)
            }),
    )
}

fn name<RDF>() -> impl RDFNodeParse<RDF, Output = IriS>
where
    RDF: FocusRDF + 'static,
{
    property_iri(sd_name().clone())
}

fn graph<RDF>() -> impl RDFNodeParse<RDF, Output = GraphDescription>
where
    RDF: FocusRDF + 'static,
{
    get_focus_iri_or_bnode().then(|focus| {
        trace!("Parsing graph at = {focus}, parsing it...");
        graph_description(focus)
    })
}

pub fn parse_void_triples<RDF>(node: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Option<NumericLiteral>>
where
    RDF: FocusRDF,
{
    set_focus_iri_or_bnode(node).with(optional(property_number(void_triples().clone())))
}

pub fn parse_void_classes<RDF>(node: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Option<NumericLiteral>>
where
    RDF: FocusRDF,
{
    set_focus_iri_or_bnode(node).with(optional(property_number(void_classes().clone())))
}

pub fn parse_void_class_partition<RDF>(node: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Vec<ClassPartition>>
where
    RDF: FocusRDF + 'static,
{
    set_focus_iri_or_bnode(node).with(parse_property_values(void_class_partition(), class_partition()))
}

pub fn parse_void_property_partition<RDF>(
    node: IriOrBlankNode,
) -> impl RDFNodeParse<RDF, Output = Vec<PropertyPartition>>
where
    RDF: FocusRDF + 'static,
{
    set_focus_iri_or_bnode(node).with(parse_property_values(void_property_partition(), property_partition()))
}

pub fn class_partition<RDF>() -> impl RDFNodeParse<RDF, Output = ClassPartition>
where
    RDF: FocusRDF + 'static,
{
    trace!("parsing class_partition");
    get_focus_iri_or_bnode().then(move |focus| {
        trace!("parsing class_partition with focus={focus}");
        ok(focus)
            .and(property_iri(void_class().clone()))
            .and(parse_property_values(void_property(), property_partition()))
            .map(|((focus, class), property_partition)| {
                ClassPartition::new(&class)
                    .with_id(&focus)
                    .with_property_partition(property_partition)
            })
    })
}

pub fn property_partition<RDF>() -> impl RDFNodeParse<RDF, Output = crate::PropertyPartition>
where
    RDF: FocusRDF + 'static,
{
    get_focus_iri_or_bnode()
        .and(property_iri(void_property().clone()).map(|p| p.clone()))
        .and(optional(property_number(void_triples().clone())))
        .map(|((focus, property), triples)| PropertyPartition::new(&property).with_id(&focus).with_triples(triples))
}
