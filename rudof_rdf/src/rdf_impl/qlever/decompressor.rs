//! Host-side decompressors for streaming compressed RDF dumps into QLever.

use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Describes one compression family. Implementations are zero-sized and return only `&'static` references.
pub trait CompressionStrategy: Debug + Send + Sync + 'static {
    /// Family identifier used in diagnostics, error messages, and as the fingerprint salt. E.g. `"bz2"`, `"xz"`.
    fn family_name(&self) -> &'static str;

    /// File-extension suffix without the leading dot (e.g. `"bz2"`).
    fn extension(&self) -> &'static str;

    /// Candidate binaries in priority order. The first whose `program` resolves on `$PATH` is used.
    fn candidates(&self) -> &'static [DecompressorCandidate];
}

/// One candidate binary that can decompress this family to stdout.
#[derive(Debug, Clone, Copy)]
pub struct DecompressorCandidate {
    /// Binary name as it appears on `$PATH` (e.g. `"lbzip2"`).
    pub program: &'static str,
    /// Arguments that come BEFORE the input file path on the command line. All current candidates use `["-dc"]` or `["-dc", "-T0"]`.
    pub args: &'static [&'static str],
    /// `true` if this candidate is a parallel implementation. Used purely for diagnostics so logs can tell the user which one was picked.
    pub parallel: bool,
}

/// User-facing handle for a compression family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Compression {
    Bzip2,
    Xz,
    Gzip,
}

impl Compression {
    /// The strategy trait object for this family.
    pub fn strategy(self) -> &'static dyn CompressionStrategy {
        match self {
            Compression::Bzip2 => &Bzip2Strategy,
            Compression::Xz => &XzStrategy,
            Compression::Gzip => &GzipStrategy,
        }
    }
}

/// All registered strategies.
fn strategies() -> &'static [&'static dyn CompressionStrategy] {
    &[&Bzip2Strategy, &XzStrategy, &GzipStrategy]
}

/// bzip2 family: `lbzip2` (parallel) → `lbzcat` (parallel) → `bzip2`(single-threaded).
#[derive(Debug)]
pub struct Bzip2Strategy;

impl CompressionStrategy for Bzip2Strategy {
    fn family_name(&self) -> &'static str {
        "bz2"
    }
    fn extension(&self) -> &'static str {
        "bz2"
    }
    fn candidates(&self) -> &'static [DecompressorCandidate] {
        &[
            DecompressorCandidate {
                program: "lbzip2",
                args: &["-dc"],
                parallel: true,
            },
            DecompressorCandidate {
                program: "lbzcat",
                args: &["-dc"],
                parallel: true,
            },
            DecompressorCandidate {
                program: "bzip2",
                args: &["-dc"],
                parallel: false,
            },
        ]
    }
}

/// xz family: `xz -dc -T0` → `xzcat -T0`. `-T0` is opportunistic, it parallelises decompression only when the source was compressed in
/// independent blocks (`xz -T <n>`), but is harmless otherwise.
#[derive(Debug)]
pub struct XzStrategy;

impl CompressionStrategy for XzStrategy {
    fn family_name(&self) -> &'static str {
        "xz"
    }
    fn extension(&self) -> &'static str {
        "xz"
    }
    fn candidates(&self) -> &'static [DecompressorCandidate] {
        &[
            DecompressorCandidate {
                program: "xz",
                args: &["-dc", "-T0"],
                parallel: true,
            },
            DecompressorCandidate {
                program: "xzcat",
                args: &["-T0"],
                parallel: true,
            },
        ]
    }
}

/// gzip family: `pigz` (parallel) → `gzip` (single-threaded) → `zcat` (alias for `gzip -dc` on most distros).
#[derive(Debug)]
pub struct GzipStrategy;

impl CompressionStrategy for GzipStrategy {
    fn family_name(&self) -> &'static str {
        "gz"
    }
    fn extension(&self) -> &'static str {
        "gz"
    }
    fn candidates(&self) -> &'static [DecompressorCandidate] {
        &[
            DecompressorCandidate {
                program: "pigz",
                args: &["-dc"],
                parallel: true,
            },
            DecompressorCandidate {
                program: "gzip",
                args: &["-dc"],
                parallel: false,
            },
            DecompressorCandidate {
                program: "zcat",
                args: &[],
                parallel: false,
            },
        ]
    }
}

/// One concrete decompressor binary that the probe resolved on `$PATH`.
#[derive(Debug, Clone)]
pub struct ResolvedDecompressor {
    /// Absolute path to the binary.
    pub program: PathBuf,
    /// Arguments to put before the input file path.
    pub args: &'static [&'static str],
    /// `true` if this is a parallel implementation (diagnostics only).
    pub parallel: bool,
    /// Family name (for diagnostics and error messages).
    pub family: &'static str,
}

/// Process-wide cache of which decompressor binary each family resolved to.
#[derive(Debug)]
pub struct DecompressorProbe {
    /// Keyed by [`CompressionStrategy::family_name`]. Missing key means no candidate for that family was found on `$PATH`.
    resolved: HashMap<&'static str, ResolvedDecompressor>,
}

impl DecompressorProbe {
    /// Lookup by strategy trait object.
    pub fn for_strategy(&self, s: &dyn CompressionStrategy) -> Option<&ResolvedDecompressor> {
        self.resolved.get(s.family_name())
    }

    /// Lookup by [`Compression`] enum (convenience).
    pub fn for_compression(&self, c: Compression) -> Option<&ResolvedDecompressor> {
        self.for_strategy(c.strategy())
    }
}

/// Process-wide decompressor probe. Populated on first call and reused forever after.
pub fn decompressor_probe() -> &'static DecompressorProbe {
    static PROBE: OnceLock<DecompressorProbe> = OnceLock::new();
    PROBE.get_or_init(|| {
        let mut resolved: HashMap<&'static str, ResolvedDecompressor> = HashMap::new();
        for strategy in strategies() {
            for cand in strategy.candidates() {
                if let Some(abs) = find_on_path(cand.program) {
                    resolved.insert(
                        strategy.family_name(),
                        ResolvedDecompressor {
                            program: abs,
                            args: cand.args,
                            parallel: cand.parallel,
                            family: strategy.family_name(),
                        },
                    );
                    break;
                }
            }
        }
        DecompressorProbe { resolved }
    })
}

/// Strip a known compression suffix and report which family matched. Returns `None` if `path` does not end with any registered strategy's
/// extension.
pub fn strip_compression_suffix(path: &Path) -> Option<(PathBuf, Compression)> {
    let s = path.to_str()?;
    for compression in [Compression::Bzip2, Compression::Xz, Compression::Gzip] {
        let suffix = format!(".{}", compression.strategy().extension());
        if let Some(stem) = s.strip_suffix(&suffix) {
            return Some((PathBuf::from(stem), compression));
        }
    }
    None
}

/// Resolve `program` against `$PATH`. Returns the absolute path of the first existing entry whose file metadata is a regular file with any
/// executable bit set.
fn find_on_path(program: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(program);
        if is_executable_file(&candidate) {
            return Some(candidate);
        }
    }
    None
}

#[cfg(unix)]
fn is_executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    match std::fs::metadata(path) {
        Ok(meta) => meta.is_file() && (meta.permissions().mode() & 0o111) != 0,
        Err(_) => false,
    }
}

#[cfg(not(unix))]
fn is_executable_file(path: &Path) -> bool {
    std::fs::metadata(path).map(|m| m.is_file()).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_compression_suffix_handles_known_extensions() {
        let (inner, c) = strip_compression_suffix(Path::new("/data/dump.nt.bz2")).unwrap();
        assert_eq!(inner, Path::new("/data/dump.nt"));
        assert_eq!(c, Compression::Bzip2);

        let (inner, c) = strip_compression_suffix(Path::new("/data/dump.ttl.bz2")).unwrap();
        assert_eq!(inner, Path::new("/data/dump.ttl"));
        assert_eq!(c, Compression::Bzip2);

        let (inner, c) = strip_compression_suffix(Path::new("/data/dump.nq.xz")).unwrap();
        assert_eq!(inner, Path::new("/data/dump.nq"));
        assert_eq!(c, Compression::Xz);

        let (inner, c) = strip_compression_suffix(Path::new("/data/dump.nt.gz")).unwrap();
        assert_eq!(inner, Path::new("/data/dump.nt"));
        assert_eq!(c, Compression::Gzip);

        // Non-compressed paths return None.
        assert!(strip_compression_suffix(Path::new("/data/dump.nt")).is_none());
        assert!(strip_compression_suffix(Path::new("/data/dump.txt")).is_none());

        // `.jsonld.bz2` matches the bz2 suffix and yields an inner
        // `.jsonld` path — non-native rejection happens downstream in
        // `input_file_from_path`, not here.
        let (inner, c) = strip_compression_suffix(Path::new("/data/dump.jsonld.bz2")).unwrap();
        assert_eq!(inner, Path::new("/data/dump.jsonld"));
        assert_eq!(c, Compression::Bzip2);
    }

    #[test]
    fn strategy_registry_contains_bz2_xz_and_gz_with_unique_extensions() {
        let all = strategies();
        let families: Vec<_> = all.iter().map(|s| s.family_name()).collect();
        assert!(families.contains(&"bz2"));
        assert!(families.contains(&"xz"));
        assert!(families.contains(&"gz"));

        let mut exts: Vec<_> = all.iter().map(|s| s.extension()).collect();
        exts.sort_unstable();
        let original_len = exts.len();
        exts.dedup();
        assert_eq!(exts.len(), original_len, "strategy extensions must be unique");
    }

    #[test]
    fn compression_strategy_round_trip() {
        assert_eq!(Compression::Bzip2.strategy().family_name(), "bz2");
        assert_eq!(Compression::Xz.strategy().family_name(), "xz");
        assert_eq!(Compression::Gzip.strategy().family_name(), "gz");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn probe_finds_bzip2_xz_and_gzip_on_linux() {
        // bzip2, xz, and gzip ship on every standard Linux base image and CI runner.
        let probe = decompressor_probe();
        assert!(
            probe.for_compression(Compression::Bzip2).is_some(),
            "expected at least one bzip2-family decompressor on PATH"
        );
        assert!(
            probe.for_compression(Compression::Xz).is_some(),
            "expected at least one xz-family decompressor on PATH"
        );
        assert!(
            probe.for_compression(Compression::Gzip).is_some(),
            "expected at least one gzip-family decompressor on PATH"
        );
    }
}
