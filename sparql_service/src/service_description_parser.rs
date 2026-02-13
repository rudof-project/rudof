use crate::{
    ClassPartition, Dataset, Feature, GraphCollection, GraphDescription, NamedGraphDescription,
    PropertyPartition, SD_BASIC_FEDERATED_QUERY_STR, SD_DEREFERENCES_URIS_STR, SD_EMPTY_GRAPHS_STR,
    SD_REQUIRES_DATASET_STR, SD_SPARQL10_QUERY_STR, SD_SPARQL11_QUERY_STR, SD_SPARQL11_UPDATE_STR,
    SD_UNION_DEFAULT_GRAPH_STR, ServiceDescription, ServiceDescriptionError, SparqlResultFormat,
    SupportedLanguage, dct_title, sd_available_graphs, sd_default_dataset, sd_default_graph,
    sd_endpoint, sd_feature, sd_graph, sd_name, sd_named_graph, sd_result_format, sd_service,
    sd_supported_language, void_class, void_class_partition, void_classes, void_property,
    void_property_partition, void_triples,
};
use iri_s::IriS;
use rdf::rdf_core::{
    FocusRDF, RDFError, parser::{
        RDFParse, 
        rdf_node_parser::{
            ParserExt, RDFNodeParse, constructors::{
                FocusParser, IriOrBlankNodeParser, IrisPropertyParser, SetFocusParser, SingleIntegerPropertyParser, 
                SingleIriPropertyParser, SingleStringPropertyParser, SuccessParser, SingleIriOrBlankNodePropertyParser
            }
        }
    }, term::{IriOrBlankNode, literal::NumericLiteral}
};

use std::{collections::HashSet, fmt::Debug};
use tracing::trace;

pub struct ServiceDescriptionParser<RDF>
where
    RDF: FocusRDF + Debug,
{
    rdf_parser: RDFParse<RDF>,
}

impl<RDF> ServiceDescriptionParser<RDF>
where
    RDF: FocusRDF + Debug + 'static,
    RDF::Term: From<IriOrBlankNode> + TryInto<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    pub fn new(rdf: RDF) -> ServiceDescriptionParser<RDF> {
        ServiceDescriptionParser {
            rdf_parser: RDFParse::new(rdf),
        }
    }

    pub fn parse(&mut self) -> Result<ServiceDescription, ServiceDescriptionError> {
        let service_node = self.rdf_parser.find_single_instance(sd_service().clone())?;
        let term: RDF::Term = service_node.into();
        self.rdf_parser.set_focus(&term);
        let service = Self::service_description().parse_focused(self.rdf_parser.rdf_mut())?;
        Ok(service.with_prefixmap(self.rdf_parser.prefixmap()))
    }

    pub fn service_description() -> impl RDFNodeParse<RDF, Output = ServiceDescription>
    where
        RDF: FocusRDF + 'static,
        RDF::Term: From<IriOrBlankNode> + TryInto<IriOrBlankNode> + Clone,
        <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
    {
        IriOrBlankNodeParser::new().then(|focus| {
            let focus_term: RDF::Term = focus.clone().into();
            endpoint().then(move |maybe_iri| {
                let focus_term = focus_term.clone();
                supported_language().then(move |supported_language| {
                    result_format().then({
                        let focus_term = focus_term.clone();
                        let iri = maybe_iri.clone();
                        move |result_format| {
                            feature().then({
                                let focus_term = focus_term.clone();
                                let sl = supported_language.clone();
                                let iri = iri.clone();
                                move |feature| {
                                    default_dataset(focus_term.clone()).optional().then({
                                        let focus_term = focus_term.clone();
                                        let iri = iri.clone();
                                        let sl = sl.clone();
                                        let result_format = result_format.clone();
                                        move |default_ds| {
                                            let focus_term = focus_term.clone();
                                            let iri = iri.clone();
                                            let sl = sl.clone();
                                            let result_format = result_format.clone();
                                            let feature = feature.clone();
                                            available_graphs(focus_term.clone()).then({
                                                move |ags| {
                                                    let focus_term = focus_term.clone();
                                                    let iri = iri.clone();
                                                    let sl = sl.clone();
                                                    let result_format = result_format.clone();
                                                    let feature = feature.clone();
                                                    let ags = ags.clone();
                                                    let default_ds = default_ds.clone();
                                                    title(focus_term).then(move |title| {
                                                        let mut sd = ServiceDescription::new()
                                                            .with_endpoint(iri.clone())
                                                            .with_available_graphs(ags.clone())
                                                            .with_default_dataset(
                                                                default_ds.clone(),
                                                            );
                                                        sd.add_title(title.as_deref());
                                                        sd.add_supported_languages(sl.clone());
                                                        sd.add_features(feature.clone());
                                                        sd.add_result_formats(
                                                            result_format.clone(),
                                                        );
                                                        SuccessParser::new(sd)
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

pub fn title<RDF>(focus: RDF::Term) -> impl RDFNodeParse<RDF, Output = Option<String>>
where
    RDF: FocusRDF + 'static,
    RDF::Term: Clone,
{
    SetFocusParser::new(focus).with(SingleStringPropertyParser::new(dct_title().clone()).optional())
}

pub fn endpoint<RDF>() -> impl RDFNodeParse<RDF, Output = Option<IriS>>
where
    RDF: FocusRDF + 'static,
{
    SingleIriPropertyParser::new(sd_endpoint().clone()).optional()
}

pub fn feature<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<Feature>>
where
    RDF: FocusRDF,
{
    IrisPropertyParser::new(sd_feature().clone()).flat_map(|ref iris| {
        let features = get_features(iris)?;
        Ok(features)
    })
}

pub fn result_format<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<SparqlResultFormat>>
where
    RDF: FocusRDF,
{
    IrisPropertyParser::new(sd_result_format().clone()).flat_map(|ref iris| {
        let result_format = get_result_formats(iris)?;
        Ok(result_format)
    })
}

pub fn supported_language<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<SupportedLanguage>>
where
    RDF: FocusRDF,
{
    IrisPropertyParser::new(sd_supported_language().clone()).flat_map(|ref iris| {
        let langs = get_supported_languages(iris)?;
        Ok(langs)
    })
}

fn get_supported_languages(iris: &HashSet<IriS>) -> Result<HashSet<SupportedLanguage>, RDFError> {
    let mut res = HashSet::new();
    for i in iris {
        let supported_language = supported_language_iri(i)?;
        res.insert(supported_language);
    }
    Ok(res)
}

fn get_features(iris: &HashSet<IriS>) -> Result<HashSet<Feature>, RDFError> {
    let mut res = HashSet::new();
    for i in iris {
        let feature = feature_iri(i)?;
        res.insert(feature);
    }
    Ok(res)
}

fn get_result_formats(iris: &HashSet<IriS>) -> Result<HashSet<SparqlResultFormat>, RDFError> {
    let mut res = HashSet::new();
    for i in iris {
        let res_format = result_format_iri(i)?;
        res.insert(res_format);
    }
    Ok(res)
}

fn supported_language_iri(iri: &IriS) -> Result<SupportedLanguage, RDFError> {
    match iri.as_str() {
        SD_SPARQL10_QUERY_STR => Ok(SupportedLanguage::SPARQL10Query),
        SD_SPARQL11_QUERY_STR => Ok(SupportedLanguage::SPARQL11Query),
        SD_SPARQL11_UPDATE_STR => Ok(SupportedLanguage::SPARQL11Update),
        _ => Err(RDFError::DefaultError {
            msg: format!("Unexpected value for supported language: {iri}"),
        }),
    }
}

fn result_format_iri(iri: &IriS) -> Result<SparqlResultFormat, RDFError> {
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

fn feature_iri(iri: &IriS) -> Result<Feature, RDFError> {
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
    node: RDF::Term,
) -> impl RDFNodeParse<RDF, Output = Vec<GraphCollection>>
where
    RDF: FocusRDF + 'static,
    RDF::Term: From<IriOrBlankNode> + TryInto<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    SetFocusParser::new(node).with(available_graph().map_property(sd_available_graphs().clone()))
}

pub fn available_graph<RDF>() -> impl RDFNodeParse<RDF, Output = GraphCollection>
where
    RDF: FocusRDF + 'static,
    RDF::Term: TryInto<IriOrBlankNode> + From<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    FocusParser::new().then(|focus: RDF::Term| {
        let focus_clone = focus.clone();
        named_graph()
            .map_property(sd_named_graph().clone())
            .map(move |named_graphs| {
                let iri_or_blank: IriOrBlankNode = focus_clone.clone()
                    .try_into()
                    .expect("Failed to convert Term to IriOrBlankNode");
                GraphCollection::new(&iri_or_blank)
                    .with_collection(named_graphs.into_iter())
            })
    })
}

pub fn default_dataset<RDF>(node: RDF::Term) -> impl RDFNodeParse<RDF, Output = Dataset>
where
    RDF: FocusRDF + 'static,
    RDF::Term: From<IriOrBlankNode> + TryInto<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    SetFocusParser::new(node)
        .with(SingleIriOrBlankNodePropertyParser::new(sd_default_dataset().clone()).then(|node_ds| dataset(node_ds)))
}

pub fn dataset<RDF>(node_ds: IriOrBlankNode) -> impl RDFNodeParse<RDF, Output = Dataset>
where
    RDF: FocusRDF + 'static,
    RDF::Term: From<IriOrBlankNode> + TryInto<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    let node_term: RDF::Term = node_ds.clone().into();
    SetFocusParser::new(node_term.clone()).with(
        FocusParser::new()
            .and(default_graph(node_term.clone()).optional())
            .and(named_graphs(node_term))
            .then(move |((_, dg), named_gs)| {
                SuccessParser::new(Dataset::new(&node_ds)
                    .with_default_graph(dg)
                    .with_named_graphs(named_gs))
            }),
    )
}

pub fn default_graph<RDF>(
    focus: RDF::Term,
) -> impl RDFNodeParse<RDF, Output = GraphDescription>
where
    RDF: FocusRDF + 'static,
    RDF::Term: From<IriOrBlankNode> + TryInto<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    trace!("parsing default_graph with focus");
    SetFocusParser::new(focus).with(
        SingleIriOrBlankNodePropertyParser::new(sd_default_graph().clone())
            .then(|node| graph_description(node.clone())),
    )
}

pub fn graph_description<RDF>(
    node: IriOrBlankNode,
) -> impl RDFNodeParse<RDF, Output = GraphDescription>
where
    RDF: FocusRDF + 'static,
    RDF::Term: From<IriOrBlankNode> + TryInto<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    trace!("parsing graph_description: focus={node}");
    let node_term: RDF::Term = node.clone().into();
    SetFocusParser::new(node_term.clone()).with(
        FocusParser::new()
            .and(parse_void_triples(node_term.clone()))
            .and(parse_void_classes(node_term.clone()))
            .and(parse_void_class_partition(node_term.clone()))
            .and(parse_void_property_partition(node_term.clone()))
            .map(
                move |((((_, triples), classes), class_partition), property_partition)| {
                    let d = GraphDescription::new(&node)
                        .with_triples(triples)
                        .with_classes(classes)
                        .with_class_partition(class_partition)
                        .with_property_partition(property_partition);
                    trace!("parsed graph_description: {d}");
                    d
                },
            ),
    )
}

pub fn named_graphs<RDF>(
    focus: RDF::Term,
) -> impl RDFNodeParse<RDF, Output = Vec<NamedGraphDescription>>
where
    RDF: FocusRDF + 'static,
    RDF::Term: TryInto<IriOrBlankNode> + From<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    SetFocusParser::new(focus).with(named_graph().map_property(sd_named_graph().clone()))
}

pub fn named_graph<RDF>() -> impl RDFNodeParse<RDF, Output = NamedGraphDescription>
where
    RDF: FocusRDF + 'static,
    RDF::Term: TryInto<IriOrBlankNode> + From<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    FocusParser::new().then(|focus| named_graph_description(focus))
}

fn named_graph_description<RDF>(
    focus: RDF::Term,
) -> impl RDFNodeParse<RDF, Output = NamedGraphDescription>
where
    RDF: FocusRDF + 'static,
    RDF::Term: TryInto<IriOrBlankNode> + From<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    trace!("parsing named_graph_description with focus");
    let focus_clone = focus.clone();
    SetFocusParser::new(focus).with(
        FocusParser::new()
            .and(name())
            .and(graph().map_property(sd_graph().clone()))
            .map(move |((_, name), graphs)| {
                let focus_iri: IriOrBlankNode = focus_clone.clone()
                    .try_into()
                    .expect("Failed to convert Term to IriOrBlankNode");
                trace!(
                    "named_graph_description: focus={focus_iri}, name={name}, graphs={}",
                    graphs.len()
                );
                NamedGraphDescription::new(Some(focus_iri), name).with_graphs(graphs)
            }),
    )
}

fn name<RDF>() -> impl RDFNodeParse<RDF, Output = IriS>
where
    RDF: FocusRDF + 'static,
{
    SingleIriPropertyParser::new(sd_name().clone())
}

fn graph<RDF>() -> impl RDFNodeParse<RDF, Output = GraphDescription>
where
    RDF: FocusRDF + 'static,
    RDF::Term: TryInto<IriOrBlankNode> + From<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    FocusParser::new().then(|focus: RDF::Term| {
        trace!("Parsing graph at focus, parsing it...");
        let iri_or_blank: IriOrBlankNode = focus
            .try_into()
            .expect("Failed to convert Term to IriOrBlankNode");
        graph_description(iri_or_blank)
    })
}

pub fn parse_void_triples<RDF>(
    node: RDF::Term,
) -> impl RDFNodeParse<RDF, Output = Option<NumericLiteral>>
where
    RDF: FocusRDF + 'static,
    RDF::Term: Clone,
{
    SetFocusParser::new(node).with(
        SingleIntegerPropertyParser::new(void_triples().clone())
            .map(|n: isize| NumericLiteral::Integer(n as i128)) 
            .optional(),
    )
}

pub fn parse_void_classes<RDF>(
    node: RDF::Term,
) -> impl RDFNodeParse<RDF, Output = Option<NumericLiteral>>
where
    RDF: FocusRDF + 'static,
    RDF::Term: Clone,
{
    SetFocusParser::new(node).with(
        SingleIntegerPropertyParser::new(void_classes().clone())
            .map(|n: isize| NumericLiteral::Integer(n as i128)) 
            .optional(), 
    )
}

pub fn parse_void_class_partition<RDF>(
    node: RDF::Term,
) -> impl RDFNodeParse<RDF, Output = Vec<ClassPartition>>
where
    RDF: FocusRDF + 'static,
    RDF::Term: TryInto<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    SetFocusParser::new(node).with(class_partition().map_property(void_class_partition().clone()))
}

pub fn parse_void_property_partition<RDF>(
    node: RDF::Term,
) -> impl RDFNodeParse<RDF, Output = Vec<PropertyPartition>>
where
    RDF: FocusRDF + 'static,
    RDF::Term: TryInto<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    SetFocusParser::new(node).with(property_partition().map_property(void_property_partition().clone()))
}

pub fn class_partition<RDF>() -> impl RDFNodeParse<RDF, Output = ClassPartition>
where
    RDF: FocusRDF + 'static,
    RDF::Term: TryInto<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    trace!("parsing class_partition");
    FocusParser::new().then(move |focus: RDF::Term| {
        let focus_clone = focus.clone();
        SuccessParser::new(focus)
            .and(SingleIriPropertyParser::new(void_class().clone()))
            .and(property_partition().map_property(void_property().clone()))
            .map(move |((_, class), property_partition)| {
                let focus_iri: IriOrBlankNode = focus_clone.clone()
                    .try_into()
                    .expect("Failed to convert Term to IriOrBlankNode");
                ClassPartition::new(&class)
                    .with_id(&focus_iri)
                    .with_property_partition(property_partition)
            })
    })
}

pub fn property_partition<RDF>() -> impl RDFNodeParse<RDF, Output = crate::PropertyPartition>
where
    RDF: FocusRDF + 'static,
    RDF::Term: TryInto<IriOrBlankNode> + Clone,
    <RDF::Term as TryInto<IriOrBlankNode>>::Error: Debug,
{
    FocusParser::new()
        .and(SingleIriPropertyParser::new(void_property().clone()).map(|p| p.clone()))
        .and(
            SingleIntegerPropertyParser::new(void_triples().clone())
                .optional()
                .map(|opt: Option<isize>| opt.map(|n| NumericLiteral::Integer(n as i128))),
        )
        .map(|((focus, property), triples): ((RDF::Term, IriS), Option<NumericLiteral>)| {
            let focus_iri: IriOrBlankNode = focus
                .try_into()
                .expect("Failed to convert Term to IriOrBlankNode");
            PropertyPartition::new(&property)
                .with_id(&focus_iri)
                .with_triples(triples)
        })
}