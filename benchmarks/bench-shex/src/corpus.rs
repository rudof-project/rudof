use anyhow::{Context, Result};
use rudof_iri::IriS;
use serde::Deserialize;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub schema_root: PathBuf,
    pub case: Vec<CaseSpec>,
}

#[derive(Debug, Deserialize)]
pub struct CaseSpec {
    pub id: String,
    pub schema: PathBuf,
    pub data: PathBuf,
    pub shapemap: PathBuf,
    #[serde(default)]
    pub size: Size,
}

#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Size {
    #[default]
    Small,
    Large,
}

impl Size {
    pub fn tag(self) -> &'static str {
        match self {
            Size::Small => "small",
            Size::Large => "large",
        }
    }
}

/// Fully-materialized inputs for a single test case.
pub struct Case {
    pub id: String,
    pub size: Size,
    pub schema_path: PathBuf,
    pub data_path: PathBuf,
    pub shapemap_path: PathBuf,
    pub schema_src: String,
    pub data_src: String,
    pub shapemap_src: String,
    pub base: IriS,
    pub source_iri: IriS,
}

pub fn load_corpus(manifest_path: &Path) -> anyhow::Result<Vec<Case>> {
    let manifest_text =
        std::fs::read_to_string(manifest_path).with_context(|| format!("read mainfest {}", manifest_path.display()))?;
    let manifest: Manifest =
        toml::from_str(&manifest_text).with_context(|| format!("parse manifest {}", manifest_path.display()))?;

    let manifest_dir = manifest_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("manifest has no parent dir: {}", manifest_path.display()))?
        .canonicalize()
        .with_context(|| format!("canonicalize {}", manifest_path.display()))?;
    let root = manifest_dir.join(&manifest.schema_root);

    manifest.case.into_iter().map(|spec| load_case(spec, &root)).collect()
}

fn load_case(spec: CaseSpec, root: &Path) -> anyhow::Result<Case> {
    let schema_path = root
        .join(&spec.schema)
        .canonicalize()
        .with_context(|| format!("canonicalize schema {}", spec.schema.display()))?;
    let data_path = root
        .join(&spec.data)
        .canonicalize()
        .with_context(|| format!("canonicalize data {}", spec.data.display()))?;
    let shapemap_path = root
        .join(&spec.shapemap)
        .canonicalize()
        .with_context(|| format!("canonicalize shapemap {}", spec.shapemap.display()))?;

    let schema_src =
        std::fs::read_to_string(&schema_path).with_context(|| format!("read schema {}", schema_path.display()))?;
    let data_src = std::fs::read_to_string(&data_path).with_context(|| format!("read data {}", data_path.display()))?;
    let shapemap_src = std::fs::read_to_string(&shapemap_path)
        .with_context(|| format!("read shapemap {}", shapemap_path.display()))?;

    let schema_dir = schema_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("schema has no parent dir: {}", schema_path.display()))?;
    let base_str = format!("file://{}/", schema_dir.display());
    let base = IriS::new(&base_str).with_context(|| format!("create base IRI from {}", base_str))?;

    let source_iri_str = format!("file://{}", schema_path.display());
    let source_iri =
        IriS::new(&source_iri_str).with_context(|| format!("create source IRI from {}", source_iri_str))?;

    Ok(Case {
        id: spec.id,
        size: spec.size,
        schema_path,
        data_path,
        shapemap_path,
        schema_src,
        data_src,
        shapemap_src,
        base,
        source_iri,
    })
}

pub fn load_all() -> Result<Vec<Case>> {
    let corpus_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let extract_root = extract_root();

    let small_dir = ensure_extracted(&corpus_root.join("small.zip"), &extract_root.join("small"))?;
    let large_dir = ensure_extracted(&corpus_root.join("large.zip"), &extract_root.join("large"))?;

    let small = load_corpus(&small_dir.join("manifest.toml"))?;
    let large = load_corpus(&large_dir.join("manifest.toml"))?;
    Ok(small.into_iter().chain(large).collect())
}

/// Extract `zip_path` into `target` if the manifest marker is missing. Returns `target`.
fn ensure_extracted(zip_path: &Path, target: &Path) -> Result<PathBuf> {
    if target.join("manifest.toml").exists() {
        return Ok(target.to_path_buf());
    }
    std::fs::create_dir_all(target).with_context(|| format!("create {}", target.display()))?;
    let file = File::open(zip_path).with_context(|| format!("open {}", zip_path.display()))?;
    let mut archive = zip::ZipArchive::new(file).with_context(|| format!("read zip {}", zip_path.display()))?;
    archive
        .extract(target)
        .with_context(|| format!("extract into {}", target.display()))?;
    Ok(target.to_path_buf())
}

fn extract_root() -> PathBuf {
    if let Ok(dir) = std::env::var("CARGO_TARGET_TMPDIR") {
        return PathBuf::from(dir).join("bench-shex-corpus");
    }
    if let Ok(dir) = std::env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(dir).join("bench-shex-corpus");
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/bench-shex-corpus")
}
