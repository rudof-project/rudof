//! `QleverGraphContainer` an RDF graph served by a locally-launched QLever Docker container.
//!
//! The struct composes a [`OxigraphEndpoint`] internally.

use std::collections::HashSet;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use colored::Colorize;
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, NamedOrBlankNode as OxSubject,
    Term as OxTerm, Triple as OxTriple,
};
use prefixmap::{PrefixMap, PrefixMapError};
use rudof_iri::IriS;

use super::index_builder::{build_index, convert_to_native, fingerprint_inputs};
use super::server::QleverServer;
use super::{IndexHandle, InputFile, NativeFormat, QleverConfig, QleverError};
use crate::rdf_core::{
    Any, AsyncRDF, BuildRDF, FocusRDF, Matcher, NeighsRDF, RDFFormat, Rdf,
    query::{QueryRDF, QueryResultFormat, QuerySolution, QuerySolutions},
};
use crate::rdf_impl::OxigraphEndpoint;

/// An RDF graph served by a locally-launched QLever Docker container.
///
/// From a caller's perspective this is interchangeable with [`OxigraphInMemory`](crate::rdf_impl::OxigraphInMemory): it produces the same
/// `oxrdf` types and implements the same trait set, but the data lives in a QLever index on disk and is queried via the container's HTTP SPARQL
/// endpoint.
#[derive(Clone)]
pub struct QleverGraphContainer {
    /// Live server. `Arc` so cheap clones share the container.
    server: Arc<QleverServer>,

    /// `http://localhost:<mappedPort>/`.
    endpoint_iri: IriS,

    /// Underlying SPARQL-over-HTTP client.
    sparql: OxigraphEndpoint,

    /// Prefix map.
    prefixmap: Arc<PrefixMap>,

    /// Current focus term for [`FocusRDF`] operations.
    focus: Option<OxTerm>,
}

impl std::fmt::Debug for QleverGraphContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QleverGraphContainer")
            .field("endpoint", &self.endpoint_iri)
            .field("server", &self.server)
            .finish()
    }
}

impl QleverGraphContainer {
    /// Build an index from one or more files on disk and serve it.
    ///
    /// When `format` is `Some`, it overrides the file-extension sniffing applied to each path; non-native formats are transparently converted
    /// into N-Triples before indexing. When `None`, each path's format is guessed from its extension.
    pub async fn from_paths<P: AsRef<Path>>(
        paths: &[P],
        format: Option<&RDFFormat>,
        config: QleverConfig,
    ) -> Result<Self, QleverError> {
        if paths.is_empty() {
            return Err(QleverError::PreFlight("from_paths called with no paths".to_string()));
        }

        // Shared dir for any conversion output, hashed from all input paths so repeated runs reuse the same converted files.
        let convert_dir = config.resolve_index_dir(&fingerprint_for_paths(paths));
        std::fs::create_dir_all(&convert_dir).map_err(|error| QleverError::IndexDirIo {
            path: convert_dir.clone(),
            error,
        })?;

        let mut inputs = Vec::with_capacity(paths.len());
        for p in paths {
            let input = input_file_from_path(p.as_ref(), format, &convert_dir).await?;
            inputs.push(input);
        }

        // Capture `@prefix` / `PREFIX` declarations from the source files. QLever's HTTP endpoint does not surface them.
        let source_prefixes = collect_prefixes_from_paths(paths);

        let already_built = {
            let fp = fingerprint_inputs(&inputs);
            let dir = config.resolve_index_dir(&fp);
            super::IndexHandle::new(&dir, &config.index_name).is_built()
        };
        let handle = build_index(&inputs, &config).await?;
        let mut server = QleverServer::start(&handle, &config).await?;
        server.mark_created_index(!already_built);
        let container = Self::wrap(server).await?;
        Ok(if source_prefixes.map.is_empty() {
            container
        } else {
            container.with_prefixmap(source_prefixes)
        })
    }

    /// Build an index from a single file on disk and serve it.
    pub async fn from_path<P: AsRef<Path>>(path: P, config: QleverConfig) -> Result<Self, QleverError> {
        Self::from_paths(&[path], None, config).await
    }

    /// Open an EXISTING QLever index (skip indexing) and serve it.
    pub async fn open(index_dir: PathBuf, index_name: String, config: QleverConfig) -> Result<Self, QleverError> {
        let config = QleverConfig { index_name, ..config };
        let handle = IndexHandle::new(index_dir, &config.index_name);
        if !handle.is_built() {
            return Err(QleverError::PreFlight(format!(
                "no QLever index found at {} named {}",
                handle.path().display(),
                handle.name()
            )));
        }
        Self::serve(handle, config).await
    }

    /// Parse `read` according to `format`, write it to a temp file, convert to
    /// QLever's nearest native format if needed, then `from_path`.
    pub async fn from_reader<R: Read>(read: R, format: &RDFFormat, config: QleverConfig) -> Result<Self, QleverError> {
        let temp_dir = tempfile::tempdir()?;
        let src_path = temp_dir.path().join(format!("input{}", source_ext(format)));
        let mut src = std::fs::File::create(&src_path)?;
        copy_reader_to_file(read, &mut src)?;
        drop(src);

        let target_dir = config.resolve_index_dir(&fingerprint_for_path(&src_path));
        std::fs::create_dir_all(&target_dir).map_err(|error| QleverError::IndexDirIo {
            path: target_dir.clone(),
            error,
        })?;
        let (converted, _) = convert_to_native(&src_path, format, &target_dir).await?;
        let config = QleverConfig {
            index_dir: Some(target_dir.clone()),
            ..config
        };
        Self::from_path(&converted, config).await
    }

    async fn serve(handle: IndexHandle, config: QleverConfig) -> Result<Self, QleverError> {
        let server = QleverServer::start(&handle, &config).await?;
        Self::wrap(server).await
    }

    async fn wrap(server: QleverServer) -> Result<Self, QleverError> {
        let endpoint_iri = IriS::from_str(server.endpoint())?;
        let sparql = OxigraphEndpoint::new(&endpoint_iri, &PrefixMap::new())?;
        Ok(Self {
            server: Arc::new(server),
            endpoint_iri,
            sparql,
            prefixmap: Arc::new(PrefixMap::new()),
            focus: None,
        })
    }

    /// HTTP SPARQL endpoint URL - `http://host:port/`.
    pub fn endpoint_iri(&self) -> &IriS {
        &self.endpoint_iri
    }

    /// Async `SELECT` query against the QLever endpoint.
    pub async fn query_select_async(&self, query: &str) -> Result<QuerySolutions<Self>, QleverError> {
        let sols = self.sparql.query_select_async(query).await?;
        let pm = self.sparql.prefixmap().clone();
        let converted: Vec<QuerySolution<Self>> = sols.iter().map(|s| s.convert(|t| t.clone())).collect();
        Ok(QuerySolutions::new(converted, pm))
    }

    /// Async `CONSTRUCT` query.
    pub async fn query_construct_async(&self, query: &str, format: &QueryResultFormat) -> Result<String, QleverError> {
        Ok(self.sparql.query_construct_async(query, format).await?)
    }

    /// Async `ASK` query.
    pub async fn query_ask_async(&self, query: &str) -> Result<bool, QleverError> {
        Ok(self.sparql.query_ask_async(query).await?)
    }

    /// Underlying [`QleverServer`] (shared via `Arc`).
    pub fn server(&self) -> &QleverServer {
        &self.server
    }

    /// Replace the prefix map.
    pub fn with_prefixmap(mut self, pm: PrefixMap) -> Self {
        self.prefixmap = Arc::new(pm.clone());
        self.sparql = self.sparql.with_prefixmap(pm);
        self
    }

    /// Pretty-print a blank node (mirrors the inherent helper on the
    /// Oxigraph backends so `qualify_subject`/`qualify_term` produce
    /// identically-styled output across all three backends).
    #[inline]
    fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        bn.to_string().green().to_string()
    }

    /// Pretty-print a literal. See `show_blanknode` for rationale.
    #[inline]
    fn show_literal(&self, lit: &OxLiteral) -> String {
        lit.to_string().red().to_string()
    }
}

impl Rdf for QleverGraphContainer {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Triple = OxTriple;
    type Err = QleverError;

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        self.prefixmap.resolve_prefix_local(prefix, local)
    }

    fn qualify_iri(&self, node: &OxNamedNode) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap_or_else(|_| IriS::new_unchecked(node.as_str()));
        self.prefixmap.qualify(&iri)
    }

    fn qualify_subject(&self, subj: &OxSubject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_iri(n),
        }
    }

    fn qualify_term(&self, term: &OxTerm) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
            OxTerm::Triple(_) => unimplemented!("Triple terms not yet supported"),
        }
    }

    fn prefixmap(&self) -> Option<PrefixMap> {
        Some((*self.prefixmap).clone())
    }
}

impl AsyncRDF for QleverGraphContainer {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = QleverError;

    async fn get_predicates_subject(&self, subject: &OxSubject) -> Result<HashSet<OxNamedNode>, QleverError> {
        Ok(self.sparql.get_predicates_subject(subject).await?)
    }

    async fn get_objects_for_subject_predicate(
        &self,
        subject: &OxSubject,
        pred: &OxNamedNode,
    ) -> Result<HashSet<OxTerm>, QleverError> {
        Ok(self.sparql.get_objects_for_subject_predicate(subject, pred).await?)
    }

    async fn get_subjects_for_object_predicate(
        &self,
        object: &OxTerm,
        pred: &OxNamedNode,
    ) -> Result<HashSet<OxSubject>, QleverError> {
        Ok(self.sparql.get_subjects_for_object_predicate(object, pred).await?)
    }
}

impl NeighsRDF for QleverGraphContainer {
    fn triples(&self) -> Result<impl Iterator<Item = OxTriple>, QleverError> {
        self.triples_matching(&Any, &Any, &Any)
    }

    fn triples_matching<S, P, O>(
        &self,
        subject: &S,
        predicate: &P,
        object: &O,
    ) -> Result<impl Iterator<Item = OxTriple> + '_, QleverError>
    where
        S: Matcher<OxSubject>,
        P: Matcher<OxNamedNode>,
        O: Matcher<OxTerm>,
    {
        let iter = self.sparql.triples_matching(subject, predicate, object)?;
        Ok(iter)
    }
}

impl QueryRDF for QleverGraphContainer {
    fn query_construct(&self, query: &str, format: &QueryResultFormat) -> Result<String, QleverError> {
        Ok(self.sparql.query_construct(query, format)?)
    }

    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>, QleverError> {
        let sols = self.sparql.query_select(query)?;
        let pm = self.sparql.prefixmap().clone();
        let converted: Vec<QuerySolution<Self>> = sols.iter().map(|s| s.convert(|t| t.clone())).collect();
        Ok(QuerySolutions::new(converted, pm))
    }

    fn query_ask(&self, query: &str) -> Result<bool, QleverError> {
        Ok(self.sparql.query_ask(query)?)
    }
}

impl FocusRDF for QleverGraphContainer {
    fn set_focus(&mut self, focus: &OxTerm) {
        self.focus = Some(focus.clone());
    }

    fn get_focus(&self) -> Option<&OxTerm> {
        self.focus.as_ref()
    }
}

impl BuildRDF for QleverGraphContainer {
    fn empty() -> Self {
        panic!("QleverGraphContainer::empty() is not supported; use `from_path`/`open`/`from_reader`");
    }

    fn add_base(&mut self, _base: &Option<IriS>) {}

    fn add_prefix(&mut self, alias: &str, iri: &IriS) {
        let mut pm = (*self.prefixmap).clone();
        pm.add_prefix(alias, iri.clone());
        self.prefixmap = Arc::new(pm.clone());
        let new_sparql = self.sparql.clone().with_prefixmap(pm);
        self.sparql = new_sparql;
    }

    fn set_prefix_map(&mut self, prefix_map: PrefixMap) {
        self.prefixmap = Arc::new(prefix_map.clone());
        self.sparql = self.sparql.clone().with_prefixmap(prefix_map);
    }

    fn merge_prefixes(&mut self, prefix_map: PrefixMap) {
        let mut pm = (*self.prefixmap).clone();
        pm.merge(prefix_map);
        self.prefixmap = Arc::new(pm.clone());
        self.sparql = self.sparql.clone().with_prefixmap(pm);
    }

    fn add_triple<S, P, O>(&mut self, _subj: S, _pred: P, _obj: O) -> Result<(), QleverError>
    where
        S: Into<OxSubject>,
        P: Into<OxNamedNode>,
        O: Into<OxTerm>,
    {
        Err(QleverError::read_only("add_triple"))
    }

    fn remove_triple<S, P, O>(&mut self, _subj: S, _pred: P, _obj: O) -> Result<(), QleverError>
    where
        S: Into<OxSubject>,
        P: Into<OxNamedNode>,
        O: Into<OxTerm>,
    {
        Err(QleverError::read_only("remove_triple"))
    }

    fn add_type<S, T>(&mut self, _node: S, _type_: T) -> Result<(), QleverError>
    where
        S: Into<OxSubject>,
        T: Into<OxTerm>,
    {
        Err(QleverError::read_only("add_type"))
    }

    fn add_bnode(&mut self) -> Result<OxBlankNode, QleverError> {
        Err(QleverError::read_only("add_bnode"))
    }

    fn serialize<W: io::Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), QleverError> {
        use oxrdfio::RdfSerializer;

        let mut serializer = RdfSerializer::from_format(super::index_builder::rdf_format_to_oxrdfio(format));
        for (prefix, iri) in &self.prefixmap.map {
            serializer = serializer
                .with_prefix(prefix, iri.as_str())
                .map_err(|e| QleverError::FormatConversion {
                    source_name: self.endpoint_iri.to_string(),
                    error: format!("{e}"),
                })?;
        }

        let mut w = serializer.for_writer(writer);
        for triple in self.triples()? {
            w.serialize_triple(&triple).map_err(|e| QleverError::FormatConversion {
                source_name: self.endpoint_iri.to_string(),
                error: format!("{e}"),
            })?;
        }
        w.finish().map_err(|e| QleverError::FormatConversion {
            source_name: self.endpoint_iri.to_string(),
            error: format!("{e}"),
        })?;
        Ok(())
    }
}

/// Build a single `InputFile` from a path, honouring an explicit format if provided.
///
/// `convert_dir` is the directory under which any conversion output is written
/// (so the converted file outlives any temp dir the caller used).
///
/// Sources whose `RDFFormat` already matches one of QLever's three native
/// formats (`Turtle`, `NTriples`, `NQuads`) are passed through untouched —
/// QLever can index them directly via `-F`, so there's no point re-parsing
/// them through `oxrdfio`.
async fn input_file_from_path(
    path: &Path,
    format: Option<&RDFFormat>,
    convert_dir: &Path,
) -> Result<InputFile, QleverError> {
    let canonical = path.canonicalize().map_err(|error| QleverError::IndexDirIo {
        path: path.to_path_buf(),
        error,
    })?;

    let source_format = match format {
        Some(f) => *f,
        None => guess_format(&canonical).ok_or_else(|| {
            QleverError::PreFlight(format!("could not detect RDF format from path {}", canonical.display()))
        })?,
    };

    if let Some(native) = native_for_passthrough(&source_format) {
        return Ok(InputFile {
            host_path: canonical.clone(),
            in_container_name: canonical
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "data".to_string()),
            format_ext: native,
            graph_iri: None,
        });
    }

    let (converted, native) = convert_to_native(&canonical, &source_format, convert_dir).await?;
    let fallback_name = match native {
        NativeFormat::NQuads => "input.nq",
        NativeFormat::NTriples | NativeFormat::Turtle => "input.nt",
    };
    Ok(InputFile {
        host_path: converted.clone(),
        in_container_name: converted
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| fallback_name.to_string()),
        format_ext: native,
        graph_iri: None,
    })
}

/// If `format` is already a QLever-native format, return the matching
/// [`NativeFormat`] so the caller can skip conversion entirely.
fn native_for_passthrough(format: &RDFFormat) -> Option<NativeFormat> {
    match format {
        RDFFormat::Turtle => Some(NativeFormat::Turtle),
        RDFFormat::NTriples => Some(NativeFormat::NTriples),
        RDFFormat::NQuads => Some(NativeFormat::NQuads),
        _ => None,
    }
}

fn guess_format(path: &Path) -> Option<RDFFormat> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    match ext.as_str() {
        "ttl" => Some(RDFFormat::Turtle),
        "nt" => Some(RDFFormat::NTriples),
        "nq" => Some(RDFFormat::NQuads),
        "rdf" | "xml" | "owl" => Some(RDFFormat::Rdfxml),
        "jsonld" | "json" => Some(RDFFormat::JsonLd),
        "trig" => Some(RDFFormat::TriG),
        "n3" => Some(RDFFormat::N3),
        _ => None,
    }
}

fn source_ext(format: &RDFFormat) -> &'static str {
    match format {
        RDFFormat::Turtle => ".ttl",
        RDFFormat::NTriples => ".nt",
        RDFFormat::NQuads => ".nq",
        RDFFormat::Rdfxml => ".rdf",
        RDFFormat::JsonLd => ".jsonld",
        RDFFormat::TriG => ".trig",
        RDFFormat::N3 => ".n3",
    }
}

fn copy_reader_to_file<R: Read>(mut read: R, file: &mut std::fs::File) -> io::Result<()> {
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = read.read(&mut buf)?;
        if n == 0 {
            break;
        }
        std::io::Write::write_all(file, &buf[..n])?;
    }
    std::io::Write::flush(file)
}

fn fingerprint_for_path(path: &Path) -> String {
    let mut h = super::index_builder::Fnv1a::new();
    h.write_path(path);
    format!("{:016x}", h.finish())
}

fn fingerprint_for_paths<P: AsRef<Path>>(paths: &[P]) -> String {
    let mut h = super::index_builder::Fnv1a::new();
    for p in paths {
        h.write_path(p.as_ref());
    }
    format!("{:016x}", h.finish())
}

/// Best-effort prefix extraction for every source path.
///
/// QLever indexes triples but its HTTP endpoint does not surface the `@prefix` / `PREFIX` declarations from the source files.
fn collect_prefixes_from_paths<P: AsRef<Path>>(paths: &[P]) -> PrefixMap {
    let mut combined = PrefixMap::new();
    for p in paths {
        let path = p.as_ref();
        let Some(format) = guess_format(path) else { continue };
        if matches!(format, RDFFormat::NTriples | RDFFormat::NQuads) {
            // No prefix declarations in line-based formats.
            continue;
        }
        if let Ok(bytes) = std::fs::read(path) {
            harvest_prefixes(&bytes, &format, &mut combined);
        }
    }
    combined
}

fn harvest_prefixes(bytes: &[u8], format: &RDFFormat, into: &mut PrefixMap) {
    use oxrdfio::RdfParser;

    let ox_format = super::index_builder::rdf_format_to_oxrdfio(format);
    let mut parser = RdfParser::from_format(ox_format).for_reader(bytes);
    // Drive the parser so `prefixes()` is populated. Stop on the first
    // error so a malformed tail of the file doesn't poison the lookup.
    for quad in parser.by_ref() {
        if quad.is_err() {
            break;
        }
    }
    for (prefix, iri) in parser.prefixes() {
        if let Ok(iri_s) = IriS::from_str(iri) {
            into.add_prefix(prefix, iri_s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// With an explicit `RDFFormat::Turtle`, a file with a non-`.ttl` extension
    /// is still treated as Turtle (the format override beats extension sniffing).
    #[tokio::test]
    async fn explicit_format_overrides_extension() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("input.dat");
        std::fs::write(&src, b"<http://x/a> <http://x/p> <http://x/b> .\n").unwrap();

        let input = input_file_from_path(&src, Some(&RDFFormat::NTriples), tmp.path())
            .await
            .expect("should treat .dat as N-Triples");
        assert_eq!(input.format_ext, NativeFormat::NTriples);
        assert_eq!(input.host_path, src.canonicalize().unwrap());
    }

    /// Without an explicit format and with an unknown extension, indexing
    /// should fail fast with a PreFlight error instead of feeding garbage to
    /// the converter.
    #[tokio::test]
    async fn unknown_extension_without_format_fails_preflight() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("input.xyz");
        std::fs::write(&src, b"").unwrap();

        let err = input_file_from_path(&src, None, tmp.path())
            .await
            .expect_err("unknown extension should fail");
        assert!(matches!(err, QleverError::PreFlight(_)), "got: {err:?}");
    }

    /// QLever indexes Turtle natively (`-F ttl`), so a `.ttl` input must be
    /// passed through without re-parsing.
    #[tokio::test]
    async fn turtle_is_passed_through_without_conversion() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("data.ttl");
        std::fs::write(&src, b"@prefix : <http://x/> .\n:a :p :b .\n").unwrap();

        let input = input_file_from_path(&src, None, tmp.path()).await.unwrap();
        assert_eq!(input.format_ext, NativeFormat::Turtle);
        assert_eq!(
            input.host_path,
            src.canonicalize().unwrap(),
            "Turtle should be mounted as-is, not converted"
        );
    }

    /// N-Quads is native too; passing through preserves the named graphs.
    #[tokio::test]
    async fn nquads_is_passed_through_without_conversion() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("data.nq");
        std::fs::write(&src, b"<http://x/a> <http://x/p> <http://x/b> <http://x/g> .\n").unwrap();

        let input = input_file_from_path(&src, None, tmp.path()).await.unwrap();
        assert_eq!(input.format_ext, NativeFormat::NQuads);
        assert_eq!(input.host_path, src.canonicalize().unwrap());
    }

    /// TriG carries named graphs that N-Triples cannot represent. The
    /// converter must target N-Quads so the graph IRI survives.
    #[tokio::test]
    async fn trig_converts_to_nquads_preserving_graph() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("data.trig");
        std::fs::write(&src, b"@prefix : <http://x/> .\n:g { :a :p :b . }\n").unwrap();

        let input = input_file_from_path(&src, None, tmp.path()).await.unwrap();
        assert_eq!(
            input.format_ext,
            NativeFormat::NQuads,
            "quad-bearing source must convert to NQ, not NT"
        );
        assert!(
            input.host_path.extension().and_then(|e| e.to_str()) == Some("nq"),
            "converted file should have .nq extension, got: {:?}",
            input.host_path
        );

        let converted = std::fs::read_to_string(&input.host_path).unwrap();
        assert!(
            converted.contains("<http://x/g>"),
            "named graph IRI lost in conversion: {converted}"
        );
    }
}
