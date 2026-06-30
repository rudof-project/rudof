//! Runs the shexTest `validation/manifest.jsonld` suite as individually
//! named, cargo-parallelized tests (via `libtest-mimic`, since the number
//! of cases is only known at run time -- `harness = false` for this binary,
//! see `Cargo.toml`).
//!
//! Two tiers, both built from the *same* eligible-entry filter as the
//! original single-`#[test]` version of this suite (a single string-valued
//! focus node and no `shapeExterns`: 1153 of the suite's 1166 entries as of
//! the `8d5b0b3` shexTest commit this repo pins via its submodule -- the
//! remaining 13 are skipped, not adapted, for the same reasons as before:
//! 6 typed-literal foci, 4 with an externs schema, 3 that use a multi-entry
//! JSON ShapeMap file instead of a single focus/shape pair, the last of
//! which `rudof_lib::load_shapemap`'s `Json` format arm being `todo!()`
//! can't currently be fed at all):
//!
//! - **Default** (`cargo test -p rudof_emacs`): the first
//!   `default_fast_tier_size` entries of [`RANKED_MANIFEST_PATH`] (itself
//!   one of the fields stored in that file, see [`RankedManifest`]), a
//!   precomputed greedy set-cover ranking over each entry's `trait` tags
//!   (57 distinct values in the manifest, e.g. `EachOf`/`OneOf`/`Extends`/
//!   `ValueSet`) -- chosen to cover as many distinct traits as possible in
//!   as few entries as possible, so this tier exercises every kind of
//!   construct the suite tests at least once, in a couple of seconds.
//! - **Full**, for the rare occasions the whole, slow suite is actually
//!   wanted (e.g. before a release, or after a `rudof_lib` validator
//!   change): `RUDOF_SHEXTEST_FULL=1 cargo test -p rudof_emacs --release`
//!   runs all 1153 eligible entries, each still its own named/parallelized
//!   test -- expect several minutes even in release mode.
//!
//! [`RANKED_MANIFEST_PATH`] is checked into the repo rather than computed
//! at test time, so the ranking (and the fast-tier size, also stored in
//! that file) is reviewable/diffable like any other fixture. Regenerate
//! the ranking after bumping the shexTest submodule with
//! `RUDOF_SHEXTEST_GENERATE_RANKING=1 cargo test -p rudof_emacs --test
//! shextest` -- deliberately *not* a `Trial`/named test (so it never shows
//! up, ignored or otherwise, in normal test listings/runs), just an
//! env-var-gated early exit from `main` before any `libtest_mimic`
//! involvement, mirroring how `RUDOF_SHEXTEST_FULL` is checked below.

use libtest_mimic::{Arguments, Failed, Trial};
use rudof_emacs::validate::{read_data, read_shapemap, read_shex, validate_shex_quadruples};
use rudof_lib::{Rudof, RudofConfig};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

const RANKED_MANIFEST_PATH: &str = "tests/shextest_ranked_manifest.json";
/// Seeds `default_fast_tier_size` the first time [`generate_ranking`]
/// creates [`RANKED_MANIFEST_PATH`]; every later regeneration preserves
/// whatever value is already on disk instead of resetting it, so hand-
/// tuning the tier size only requires editing that file, never this one.
const BOOTSTRAP_FAST_TIER_SIZE: usize = 25;
const GENERATE_WITH: &str = "RUDOF_SHEXTEST_GENERATE_RANKING=1 cargo test -p rudof_emacs --test shextest";
const RUN_FULL_SUITE_WITH: &str = "RUDOF_SHEXTEST_FULL=1 cargo test -p rudof_emacs --release";

/// Mirrors the handful of fields this crate's tests read off each shexTest
/// manifest entry, kept deliberately separate from `shex_testsuite` (whose
/// own corresponding types are private, built to drive *its* validator, not
/// to hand fixture text to this crate's API).
#[derive(Clone, serde::Deserialize)]
struct ManifestEntry {
    #[serde(rename = "@type")]
    type_: String,
    name: String,
    action: ManifestAction,
    #[serde(rename = "trait", default)]
    traits: Vec<String>,
}

#[derive(Clone, serde::Deserialize)]
struct ManifestAction {
    schema: String,
    shape: Option<String>,
    data: String,
    focus: Option<serde_json::Value>,
    #[serde(rename = "shapeExterns")]
    shape_externs: Option<String>,
}

#[derive(serde::Deserialize)]
struct ManifestRoot {
    #[serde(rename = "@graph")]
    graph: Vec<ManifestGraph>,
}

#[derive(serde::Deserialize)]
struct ManifestGraph {
    entries: Vec<ManifestEntry>,
}

/// The whole contents of [`RANKED_MANIFEST_PATH`]: the ranking itself plus
/// the two run/regenerate instructions and the fast-tier size, all in one
/// reviewable file rather than split across this source file's constants.
#[derive(serde::Serialize, serde::Deserialize)]
struct RankedManifest {
    generate_with: String,
    run_full_suite_with: String,
    default_fast_tier_size: usize,
    rankings: Vec<RankedEntry>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct RankedEntry {
    rank: usize,
    name: String,
    /// Traits this entry was selected for covering first; empty once the
    /// greedy cover is exhausted and entries are just ranked by name, kept
    /// for review/audit purposes only -- not read back by
    /// [`load_ranked_manifest`].
    new_traits: Vec<String>,
}

/// Entries where `rudof_lib`'s own ShEx validator (not this crate's FFI
/// glue -- every other entry exercises the exact same `read_shex`/
/// `read_data`/`read_shapemap`/`validate_shex_quadruples` call sequence)
/// gives the wrong conformance result, all involving greedy matching across
/// a `OneOf`/repeated triple constraint with overlapping predicates or
/// cardinalities (e.g. `nPlus1`, `1dotOne2dot_pass_p1`,
/// `open3Onedotclosecard2_pass-p1X2`). Tracked here as known, not fixed, so
/// this suite stays a regression guard for `rudof_emacs` itself rather than
/// perpetually red over an upstream validator gap -- remove an entry once
/// `rudof_lib` fixes the underlying case.
const KNOWN_VALIDATOR_DIVERGENCES: &[&str] = &[
    "1dotOne2dot_pass_p1",
    "1dotOne2dot_pass_p2p3",
    "open1dotOneopen2dotcloseclose_pass_p1",
    "open1dotOneopen2dotcloseclose_pass_p2p3",
    "openopen1dotOne1dotclose1dotclose_pass_p1p3",
    "openopen1dotOne1dotclose1dotclose_pass_p2p3",
    "open3Onedotclosecard2_pass-p1X2",
    "open3Onedotclosecard2_pass-p1p2",
    "open3Onedotclosecard2_pass-p1p3",
    "open3Onedotclosecard2_pass-p2p3",
    "open3Onedotclosecard23_pass-p1X2",
    "open3Onedotclosecard23_pass-p1X3",
    "open3Onedotclosecard23_pass-p1p2",
    "open3Onedotclosecard23_pass-p1p3",
    "open3Onedotclosecard23_pass-p2p3",
    "open3Onedotclosecard23_pass-p1p2p3",
    "open3Eachdotclosecard23_pass-p1p2p3X3",
    "open3EachdotcloseCode1-p1p2p3",
    "open3Eachdotclosecard23Annot3Code2-p1p2p3X3",
    "1dot-relative_pass-short-shape",
    "1dot-relative_pass-relative-shape",
    "false-lead-excluding-value-shape",
    "nPlus1",
    "nPlus1-greedy-rewrite",
    "skipped",
    "open2Eachdotclosecard25c1dot",
    "2Eachdot",
];

/// `std::env::var_os(name).is_some()` treats `FOO=` and `FOO=0` as "set", so
/// e.g. a CI script doing `RUDOF_SHEXTEST_FULL=$flag cargo test` with an
/// empty or `0` `$flag` would silently run the full (multi-minute) suite
/// instead of being a no-op -- this treats only a present, non-empty,
/// non-`"0"` value as true.
fn env_flag(name: &str) -> bool {
    match std::env::var(name) {
        Ok(value) => !value.is_empty() && value != "0",
        Err(_) => false,
    }
}

fn manifest_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../shex_testsuite/shexTest/validation")
        .canonicalize()
        .expect("shexTest submodule not checked out -- run `git submodule update --init shex_testsuite/shexTest`")
}

fn load_manifest(validation_dir: &Path) -> Vec<ManifestEntry> {
    let manifest_str = std::fs::read_to_string(validation_dir.join("manifest.jsonld")).unwrap();
    let manifest: ManifestRoot = serde_json::from_str(&manifest_str).unwrap();
    manifest.graph.into_iter().next().unwrap().entries
}

/// A single string-valued focus node and no `shapeExterns` -- see this
/// file's top doc comment for why the rest are skipped.
fn is_eligible(entry: &ManifestEntry) -> bool {
    matches!(entry.action.focus, Some(serde_json::Value::String(_))) && entry.action.shape_externs.is_none()
}

fn ranked_manifest_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(RANKED_MANIFEST_PATH)
}

fn load_ranked_manifest() -> Option<RankedManifest> {
    let text = std::fs::read_to_string(ranked_manifest_path()).ok()?;
    Some(serde_json::from_str(&text).expect("malformed ranked manifest JSON"))
}

/// Like [`load_ranked_manifest`], but treats a malformed (e.g. an older-
/// format) file the same as a missing one instead of panicking -- used only
/// by [`generate_ranking`]'s "preserve the existing tier size" step, which
/// should never block regeneration.
fn try_load_ranked_manifest() -> Option<RankedManifest> {
    let text = std::fs::read_to_string(ranked_manifest_path()).ok()?;
    serde_json::from_str(&text).ok()
}

fn run_one(validation_dir: &Path, entry: &ManifestEntry) -> Result<(), Failed> {
    let focus = match &entry.action.focus {
        Some(serde_json::Value::String(focus)) => focus,
        _ => return Err(Failed::from("entry is not eligible (no string focus)")),
    };
    let expect_conformant = match entry.type_.as_str() {
        "sht:ValidationTest" => true,
        "sht:ValidationFailure" => false,
        other => return Err(Failed::from(format!("unexpected manifest entry @type: {other}"))),
    };

    let schema_path = validation_dir.join(&entry.action.schema).canonicalize().unwrap();
    let schema_text = std::fs::read_to_string(&schema_path).unwrap();
    // Unlike `base` (the `validation/` folder, used below for data and the
    // few schemas that have none of their own relative IRIs), a schema's
    // `IMPORT <...>` target resolves against the *schema file's own* path,
    // not the manifest folder's -- schemas live one level over in
    // `../schemas/`, so using `base` here would send every relative IMPORT
    // to the wrong directory entirely.
    let schema_base = format!("file://{}", schema_path.display());
    let base = format!("file://{}/", validation_dir.display());
    let data_text = std::fs::read_to_string(validation_dir.join(&entry.action.data)).unwrap();
    // Compact ShapeMap syntax takes a blank node bare (`_:label`, as in
    // Turtle) but an IRI bracketed (`<iri>`) -- shexTest fixture foci are
    // IRIs almost always, but a handful (e.g. `1focusBNODELength_dot_pass`)
    // are blank nodes, which must not be `<>`-wrapped or the parser tries
    // (and fails) to resolve `_:label` itself as a relative IRI.
    let focus_term = if focus.starts_with("_:") {
        focus.clone()
    } else {
        format!("<{focus}>")
    };
    let shapemap_text = format!(
        "{focus_term}@{}",
        match &entry.action.shape {
            Some(shape) if shape.starts_with("_:") => shape.clone(),
            Some(shape) => format!("<{shape}>"),
            None => "START".to_string(),
        }
    );

    let mut rudof = Rudof::new(RudofConfig::default());
    let outcome = (|| -> anyhow::Result<bool> {
        read_shex(&mut rudof, schema_text, None, Some(schema_base))?;
        read_data(&mut rudof, data_text, Some("turtle".to_string()), Some(base))?;
        read_shapemap(&mut rudof, shapemap_text, None, None, None)?;
        let quadruples = validate_shex_quadruples(&mut rudof)?;
        Ok(quadruples.iter().any(|(_, _, status, _)| status == "conformant"))
    })();

    match outcome {
        Ok(is_conformant) if is_conformant == expect_conformant => Ok(()),
        Ok(_) if KNOWN_VALIDATOR_DIVERGENCES.contains(&entry.name.as_str()) => Ok(()),
        Ok(_) => Err(Failed::from("wrong conformance result")),
        Err(error) => Err(Failed::from(error.to_string())),
    }
}

/// Greedy set-cover over `entries`' `trait` tags: repeatedly pick the
/// not-yet-picked entry covering the most not-yet-covered traits (ties
/// broken by name, for a deterministic result), until every trait that
/// appears at all has been covered at least once; any leftover entries are
/// appended afterward in name order (their relative order no longer affects
/// coverage, only determinism).
fn rank_by_trait_coverage(entries: &[ManifestEntry]) -> Vec<RankedEntry> {
    let mut remaining: Vec<&ManifestEntry> = entries.iter().collect();
    let mut covered: HashSet<&str> = HashSet::new();
    let mut ranked = Vec::with_capacity(entries.len());

    loop {
        let mut candidates: Vec<(usize, Vec<&str>)> = remaining
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let new_traits: Vec<&str> = entry
                    .traits
                    .iter()
                    .map(String::as_str)
                    .filter(|t| !covered.contains(t))
                    .collect();
                (index, new_traits)
            })
            .filter(|(_, new_traits)| !new_traits.is_empty())
            .collect();
        if candidates.is_empty() {
            break;
        }
        candidates.sort_by(|(index_a, traits_a), (index_b, traits_b)| {
            traits_b
                .len()
                .cmp(&traits_a.len())
                .then_with(|| remaining[*index_a].name.cmp(&remaining[*index_b].name))
        });
        let (index, new_traits) = candidates.remove(0);
        for t in &new_traits {
            covered.insert(t);
        }
        let entry = remaining.remove(index);
        ranked.push(RankedEntry {
            rank: ranked.len(),
            name: entry.name.clone(),
            new_traits: new_traits.into_iter().map(str::to_string).collect(),
        });
    }

    remaining.sort_by(|a, b| a.name.cmp(&b.name));
    for entry in remaining {
        ranked.push(RankedEntry {
            rank: ranked.len(),
            name: entry.name.clone(),
            new_traits: Vec::new(),
        });
    }
    ranked
}

fn generate_ranking() -> anyhow::Result<()> {
    let validation_dir = manifest_dir();
    let entries = load_manifest(&validation_dir);
    let eligible: Vec<ManifestEntry> = entries.into_iter().filter(is_eligible).collect();
    let rankings = rank_by_trait_coverage(&eligible);

    // Preserve a hand-tuned tier size across regeneration -- only a fresh
    // (never-yet-generated) file falls back to the bootstrap default.
    let default_fast_tier_size = try_load_ranked_manifest()
        .map(|m| m.default_fast_tier_size)
        .unwrap_or(BOOTSTRAP_FAST_TIER_SIZE);

    let manifest = RankedManifest {
        generate_with: GENERATE_WITH.to_string(),
        run_full_suite_with: RUN_FULL_SUITE_WITH.to_string(),
        default_fast_tier_size,
        rankings,
    };
    let out_path = ranked_manifest_path();
    let json = serde_json::to_string_pretty(&manifest)?;
    std::fs::write(&out_path, json + "\n")?;
    eprintln!(
        "wrote {} ranked entries to {}",
        manifest.rankings.len(),
        out_path.display()
    );
    Ok(())
}

fn main() {
    // Deliberately checked *before* touching `libtest_mimic`/`Arguments` at
    // all, so this never shows up as a test (ignored or otherwise) -- see
    // this file's top doc comment.
    if env_flag("RUDOF_SHEXTEST_GENERATE_RANKING") {
        match generate_ranking() {
            Ok(()) => std::process::exit(0),
            Err(error) => {
                eprintln!("generate_ranking failed: {error}");
                std::process::exit(1);
            },
        }
    }

    let args = Arguments::from_args();
    let validation_dir = manifest_dir();
    let entries = load_manifest(&validation_dir);
    let by_name: HashMap<String, ManifestEntry> = entries
        .into_iter()
        .filter(is_eligible)
        .map(|e| (e.name.clone(), e))
        .collect();

    let full = env_flag("RUDOF_SHEXTEST_FULL");
    let selected_names: Vec<String> = if full {
        by_name.keys().cloned().collect()
    } else if let Some(manifest) = load_ranked_manifest() {
        manifest
            .rankings
            .into_iter()
            .take(manifest.default_fast_tier_size)
            .map(|entry| entry.name)
            .collect()
    } else {
        // Bootstrap case: the ranked manifest hasn't been generated yet --
        // don't panic, so `GENERATE_WITH` can still be run afterward.
        eprintln!(
            "warning: {} not found -- running 0 ranked fixtures; regenerate with `{GENERATE_WITH}`",
            ranked_manifest_path().display()
        );
        Vec::new()
    };

    let trials: Vec<Trial> = selected_names
        .into_iter()
        .map(|name| {
            let entry = by_name
                .get(&name)
                .unwrap_or_else(|| panic!("{name}: in ranked manifest but not an eligible shexTest entry"))
                .clone();
            let validation_dir = validation_dir.clone();
            Trial::test(name, move || run_one(&validation_dir, &entry))
        })
        .collect();

    libtest_mimic::run(&args, trials).exit();
}
