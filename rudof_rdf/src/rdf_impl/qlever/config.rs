//! Configuration for the QLever container backend.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use super::decompressor::Compression;

/// Default upstream image
const DEFAULT_IMAGE_NAME: &str = "adfreiburg/qlever";
const DEFAULT_IMAGE_TAG: &str = "commit-a307781";

/// Default internal container port. Matches the upstream `adfreiburg/qlever` image's documented default.
const DEFAULT_CONTAINER_PORT: u16 = 7001;

/// Default working directory inside the container.
pub(crate) const CONTAINER_WORKING_DIR: &str = "/data";

/// Default index name when the user doesn't override it.
const DEFAULT_INDEX_NAME: &str = "default";

/// Default `-m` (server memory budget).
const DEFAULT_MEMORY_MAX_SIZE: &str = "5G";

/// Default `-c` (cache size).
const DEFAULT_CACHE_MAX_SIZE: &str = "2G";

/// Default `-e` (cache max size per single entry).
const DEFAULT_CACHE_MAX_SIZE_SINGLE_ENTRY: &str = "1G";

/// How long to wait for the server to answer a SPARQL probe before giving up.
pub(crate) const DEFAULT_SERVER_READINESS_TIMEOUT_SECS: u64 = 60;

/// Configuration for the QLever container backend.
///
/// Most fields are `Option<_>` so an empty TOML section is enough to boot QLever with its own defaults;
/// the few non-optional fields hardcoded defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct QleverConfig {
    /// Docker image name. Default: `adfreiburg/qlever`.
    pub image_name: String,

    /// Image tag. Default: `commit-a307781`.
    pub image_tag: String,

    /// Where on the host the index lives. `None` means `<dirs::cache_dir()>/rudof/qlever/<hash>`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_dir: Option<PathBuf>,

    /// Index base name (the `<name>` argument to `-i`). Default: `"default"`.
    pub index_name: String,

    /// When `true`, the index is wiped on `Drop` if this run created it.
    ///
    /// Default `false`: indexes persist across rudof invocations so a second
    /// run can skip the indexing step.
    pub auto_delete_if_created: bool,

    /// `-m` for the index builder (STXXL on-disk sort memory budget).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stxxl_memory: Option<String>,

    /// `--parser-buffer-size` for the index builder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser_buffer_size: Option<String>,

    /// `--parse-parallel` (alias `-p`) for the index builder.
    ///
    /// Default (when `None`): QLever's own default, which is `true` for a
    /// single input file. Set to `Some(false)` to drastically reduce the
    /// peak RAM of the indexing pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser_parallel: Option<bool>,

    /// Hard cap on the QLever container's RAM (Docker `--memory`).
    ///
    /// Accepts the usual human-readable suffixes (`"2G"`, `"512M"`,
    /// `"1.5GiB"`, `"1073741824"`). `None` means no cgroup limit, the
    /// container can consume the whole hosst.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_memory: Option<String>,

    /// Hard cap on RAM + swap (Docker `--memory-swap`).
    ///
    /// Set to the same value as [`container_memory`] to disable swap for
    /// this container (recommended on machines where swap is slow). `None`
    /// uses Docker's default (no swap cap).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_memory_swap: Option<String>,

    /// If `Some`, pin the host port to this value. `None` asks Docker for an
    /// ephemeral host port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<u16>,

    /// Container-side port the QLever server binds to. Default: `7001`.
    pub container_port: u16,

    /// `-a` access token for SPARQL UPDATE / privileged ops. Skipped on
    /// serialization when unset so secrets never round-trip through TOML
    /// dumps that only meant to surface non-secret config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,

    /// `-j` maximum number of simultaneous queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_simultaneous_queries: Option<u32>,

    /// `-m` server-side memory budget. Default: `"5G"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_max_size: Option<String>,

    /// `-c` cache size. Default: `"2G"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_max_size: Option<String>,

    /// `-e` cache size per single entry. Default: `"1G"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_max_size_single_entry: Option<String>,

    /// `-E` lazy result max cache size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_result_max_cache_size: Option<String>,

    /// `-k` max cache entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_max_num_entries: Option<u64>,

    /// `-P` disable pattern optimisation.
    pub no_patterns: bool,

    /// `-T` disable the pattern trick.
    pub no_pattern_trick: bool,

    /// `-t` enable text search.
    pub text: bool,

    /// `-o` only build the PSO and POS permutations.
    pub only_pso_and_pos_permutations: bool,

    /// `-s` default query timeout (e.g. `"30s"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_query_timeout: Option<String>,

    /// `-S` max value rows returned by `SERVICE` clauses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_max_value_rows: Option<u64>,

    /// `--throw-on-unbound-variables`.
    pub throw_on_unbound_variables: bool,

    /// Run the container as the host UID/GID so the index files end up owned
    /// by the user. No-op on non-Linux platforms.
    pub run_as_host_user: bool,

    /// Optional label applied to the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_label: Option<String>,

    /// Readiness probe timeout (seconds).
    pub server_readiness_timeout_secs: u64,
}

impl Default for QleverConfig {
    fn default() -> Self {
        Self {
            image_name: DEFAULT_IMAGE_NAME.to_string(),
            image_tag: DEFAULT_IMAGE_TAG.to_string(),
            index_dir: None,
            index_name: DEFAULT_INDEX_NAME.to_string(),
            auto_delete_if_created: false,
            stxxl_memory: None,
            parser_buffer_size: None,
            parser_parallel: None,
            container_memory: None,
            container_memory_swap: None,
            host_port: None,
            container_port: DEFAULT_CONTAINER_PORT,
            access_token: None,
            num_simultaneous_queries: None,
            memory_max_size: Some(DEFAULT_MEMORY_MAX_SIZE.to_string()),
            cache_max_size: Some(DEFAULT_CACHE_MAX_SIZE.to_string()),
            cache_max_size_single_entry: Some(DEFAULT_CACHE_MAX_SIZE_SINGLE_ENTRY.to_string()),
            lazy_result_max_cache_size: None,
            cache_max_num_entries: None,
            no_patterns: false,
            no_pattern_trick: false,
            text: false,
            only_pso_and_pos_permutations: false,
            default_query_timeout: None,
            service_max_value_rows: None,
            throw_on_unbound_variables: false,
            run_as_host_user: true,
            container_label: None,
            server_readiness_timeout_secs: DEFAULT_SERVER_READINESS_TIMEOUT_SECS,
        }
    }
}

impl QleverConfig {
    /// Convenience constructor (same as `Default::default()`).
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: set the index directory.
    pub fn with_index_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.index_dir = Some(path.into());
        self
    }

    /// Builder: set the index name.
    pub fn with_index_name(mut self, name: impl Into<String>) -> Self {
        self.index_name = name.into();
        self
    }

    /// Builder: set the image tag.
    pub fn with_image_tag(mut self, tag: impl Into<String>) -> Self {
        self.image_tag = tag.into();
        self
    }

    /// Builder: set the server memory budget (`-m`).
    pub fn with_memory_max_size(mut self, m: impl Into<String>) -> Self {
        self.memory_max_size = Some(m.into());
        self
    }

    /// Builder: pin the host port.
    pub fn with_host_port(mut self, port: u16) -> Self {
        self.host_port = Some(port);
        self
    }

    /// Builder: opt into auto-deleting the index on Drop.
    pub fn with_auto_delete(mut self, yes: bool) -> Self {
        self.auto_delete_if_created = yes;
        self
    }

    /// Builder: `-m` STXXL memory budget for the index builder.
    pub fn with_stxxl_memory(mut self, m: impl Into<String>) -> Self {
        self.stxxl_memory = Some(m.into());
        self
    }

    /// Builder: `--parser-buffer-size` for the index builder.
    pub fn with_parser_buffer_size(mut self, b: impl Into<String>) -> Self {
        self.parser_buffer_size = Some(b.into());
        self
    }

    /// Builder: `--parse-parallel`. Passing `false` is the most effective
    /// way to reduce the index builder's peak RAM.
    pub fn with_parser_parallel(mut self, parallel: bool) -> Self {
        self.parser_parallel = Some(parallel);
        self
    }

    /// Builder: cap the container's RAM (Docker `--memory`).
    pub fn with_container_memory(mut self, m: impl Into<String>) -> Self {
        self.container_memory = Some(m.into());
        self
    }

    /// Builder: cap the container's RAM + swap (Docker `--memory-swap`).
    pub fn with_container_memory_swap(mut self, m: impl Into<String>) -> Self {
        self.container_memory_swap = Some(m.into());
        self
    }

    /// Full image reference (`name:tag`).
    pub fn image(&self) -> String {
        format!("{}:{}", self.image_name, self.image_tag)
    }

    /// Resolve the index directory, falling back to a per-input cached path
    /// under the platform cache dir.
    pub fn resolve_index_dir(&self, fingerprint: &str) -> PathBuf {
        if let Some(p) = &self.index_dir {
            return p.clone();
        }
        let base = match std::env::var_os("XDG_CACHE_HOME") {
            Some(s) => PathBuf::from(s),
            None => match std::env::var_os("HOME") {
                Some(home) => PathBuf::from(home).join(".cache"),
                None => std::env::temp_dir(),
            },
        };
        base.join("rudof").join("qlever").join(fingerprint)
    }
}

/// An RDF file to feed to QLever's index builder.
#[derive(Debug, Clone)]
pub struct InputFile {
    /// Path to the file on the host as it will be mounted into the
    /// container. Must be inside the directory that gets bind-mounted to
    /// [`CONTAINER_WORKING_DIR`].
    pub host_path: PathBuf,

    /// File name as it appears inside the container, relative to `/data`.
    /// E.g. `"data.ttl"` or `"input-0.nt"`.
    pub in_container_name: String,

    /// QLever's `-F` flag (must be `"ttl"`, `"nt"`, or `"nq"`). When
    /// [`compression`] is set, this refers to the format of the DECOMPRESSED
    /// stream (e.g. `NTriples` for `dump.nt.bz2`).
    pub format_ext: NativeFormat,

    /// Optional `-g` graph IRI. `None` means default graph (passes `-`).
    pub graph_iri: Option<String>,

    /// `Some` if [`host_path`] is a compressed dump that must be decompressed
    /// on the host and streamed into the index builder via stdin. `None`
    /// (the default) means the file is fed via the usual bind-mount path.
    pub compression: Option<Compression>,
}

impl InputFile {
    /// Path as it appears inside the container, e.g. `/data/data.ttl`.
    pub fn container_path(&self) -> String {
        format!("{}/{}", CONTAINER_WORKING_DIR, self.in_container_name)
    }

    /// String to pass to `-g` (`-` for default graph).
    pub fn graph_arg(&self) -> &str {
        self.graph_iri.as_deref().unwrap_or("-")
    }
}

/// RDF formats QLever's index builder accepts natively.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeFormat {
    /// Turtle (`-F ttl`).
    Turtle,
    /// N-Triples (`-F nt`).
    NTriples,
    /// N-Quads (`-F nq`).
    NQuads,
}

impl NativeFormat {
    /// String passed to the `-F` CLI flag.
    pub fn cli_arg(&self) -> &'static str {
        match self {
            NativeFormat::Turtle => "ttl",
            NativeFormat::NTriples => "nt",
            NativeFormat::NQuads => "nq",
        }
    }

    /// Best-effort detection from a file extension.
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_ascii_lowercase();
        match ext.as_str() {
            "ttl" => Some(NativeFormat::Turtle),
            "nt" => Some(NativeFormat::NTriples),
            "nq" => Some(NativeFormat::NQuads),
            _ => None,
        }
    }
}
