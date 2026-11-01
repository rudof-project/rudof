use crate::{
    Result, Rudof,
    errors::DataError,
    formats::{DataFormat, DataReaderMode, InputSpec},
    types::{DEFAULT_EXACT_SEARCH_LIMIT, DiffOptions, GraphDiff, GraphRelation, SubgraphMode},
    utils::{PrefixDirective, default_prefix_header, get_base_iri, rdf_data},
};
use rudof_iri::{IriS, MimeType};
use rudof_rdf::rdf_impl::OxigraphInMemory;
use std::borrow::Cow;
use std::io::{Cursor, Read};

/// Compares two RDF graphs, returning whether they are isomorphic,
/// equivalent, or one is contained in the other, up to blank node renaming.
/// Containment is checked according to `mode` (injective by default).
///
/// With no `data1`, the first graph is the currently loaded data, which must
/// be an in-memory graph; `format1` and `base1` are then ignored. The loaded
/// data is never modified.
#[allow(clippy::too_many_arguments)]
pub fn compare_data(
    rudof: &Rudof,
    data1: Option<&InputSpec>,
    data2: &InputSpec,
    format1: Option<&DataFormat>,
    format2: Option<&DataFormat>,
    base1: Option<&str>,
    base2: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    mode: Option<&SubgraphMode>,
) -> Result<GraphRelation> {
    let (graph1, graph2) = read_graphs(rudof, data1, data2, format1, format2, base1, base2, reader_mode)?;
    Ok(graph1.compare_with_mode(&graph2, mode.copied().unwrap_or_default()))
}

/// Like [`compare_data`], but also returns the triples of each graph that the
/// other one does not have. `exact_search_limit` bounds the search for the
/// best blank node mapping ([`DEFAULT_EXACT_SEARCH_LIMIT`] if None).
#[allow(clippy::too_many_arguments)]
pub fn diff_data(
    rudof: &Rudof,
    data1: Option<&InputSpec>,
    data2: &InputSpec,
    format1: Option<&DataFormat>,
    format2: Option<&DataFormat>,
    base1: Option<&str>,
    base2: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    mode: Option<&SubgraphMode>,
    exact_search_limit: Option<usize>,
) -> Result<GraphDiff> {
    let (graph1, graph2) = read_graphs(rudof, data1, data2, format1, format2, base1, base2, reader_mode)?;
    let options = DiffOptions {
        exact_search_limit: exact_search_limit.unwrap_or(DEFAULT_EXACT_SEARCH_LIMIT),
    };
    Ok(graph1.diff_with(&graph2, mode.copied().unwrap_or_default(), &options))
}

#[allow(clippy::too_many_arguments)]
fn read_graphs<'r>(
    rudof: &'r Rudof,
    data1: Option<&InputSpec>,
    data2: &InputSpec,
    format1: Option<&DataFormat>,
    format2: Option<&DataFormat>,
    base1: Option<&str>,
    base2: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
) -> Result<(Cow<'r, OxigraphInMemory>, OxigraphInMemory)> {
    let reader_mode = reader_mode.copied().unwrap_or_default();
    let graph1 = match data1 {
        Some(data1) => Cow::Owned(read_graph(
            rudof,
            data1,
            format1.copied().unwrap_or_default(),
            base1,
            reader_mode,
        )?),
        None => Cow::Borrowed(loaded_graph(rudof)?),
    };
    let graph2 = read_graph(rudof, data2, format2.copied().unwrap_or_default(), base2, reader_mode)?;
    Ok((graph1, graph2))
}

/// The loaded data, as long as it is an in-memory graph.
fn loaded_graph(rudof: &Rudof) -> Result<&OxigraphInMemory> {
    let backend = rdf_data(rudof)?.backend();
    backend.as_in_memory().ok_or_else(|| {
        Box::new(DataError::ComparisonNeedsInMemoryData {
            backend: backend.variant_name().to_string(),
        })
        .into()
    })
}

fn read_graph(
    rudof: &Rudof,
    input: &InputSpec,
    format: DataFormat,
    base: Option<&str>,
    reader_mode: DataReaderMode,
) -> Result<OxigraphInMemory> {
    let base: IriS = get_base_iri(rudof, base)?;
    let rdf_format = format.try_into()?;
    let source_name = input.source_name();
    let parse_error = |error: String| {
        Box::new(DataError::FailedParsingRdfData {
            source_name: source_name.clone(),
            format: format.to_string(),
            base: base.to_string(),
            reader_mode: reader_mode.to_string(),
            error,
        })
    };

    let mut reader = input.open_read(Some(format.mime_type()), "RDF data").map_err(|error| {
        Box::new(DataError::DataSourceSpec {
            message: format!("Failed to open data source '{source_name}': {error}"),
        })
    })?;
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .map_err(|error| parse_error(error.to_string()))?;

    // As in `load_data`, Turtle documents get the configured default prefixes
    if matches!(format, DataFormat::Turtle) {
        let header = default_prefix_header(rudof, &content, PrefixDirective::Turtle);
        content = format!("{header}{content}");
    }

    let graph = OxigraphInMemory::from_reader(
        &mut Cursor::new(content.into_bytes()),
        &source_name,
        &rdf_format,
        Some(base.as_str()),
        &reader_mode.into(),
    )
    .map_err(|error| parse_error(error.to_string()))?;
    Ok(graph)
}
