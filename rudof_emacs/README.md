# rudof_emacs

An [Emacs dynamic module](https://www.gnu.org/software/emacs/manual/html_node/elisp/Dynamic-Modules.html)
exposing rudof's ShEx conformance validation to Emacs Lisp, built with the
[`emacs`](https://crates.io/crates/emacs) crate.

## Building

```shell
cargo build --release -p rudof_emacs
```

This produces `target/release/librudof_emacs.{so,dylib,dll}` (the exact
extension depends on the platform; Emacs's `module-file-suffix` already
matches whichever one your build produces).

## Usage

The module exposes a *stateful* API: `rudof-emacs-new` creates an opaque
handle (a `user-ptr`) threaded through every other function as their first
argument, mirroring `python/src/pyrudof_lib.rs`'s own `read_data`/
`read_shex`/`read_shapemap`/`validate_shex` shape. Nothing about the
loaded schema/data/ShapeMap is ever represented in Lisp -- only the handle
is, plus the final, flattened validation-result triples.

```emacs-lisp
(module-load "/path/to/target/release/librudof_emacs.dylib")

(let ((rudof (rudof-emacs-new)))
  (rudof-emacs-read-shex
   rudof
   "PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    <http://example.org/PersonShape> { <http://example.org/age> xsd:integer }"
   "shexc" nil)
  (rudof-emacs-read-data
   rudof "<http://example.org/alice> <http://example.org/age> 30 ." "turtle" nil)
  ;; read-shapemap must come *after* read-shex/read-data -- see below.
  (rudof-emacs-read-shapemap
   rudof "<http://example.org/alice>@<http://example.org/PersonShape>" nil nil nil)
  (rudof-emacs-validate-shex rudof))
;; => (("http://example.org/alice" "http://example.org/PersonShape" "conformant" "..."))
```

`rudof-emacs-validate-shex` returns a flat list of `(node shape status
reason)` string quadruples (one per ShapeMap association), ready for e.g.
a flymake backend to walk directly -- no JSON/text parsing needed on the
Lisp side. `status` is one of `"conformant"`/`"nonconformant"`/`"pending"`/
`"inconsistent"`; `reason` is a human-readable explanation (e.g. why a node
failed to conform), usable as a diagnostic message as-is. Every function
signals a Lisp error (using rudof's own message) on malformed input or a
validation-setup failure (bad format
name, unparsable schema/data/ShapeMap, nothing loaded yet, etc.).

Format arguments accept the same names as the `rudof` CLI (e.g.
`"turtle"`/`"ntriples"`/`"jsonld"` for RDF data, `"shexc"`/`"shexj"` for
ShEx schemas, `"compact"` for ShapeMaps); passing `nil` uses each one's
own default (see each function's doc comment in `src/validate.rs`).

### Load order matters

`rudof_lib::Rudof::load_shapemap` itself requires data (and a schema) to
already be loaded, since a compact-syntax ShapeMap's node/shape selectors
are resolved against the data's/schema's currently-declared prefixes --
so **`read-shapemap` must come after both `read-shex` and `read-data`**,
not before. Once loaded, the ShapeMap no longer depends on the data
store, so re-reading data afterward (e.g. on every edit in a "data"
buffer) does not require re-reading the ShapeMap too -- only `read-data`
+ `validate-shex` need to be re-run, leaving the (typically much larger,
unchanged) schema and ShapeMap alone.

### `read-data` replaces, never accumulates

`rudof-emacs-read-data` always *replaces* whatever data was previously
loaded into the same handle -- this deliberately differs from
`rudof_lib::Rudof::load_data`'s own default of merging new data into
existing data. An editor buffer's *current* text should always become
the data exactly as written, not the previous text with the new text
silently appended underneath it on every edit.

## Testing

`cargo test -p rudof_emacs` runs the shexTest `validation/manifest.jsonld`
suite (requires the `shex_testsuite/shexTest` submodule: `git submodule
update --init shex_testsuite/shexTest`) as individually named, parallelized
tests, by default limited to a fixture subset chosen to exercise every
distinct `trait` tag in the manifest at least once -- a couple of seconds.

That subset, how many fixtures it contains, and the two commands below are
all stored in `tests/shextest_ranked_manifest.json` (a checked-in,
reviewable/diffable file, not computed at test time) -- edit its
`default_fast_tier_size` field directly to change the subset size; no code
change needed.

For the full 1153-fixture suite (expect several minutes even in
`--release`, so this isn't run by default):

```shell
RUDOF_SHEXTEST_FULL=1 cargo test -p rudof_emacs --release
```

After bumping the shexTest submodule, regenerate the ranking (preserves the
existing `default_fast_tier_size`):

```shell
RUDOF_SHEXTEST_GENERATE_RANKING=1 cargo test -p rudof_emacs --test shextest
```

## Why a dynamic module, not a C-ABI library

Plain Emacs Lisp has no built-in FFI for calling into an arbitrary C-ABI
shared library; `module-load` instead expects a library implementing
Emacs's own `emacs-module` ABI. The `emacs` crate generates that ABI glue
from ordinary `#[defun]`-annotated Rust functions, so this crate's own code
is just the validation logic itself (see `src/validate.rs`).
