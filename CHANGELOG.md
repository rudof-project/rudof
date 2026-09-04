# CHANGE LOG
This ChangeLog follows the Keep a ChangeLog guidelines](https://keepachangelog.com/).

## [Unreleased]
### Added
### Fixed
### Changed
### Removed

## 0.3.18 
### Fixed

Fix release/artifact builds broken by the lbug dependency

lbug (LadybugDB) imposes platform floors that broke both artifact jobs:

- Linux: it only ships glibc prebuilt static libs and needs C++20
  <format>, unavailable in the musl-cross-make toolchains. Build
  Dockerfile_rudof against x86_64-unknown-linux-gnu instead of musl,
  and ship a slim ubuntu:26.04 runtime image (matching the builder's
  glibc) instead of FROM scratch, with the runtime libs lbug/openssl
  need (libssl3, libstdc++6, zlib1g, libzstd1).

- macOS: building lbug from source hits std::format, whose
  floating-point to_chars/from_chars Apple's libc++ only supports
  from macOS 13.3 onward. Bump artifacts.yml's mac job
  MACOSX_DEPLOYMENT_TARGET from 12.0 to 13.3.

Also fix Dockerfile_rudof's ENTRYPOINT, which used exec-form and never
actually expanded $BINARY_NAME so the image could never run; and fix
the `as`/`AS` FROM casing lint warning.

## 0.3.17

The comments below gather changes between 0.3.14 and 0.3.17.

### Added
- `connect <db>` command: opens (or creates) a database and stores the connection details in a TOML file (default `.rudof-connection.toml`) so stateless commands can reuse them (discussions #747, #748). `--backend <BACKEND>` selects the backend (default and, for now, only connectable value: `lbug`/LadybugDB) and is persisted in that file, so more backends can be added later without another CLI-shape change.
- `--backend` (already shared by `data`/`node`/`query`/`shacl`/`shacl-validate`/`shex-validate`/`validate`) is now also the flag `connect` uses, replacing the pg_db-specific `--engine`/`DbEngine`, which have been removed — `lbug` is now one more value of the same `BackendSpec` type/`--backend` flag rather than a separate concept. `--backend lbug` is accepted everywhere `--backend` is, but on RDF-loading commands it's a deliberate not-yet-implemented seam: rudof can derive a property graph from RDF (`load`/`ddl`) but can't yet read one back out as RDF, so selecting `lbug` there gives a clear, specific error rather than a silent no-op or a generic "wrong backend" message — see [discussion #747](https://github.com/rudof-project/rudof/discussions/747).
- `ddl` command: derives a property graph schema from RDF data and emits DDL for the `cypher` or `gql` dialect (`--dialect cypher|gql`), without touching a database.
- `load` command: validates RDF data against SHACL shapes and copies it into a LadybugDB property graph database (node/rel tables derived from the data; `--skip-validation` copies without validating). Replaces the earlier `lbug load-shacl`.
- `query --dialect cypher` mode to run a Cypher query (given via `-q`/`--query`, same flag as SPARQL — file, URL, `-`, or inline text) against a LadybugDB database selected via `--db <PATH>` or a connection details file. `--dialect` defaults to `sparql`.
- `connect`/`ddl`/`load`/`query --dialect cypher` are now documented and covered by tests as `rudof shell` subcommands too (see [shell docs](docs/src/cli_usage/shell.md#working-with-a-ladybugdb-database)); no code changes were needed since the shell already dispatches through the same command enum as the top-level CLI.
- Python bindings gained the LadybugDB property graph database integration: `connect_pg_db`, `pg_db_ddl`, `load_pg_db`, `query_cypher` (returning a native Python object via `pythonize` rather than a Rust debug string), plus `DbEngine`/`DdlDialect` enums. This closed a functional-parity gap: the CLI/shell versions of these commands were implemented directly in `rudof_cli` against the `lbug` crate and were unreachable from Python; the underlying logic now lives in `rudof_lib` (`rudof_lib/src/api/pg_db/`, following the same operations-trait/builder/implementation pattern as `pgschema`/`shacl`) and `rudof_cli`'s commands were refactored into thin callers of it. `load`'s SHACL validation now reuses `rudof_lib`'s existing `load_shacl_shapes`/`validate_shacl` operations instead of a hand-rolled duplicate. `query --dialect cypher`'s row output changed from Rust's `{:?}` debug format to one JSON object per line.
- SHACL validation report can now record *why* a `(node, shape)` pair conforms, not just why it fails. Two new `[shacl]` config keys (`store_errors`, `store_evidences` — see [Config reference](docs/src/references/config.md#shacl--shacl-validation)), settable via `--no-errors`/`--with-evidences` on `shacl-validate`/`validate`, independently control what ends up in the `ValidationReport` (4 combinations; default unchanged: violations only). The `-r details`/`-r compact` table output shows evidence rows marked `Conforms` in green alongside the violation rows, including one per-shape summary row per conforming `(node, shape)` pair (e.g. "conforms to `:PersonShape`") in addition to the finer per-constraint-component evidence (`sh:datatype`, `sh:minCount`, ...) — `evidence_shapes_only`/`--evidences-shapes-only` keeps just the per-shape summaries when the full detail is too much. Internally, the SHACL validator's `(node, shape)` memoization cache was migrated to `rudof_typing`, the same generic cache abstraction the ShEx validator already used.
- SHACL now supports recursive (self-referencing) shapes — a shape whose property must itself conform to the same (or another) shape, previously always rejected as a "dependency graph has cycles" error, even for schemas where the cycle could never actually be reached by real data. `[shacl].recursion_semantics`/`--recursion-semantics` (`shacl-validate`/`validate`) selects how a cyclic shape reference is resolved: `cautious` (default) — least fixpoint, a node caught in an otherwise-ungrounded cycle does not conform; `brave` — greatest fixpoint, it does conform as long as that's self-consistent; `none` — reject the shapes graph outright, restoring the old behavior. A cycle that also involves a negating constraint (`sh:not`, `sh:xone`, ...) is supported too, as long as it's *stratified*: every such constraint must target a shape that doesn't itself depend on any recursion, so its answer can be settled independently of the recursive part — the classical Datalog stratification condition. A cycle where negation instead reaches back into a recursive shape (its own, or a different one's) has no well-defined order to resolve it in, and is still always rejected, under every recursion semantics. See [Recursive shapes](docs/src/cli_usage/shacl_validate.md#recursive-shapes).
- `shacl -r internal` now reports each shape's recursion status — not recursive, positive recursive, stratified recursive, or non-stratified recursive — in a "Recursive shapes" section, alongside the existing shapes-graph/dependency-graph dump.

## 0.3.13
### Added
- `shex-validate --strategy dereference`: for Wikibase endpoints (Wikidata, MaRDI), answer lookups by dereferencing each entity's IRI over HTTP instead of SPARQL.
- `[logging]` config section (`logging.level`), settable at runtime via the shell's `config set`, to change the tracing filter without restarting.
- Ctrl-C in the shell now cancels only the running command; a second Ctrl-C still force-exits. Session state and the shell itself survive.
- Python bindings now cover PG Schema, prefix management, and RDF-config, plus `check_shex`, precompiled ShEx schema caching, and `dereference`; CLI/shell gained a matching `shex-check` command, a `resolvers` shell command, and finer-grained `reset` targets (`shex-validation`/`shacl-validation`/`pgschema-validation`).

### Fixed
- Python bindings' `reset_validation_results()` no longer silently wipes the loaded ShEx schema; it and the new `reset_shacl_validation`/`reset_pgschema_validation` now clearly document and correctly scope what they clear.
- SPARQL requests now send a policy-compliant User-Agent, avoiding Wikidata's restrictive anonymous-scraping throttling tier; `Retry-After` on a 429 is honored in full instead of capped, and the client backs off proactively after one.
- SPARQL retries now also cover connection timeouts/resets, not just HTTP error statuses.
- `-s`/`-m` CLI arguments accept prefixed names (e.g. `es:E371`) like the shell already did, instead of erroring "file does not exist".

### Changed
- `outgoing_arcs_from_list`/`incoming_arcs_from_list` now issue a UNION of fully-bound triple patterns instead of `FILTER(?p IN (...))`, which SPARQL engines optimize far better.
- Default `--max-steps` raised from 100 to 1000.
- Progress logging for long-running SPARQL/dereference operations moved from `info!` to `debug!`, scoped to rudof's own crates by default.

### Removed

## 0.3.11
- Shell: Tab now lists all matching subcommand names instead of silently cycling through them, and completes the `endpoint` command's argument against registered endpoint names, `config`'s own subcommand against `get`/`set`, and `config get`/`config set`'s `KEY` argument against every dotted key path in the effective config.
- Shell: `data`, `shex`, `shacl`, `dctap`, `pgschema` and `service` now print a short stats line (e.g. "N triples loaded") when loading something new, instead of dumping the full content; a bare call still shows what's loaded in full.
- Shell: `data FILE --merge` merges into the currently loaded RDF/PG data instead of replacing it, which is now the shell's default.
- Added a `sparql` command to load and show a SPARQL query without running it; `query` now runs a query loaded this way when called bare.
- Added a "Config reference" page to the docs, documenting every `rudof.toml` setting.
- Added `RdfDataConfig::load_endpoint_description` to register a SPARQL endpoint (under its own `name` field) from a local TOML file, and `endpoint FILE.toml` in the shell to register and activate one this way.
- Shell: a resource argument (`shex -s`, `shacl -s`, `sparql -q`, `shapemap -m`, ...) can be a prefixed name (`es:E10`) instead of a full URL, expanded against the session's default prefixes or the active endpoint's own prefixes before loading — e.g. `endpoint wikidata` then `shex es:E10` loads `https://www.wikidata.org/wiki/Special:EntitySchemaText/E10`.
- ShEx schemas can now be serialized as RDF (Turtle, N-Triples, RDF/XML, TriG, N3, N-Quads), using the ShExR vocabulary — e.g. `rudof shex -s schema.shex -r turtle`, or in the shell, `shex schema.shex` then `shex -r turtle`. Available everywhere a ShEx result format can be chosen: the CLI, the shell, the MCP server's `shex` tool, and the Python bindings (`ShExFormat.Turtle`/`NTriples`/`RdfXml`/`TriG`/`N3`/`NQuads`). The generated RDF always declares `sx:` for the ShExR vocabulary, plus whatever prefixes the source ShExC schema itself declared.
- Added the reverse direction too: a ShEx schema can now be *loaded* from ShExR RDF (Turtle, N-Triples, RDF/XML, N-Quads) as well as serialized to it — e.g. `rudof shex -s schema.ttl -f turtle` — available everywhere a ShEx schema can be loaded: the CLI, the shell, the MCP server's `show_shex`/`check_shex` tools, and the Python bindings.

### Fixed
- Fixed ShEx validation against a live SPARQL endpoint (e.g. `endpoint wikidata` + `shex-validate`) being able to hang forever: incoming-arc lookups now fetch only the predicates a shape actually needs (instead of a node's entire incoming-triple set), SPARQL requests carry a timeout, and transient 502/503/504 responses are retried like 429 already was. `shex-validate --max-steps` (previously parsed but unused) and the engine's step limit (previously configured but never checked) now actually abort validation with a clear error instead of running unbounded.
- Fixed `pgschema`'s "Labels do not match" error printing the record's own labels on both sides, making every label mismatch look like identical strings instead of showing the type's actual expected labels.
- Fixed the ShEx pretty-printer omitting a schema's trailing newline, which could make the shell overwrite the last line of a `shex`/`shacl` command's output with the next prompt.
- Shell: a panic raised deep inside any command (a bug, as opposed to a normal error) is now caught, reported like any other error, and the session keeps running instead of the whole shell crashing.
- Fixed error messages (in the shell and the top-level CLI) being printed with their text duplicated — several error types embed their full wrapped-cause text into their own message, which used to be repeated a second time by the generic "print every link in the cause chain" formatting; each genuinely new piece of information is now printed once, on its own line.
- The leading "Error:" label on error messages (in the shell and the top-level CLI) is now printed in red when the target stream supports color, following the same `NO_COLOR`/TTY detection already used elsewhere.
- Fixed `RdfVocabulary::base_iri()` (used by `RdfVocab`, `ShexRVocab`, `XsdVocab`, etc.) caching its result in a `static` shared across *all* vocabularies instead of one per vocabulary, so whichever vocabulary ran first in the process silently won for every other one — corrupting, e.g., ShExR's `sx:` prefix depending on call order.

### Changed
- The default `Wikidata`/`DBpedia`/`UniProt` endpoints are now defined as TOML files under `rudof_rdf/endpoints/`, embedded at compile time, instead of hardcoded in Rust — every `*.toml` file added there is registered automatically, so adding or editing a default endpoint no longer requires a code change.
- `EndpointDescription` gained a required `name` field, which is now the registered key (rather than a config-file table key or file stem); endpoint name matching (`-e`/`--endpoint`, the shell's `endpoint` command and its completion) is case-insensitive, so `wikidata`, `Wikidata` and `WikiData` all resolve the same endpoint.

### Removed
- Removed `RdfDataConfig::with_wikidata`/`with_dbpedia`/`with_uniprot` (superseded by the TOML-file-backed defaults above); use `load_endpoint_description`/`with_endpoints` instead.

## 0.3.10
### Added
- `prefixes` functionality added to shell so one can interatively manage default prefix declarations adding new ones, removing new ones or changing existing ones.
- Handling multiline input in the shell so it is possible to use the shell to define data, queries or shapes that require multiple lines. 

### Fixed
- Fixes issue #730: exponential ShEx validation time on nested/chained shapes. `Engine::prove` now memoizes `(node, shape)` proof results across recursive calls, so each pair is proved at most once instead of once per path. Proofs that assumed a pair still being proved higher up the recursion stack are excluded from the cache, so validation results are unchanged.

### Changed
- The default output of shell is now in ShExC format and in SHACL it is Turtle to accomodate to the popular formats.
- Allow native SHACL builds without the SPARQL builds. PR #771

### Removed

## 0.3.9 

### Added
- Added `rudof shell`, an interactive REPL that keeps loaded data/schema/etc. across commands, with `endpoint` and `reset` built-in commands.
- Added `--generator-config` to `rudof generate` for fine-grained data generation settings (entity distribution, cardinality, field generators, ...).

### Fixed
- fix(shapemap fail color): It was printing the default FAIL color in Green instead of Red
- Fixed default Wikidata/basic prefix maps using `https://` instead of `http://`, which broke resolving `wd:`, `rdf:`, `xsd:`, etc. against real data.
- Fixed `reset` leaving a stale compiled ShEx validator behind, letting a shapemap load silently succeed against an already-reset schema.
- Fixed `rudof_generate`'s config file requiring fields that were documented as optional, breaking minimal config files.

### Changed
- We moved NoMatchReason to its own module

## 0.3.8

### Added
- We refactored the way configuration files work in rudof as described in [this PR](https://github.com/rudof-project/rudof/pull/749) with a layered system

### Changed
- We replaced the dependency from `bincode` by `postcard` #745

### Fixed
- Solves stack overflow with FHIR files #739
- Improves some of the ShEx error messages solving issue #742
- Improves the non-deterministic behaviour of SHACL with negative cycles detection solving issue #735

## 0.3.7
### Added
- Adds support for disjunctions in ShEx extends
- Now SHACL `NativeEngine` validator parses `sh:sparql` with `sparql` feature (#707)
- Added SHACL Path serialization (#728)
### Fixed
- Fixed issue #724 extra line breaks in ShEx validation output
- Fixed multiple SPARQL constraints for SHACL validation (#707)
- Fixed ShEx validation output (#733, #734)
- Fixes non-deterministic behavior of SHACL negative cycles detection (#735)
### Changed
- Solves issue #721. Change the behaviour with recursive shapes in SHACL to give an error. 
  In the future we should attempt recursive shapes with non-negative cycles and use least-fixpoint semantics as suggested in the paper.

### Removed

## v0.3.6

### Fixed
- fix(sparql): fix SPARQL endpoint validation and add rate limiting and caching
- fix(cli): fix CLI to allow node command with endpoint backend and no data input, and add --endpoint/-e shortcut for endpoint backend
- fix(shex): resolve shapemap prefixes from data/schema sources
- fix(sparql): fix SPARQL endpoint validation and add rate limiting and caching

### Added
- feat(rbe): report mandatory values in empty-candidates errors
- feat(shex,rbe): explain why triple expression matching failed
- feat(config): add missing `genid` prefix to the default config.
- feat(shex): report why references failed in FailedPending errors

## v0.3.5
### Added
- #706: added emacs bindings by @ericprud
### Fixed
- #708: allow node command with endpoint backend and no data input, and add --endpoint/-e shortcut for endpoint backend
- #698: Add more information about error messages in ShEx validation

## v0.3.4
### Added
- #533: Add rudof MCP server to Model Context Protocol registry
### Fixed
- #703: Replaced independent tokio runtimes with a shared one

## v0.3.3
### Added
- #682: QLever Docker backend + RdfBackend strategy pattern

### Fixed
- #688: Issues with negative cycles, blank node focus nodes, and IRI regex matching in ShEx
- #655: Support for JSON and CSV formats in SPARQL SELECT Queries

## v0.3.2
### Added
- #660: Added support to Nix
- #672: Support for SPARQL-based constraints in SHACL (SHACL)
- #685: Validation of edge types in PGSchema (pgschema), changed option to validate pgschema so it is `pgschema-validate` instead of `pg-schema-validate`.
- #633: Support for Start semantic actions in ShEx (ShEx)
- #663: Pattern flags (ShEx)
- #667: Support for ShapeExternal (EXTERN) delegation in the validator (ShEx)
- #676: External shape resolver selection in CLI / MCP / Python
- #652: Solved issue with Extends in ShEx (ShEx)

### Fixed
- #665: Language stem BCP-47 subtag matching (shex)
- #676: FRACTIONDIGITS facet rejects decimals with trailing zeros (shex)
- #678: EXTRA predicates not consulted during ShEx validation (non-matching arcs incorrectly fed to RBE)
- #680: Relative IRI shape labels not resolved in ShEx validation testsuite

### Changed
- #669: Improve MCP metadata

## v0.3.1
### Added
Tests for stats on rudof_generate and upgrade stats.json with the information of the dataset #653

## v0.2.19
### Added
- #624: Allow prefixes when converting to turtle issue 
- #645: Implement ^ inverse in ShEx validator
- #635: Add IriNormalizationMode with lax/strict parsing
- #585: Implement the inverval algorithm for optimal RBE checking

### Fixed
- Solves issue #647 by adding a method `set_default_base_prefixes` which sets the default base in `PrefixMap`. It also changes the behaviour of `qualify` in `PrefixMap` to use the default base and show the IRIs that belong to the default base in relative mode.
- Bug with min cardinality was not passed down, now is fixed solving issue #648
- Solves issue #635 allowing node parameters without angle brackets
- Solves issue #626 to allow importing relative IRIs
- Fixes some dependencies on pgschema issues #619, #618 and #617
- Solves issue #648: Fix bug min cardinality in rudof generate
- Solves issue #643: fix(shex_validation): numeric value set term equality + XSD lexical form validation
- Solves issue #641: Shape label not found in a ShEx with a relative IRI as start
- Fixed SHACL error handling
- Enhanced arc references in MemoryGraph
- Fixed regex in InputSpec builder to accept win absolute paths

### Changed
### Removed

## v0.2.18
### Added
- Now its possible to add new prefixes when converting data with the data command
- Added an IriRegistry trait to allow us to index IriS in some rudof subcrates

### Fixed
- Solves issue #629: IMPORT with relative IRIs when loading local ShEx schemas

## v0.2.17
- Solves issues #617, #618 and #618 about updating rustemo in PGSchema, the old documentation of pgschema binary and an un-needed dependency on test-digg in PGSchema. We renamed the `pgschema/src/main.rs` as a candidate to be removed because in principle we should unify the CLI for PGSchema features in the rudof binary to avoid a multiplication of binaries. The `pgschema/src/docs` should be removed or integrated in the `docs` folder.
- Solves issue #616 initializing the store using `check_store` so it is possible to do SPARQL queries to resolve shape maps when validating ShEx.
- Several SHACL performance improvements, including SHACL validation algorithm parallelization

## v0.2.16

### Fixed
- Solves issue #584 by changing `InputSpec` and allowing three different behaviours:
    - Function `InputSpec::str(...)` can be used to create a plain string
    - Function `parse_from_str` can be used to get an `InputSpec` from a string. It has a new parameter `allow_plain_str` which if `false` will complain if the file doesn't exist, and if `true` allows a plain string. 
    - Implements `FromStr` assuming but not allowing plain strings to avoid confusing errors.

## v0.2.15

### Changed

- Semantics of labels in `pgschema` now works as a set of sets of labels which is converted to disjunctive normal form. The pgschema parser has been adapted to use groupings in labels like `(A | B) & (B | C)`.

## v0.2.14

### Changed
- Updated some dependencies
- Now `iri_s` crate has been moved to `rudof_iri`

## v0.2.13
### Fixed
- Problem with MCP panicking, issue #604

### Changed
- Unified `shacl_ast`, `shacl_ir`, `shacl_rdf` and `shacl_validation` crates into a single `shacl` crate 

## v0.2.12

### Added
- `rudof_generate`: Added generation-time conformance metrics in `*.stats.json` with:
  - `triple_validity_percentage`
  - `shape_translation_loss_percentage`
- `rudof_generate`: Added conversion coverage accounting for ShEx/SHACL to unified constraints (`original_schema_constraints` and `represented_constraints_in_unified`).
- `rudof_generate`: Added new runnable conformance scenarios under `rudof_generate/examples/conformance_cases`.
- Implemented import in ShEx solving issue #589
- Support to materialize #574 which now can be used in combination with the Map extension in ShEx to transform RDF data into other RDF data using source and target shapes.
### Fixed
- Regression in command line that didn't accept either data or endpoint
- Regression in CLI that didn't merge RDF files by default. Solves also issue #595

### Changed
- `rudof_generate`: Updated README statistics example and coverage notes to document the new conformance metrics output.
### Removed


## v0.2.11

### Fixed

- Solved issue #582 with an error in RBE
- Solved issue #583 about a regression in CLI accepting shapemap as optional parameter

## v0.2.10

- This release was not done because there was a change quickfix in the middle

## v0.2.9

### Added
- Added documentation for the `rudof_lib` crate (#455)
- Added documentation for the `python` crate (#430)
- Added option to visualize ShEx internal representation
- Added option to visualize ShEx as JSON and JSONLD (the same as ShExJ)
- First implementation of semantic actions with two extensions: Test (https://shex.io/extensions/Test/) and Map (http://shex.io/extensions/Map), issue #570
- Integrated SHEx testsuite in CI #563

### Fixed
- Fixed issue #577: MCP server failed to initialize in VS Code due to a strict initialization sequence
- Fixed issue #530: Reviewed and refactored Python examples
- Fixed issue #548: Corrected `NumericLiteral` equality to compare numeric values instead of variant types (resolves ShEx test suite failures)
- Fixed issue #550: Resolved ShEx lexer bug where floats were tokenized as integers, breaking parsing
- Fixed issue #575: Where inheritance between shapes that had recursion were not taken into account
- Fixed issue #571 allowing to pass a context with subject, predicate and object in semantic actions

### Changed
- Refactored `rudof_lib` crate (#455); updated Python and `rudof_mcp` crates to use the new API
- Updated `pyrudof` Jupyter notebook tutorials to align with the new API (#577)

## v0.2.8

### Added
- Support for more than one value in ValueShape in DCTAP
- Added support to visualizing more comples Shapes wihch can have references in ShEx and DCTAP

### Fixed
- Small error in DCTAP with prefix maps in value shapes 
- Removed 2 unwraps in convert that panicked when the formats to convert to ShEx were wrong

## v0.2.7 

### Fixed
- Removed unneeded dependency on `rust_decimal_macros` from `rudof_rdf`
- Solved issue #531: ShEx validation fails when using multiple a [...] constraints for different classes in the same shape
- Solved issues #526, #520, #506 and #513 about prefix maprelative IRIs issue and automatic Python docs examples generation with tests
- Refactored Internal CLI implementation: #418, #433

### Added
- Added command to generate Shell completion scripts #524
- Added more query formats: #523
- Added support to dereference shapemaps against a base IRI #521

## v0.2.6
### Fixed
- Relative IRIs are now parsed consistently in shapemaps, RDF data and ShEx solving issue #520
- All the python default values are configured properly (as is stated in the docs).
Also fixed an issue related #516 to remove an openssl dependency.
- Fixes #514.

### Changed
- Option `show-schema` has been changed to use the negatable pattern with `show_schema` as a flag that can be overriden by `no_show_schema` which seems more ergonomic.
- trait `Deref` renamed as `DerefIri` and method `deref` as `deref_iri` to avoid conflicts with the existing `deref` keyword

## v0.2.5

### Fixed 

Repaires Jobs for macos publishing

## v0.2.3/v0.2.4

### Fixed

Change in docker publishing artifact so it works. Issue: https://github.com/rudof-project/rudof/issues/505

## v0.2.2
### Added
- Now the shex validator continues validating a shapemap in case a shape label is not found. The previous behaviour failed. Issue #502

### Changed
- shex_validator: validate_shapemap2 renamed to validate_shapemap

## v0.2.1
### Added
- New methods in Python's prefixmap so it can be tested and we publish a new version

### Changed
- Shapemaps allow an optional trailing comma instead, [issue 501](https://github.com/rudof-project/rudof/issues/501)

## v0.2.0 

We created a new minor release 0.2.0 to reset the number (0.1.149) and to reflect that this is the first release contains several contributions from 2 new core developers: @samuel-bustamante and @algarfer

### Added
- Option validation mode = PGSchema in rudof_cli

### Fixed
- Regression bug in handling of prefixmap declarations: https://github.com/rudof-project/rudof/issues/497
- Unimplemented behaviour in regular bag expressions like `(a;b)*` has now been implemented

### Changed
### Removed

## v0.1.148

### Added
- Improved rudof_generate

## v0.1.147

### Added
### Fixed
### Changed

- Syntax of PGSchema data has been aligned with [YARSPG](https://github.com/lszeremeta/yarspg)

### Removed

## v0.1.146
### Fixed
- Small bug that was opening an interactive window when running plantuml commands

## v0.1.145
## Added
- Visualization of ShEx results as CSV
- Visualization of PGSchema validation results as CSV

## Fixed
- Some extra info! messages

## v0.1.144
## Fixed
- Bug in node information when depth was bigger to 1 which was ignoring predicates

## v0.1.143
## Added
- SHACL tools for MCP server
- ShEx tools for MCP server

## Fixed
- Support for YARSPG grammar in Property graphs
- Improved MCP server to follow best practices

## Changed
- Removed several todo!s in shex2uml converter

## v0.1.142
## Added
- Support for SPARQL extension of shapemaps and node selectors
- Support for triple patterns in shapemaps and node selectors

## v0.1.141
## Added
- `node` information accepts a new parameter `outgoing_depth` to declare the number of neighbours from a node that are shown

## Fixed
- Improved the error messages when there is no PLANTUML variable declaration

## v0.1.140
### Added
- Methods to return the list of validation results in SHACL and to get information from those results

## v0.1.139
### Added
- Visualization of subject, predicate and objects in triple terms now have customizable colors and styles solving issue #381

## v0.1.138
### Added
- Added experimental feature to support Property Graphs and Property graph schemas (PGSchema)

### Fixed
- Recovered the option to get tracing information from RUST_LOG variable
- Improved the visualization of SHACL validation results which were not presenting the path in property shapes

## v0.1.137
### Added
- More documentation in MCP server
- First support for recursive shapes in SHACL. At this moment, the compiler detects recursive SHACL and classifies in stratified schemas (non-negative cycles) and non-stratified schemas (negative cycles) whose semantics can be more difficult to implement. It also does a first implementation of recursive schemas, which is not yet well tested. This can solve issue #238
- Issue solved: #358 MCP server for ShEx validation, progress toward: #325

### Fixed

### Changed
- GraphValidation::from_path now takes as argument an `AsRef<Path>`
- The SHACL internal representation now has a dependency graph

## v0.1.137
- Improved information about errors when reading SHACL shapes graphs
- Added checking on recursive shapes

## v0.1.136
### Added
- Documentation to MCP server

### Fixed
- `node` indicates if a node is not found in the RDF data

## v0.1.135
### Added
- SHACL 1.2, added first version with suppport for `sh:reifierShape` and `sh:reificationRequired`
- MCP_SERVER: added documentation
- Python bindings: added bindings for Rudof_Generate

## v0.1.134
### Fixed

- Issue with SPARQL queries that used annotation syntax updating Oxigraph to 0.5.2

## v0.1.132
### Added
- Parameter `-e` to specify templates-folder in shex2html and dctap2html conversion

## v0.1.131
### Added
- Support for query in MCP_Server

### Fixed
- trimming shape ids and property ids in DCTAP to support extra whitespaces
- Added `_:` to show blank nodes in SPARQL results

## v0.1.130
### Added
- Added `query` tools to MCP server

## v0.1.129
### Added
- `merge` parameter to `read_data` and `read_data_str` to indicate if we want to merge the RDF data read with the current one or replace it


## v0.1.128
### Added
- text_signature to pyrudof methods

## v0.1.127
### Added
- Added `data` tools to MCP server
- Added option to show node info without colors

## v0.1.126

### Fixed
- Issue with dctap2uml which was using all the triples in all endpoints for the generation!

## v0.1.125
### Added
- `list_use_endpoints` and `list_endpoints` to rudof and pyrudof

### Changed
- The behavior of `reset_all()` only clears the use_endpoints but keeps the list of available endpoints

## v0.1.124
### Added
- default_config.toml is now read at compile time and contains a default config file with some endpoints like dbpedia, wikidata, uniprot, etc.
- added list_endpoints to PyRudof
- Improved visualization of node information using termtree, now it shows the incoming/outgoing using arrow glyphs
- node_info in Python bindings

### Changed
- Moved node_formatter from rudof_cli to rudof_lib so it can be reused by Python bindings, CLI and MCP

### Fixed
- A problem with node information that was showing extra `<` and `>` characters

### Removed
- folder shex_compact_winnow which was no longer used

## v0.1.121
### Added
- `mcp` command now starts rudof as an MCP server. First contribution by @samuel-bustamante
- Solved issue #349 to replave dependency on ptree for termtree
- Started to have a list of known SPARQL endpoints identified by key like `wikidata`, `dbpedia`, etc.
- show_table() to SPARQL query results (QueryResults) in rudof and pyrudof
- Solves issue #351
- Solves issue #349 thanks to patch made by @jonassmedegaard

### Fixed

### Changed
- The import in ShEx schemas supports `IriOrStr` in order to handle relative IRIs. Now the base IRI that is passed is the current folder.
- Changed initialization of rudof to return a potential error in case there is some problem with RudofConfig

### Removed

## v0.1.120
### Added
- Visualization of results of ShEx validation as nice tables leveraging on [tabled](https://docs.rs/tabled/latest/tabled/) crate
- Option to sort the results of ShEx validation by either node, shape, status
- Pretty prints the results of SPARQL queries as tables
- Added option to show results of ShEx validation with details
- Started to pretty print results: reasons already have pretty print qualifying IRIs, pending validation errors

### Fixed
### Changed
### Removed

## v0.1.119
### Added
- Support to visualize a single shape in a ShEx schema with the option `--label` in CLI (issue 341)
- Restored support for Excel files in DCTAP
- Added support to export validation results as lists of tuples (node, shape, validatioStatus) in Python

### Fixed
### Changed
### Removed

## 0.1.118
### Added
- Support for `generate` integrating the code from `data_generator` (Diego)

### Changed
### Removed


## 0.1.117
### Added
- Support for imports in ShEx

### Changed
- We moved the contents of ShEx Compact crate to ShEX AST because when we resolve imports, we need to parse the imported ShEx schema so there it is necessary to know which formats we are importing

### Removed
- crates `shex_compact` and `shapemap` will no longer be published as independent crates and will instead be part of `shex_ast`.


## 0.1.116
### Added
- ShEx validation now supports (min/max)(In/Ex)clusive, stems and stem ranges for IRIs, literals and languages
- Added support for Start in validation
- ShEx testsuite status: Passed: 1053, Failed: 91, Skipped: 22, Not implemented: 0

### Fixed
- Several errors in ShEx validation
- Issue #338 about empty shapes
- Issue #309 about IRI ranges

## 0.1.115
### Added
- `run_query_endpoint` in pyrudof and rudof

### Fixed
- `read_shacl` in pyrudof which was trying to read from string instead of from a file

## 0.1.113
### Added
- More support for SPARQL queries in rudof and pyrudof
- We had several issues and published several minor releases

## 0.1.108
### Fixed
- We found a problem with SPARQL queries that were returning no results
- Repaired problem with xsd:dateTime



## 0.1.107
### Added
- Added the possibility to read different elements from file paths or URLs. We removed the suffix `_path` for all the methods that read from those inputs. We keep only the `_str` suffix for methods that read from a string. For example, `read_data(input, ...)` allows the input to be a URL, a file path or stdin (which can be useful in linux pipes), while `read_data_str(input, ...)` requires the input to be a string.
- Added `read_shapemap(input,...)` which was required by issue #329.

### Fixed
- We found a issue when validating datatype literals because we were not handling
### Changed


### Removed

## 0.1.105
### Added
### Fixed
### Changed
- Updated dependency on oxigraph to 0.5.0 solving issue #335

### Removed

## 0.1.104
### Added
- Added more information to MIE files

### Fixed
- Tried to improve the error message when parsing ShEx files that have an undeclared alias according to issue #331

### Changed

### Removed


## 0.1.103
### Added

### Fixed
- GraphCollection in service description contains a collection of named graphs (before was a collection of graph descriptions)
- The parser now parses also the available graphs

### Changed


### Removed

## 0.1.102
### Added
- Comparison between schemas
- Added documentation about comparison between schemas
- Published Windows amd-64 Python wheel
- Added parsed title in SPARQL service description from property dcterms:title

### Fixed
- Cleaned and Clippied the code that we did in a hurry during Biohackathon

### Changed
- The behavour of `base` which was assumed to be None by default and now can be passed as a command line option.

### Removed


## 0.1.93
### Added
### Fixed
- Repaired a problem with the parser with case insensitive keywords like IRI, BnodE, etc.
- Repaired python bindings

### Changed
### Removed

## 0.1.92
### Added

This release has been created during the [Biohackathon 2025](https://2025.biohackathon.org/) where we have been adding several features by quick demands of the attendees. It is possible that not all the features have been thoroughly tested, but those features are demanded by users and we plan to improve them in future releases.
- Initial support for comparing 2 schemas
- Initial support to read rdf_config files

### Fixed
### Changed
### Removed

## 0.1.90
### Added
- Added serialize_current_shex to pyrudof
- Added read_service_description, serialize_service_description to rudof_lib and pyrudof
- Added data2plantuml_file to pyrudof

### Fixed
### Changed
- from_reader in ServiceDescription now accepts a `io::Read` instead of a `BufRead`.
- Refactored run_service to be based on rudof lib

### Removed


## 0.1.89
### Added

- Added support for SHACL Paths, sh:uniqueLang, flags in sh:pattern, sh:qualifiedValueShape
- Added support for severities and printing validation results with colors

### Fixed
- Error in sh:hasValue when the value was a literal
- sh:lessThan and sh:lessThanOrEquals now return the expected errors

### Changed
### Removed


## 0.1.88
### Added

Support for lessThan, lessThanOrEquals, equals and disjoint
### Fixed
### Changed
### Removed

## 0.1.87
### Added
- Support for SHACL validation of: deactivated, closed, ignoredProperties

### Fixed

- Error with datatype test from SHACL validation

### Changed
- Command line interface for `shacl` option now suppports information from RDF data or Schema to have an interface similar to `shacl-validate`

## v0.1.86
### Added
### Fixed
### Changed
- Updated dependency on py03 to use 0.25.1, it required adding Sync to Cond trait
### Removed


## v0.1.84
### Added
- Support for JSON-LD oslving issue #295
### Fixed
### Changed
### Removed

## v0.1.83 - 2025-08-21

### Added

Method `data2plantuml` to rudof Python bindings

### Fixed

Issue #312 changing the behaviour of RDF/XML and NQuads parsers which were generating empty RDF graphs for incorrect RDF files instead of raising an error. Those empty RDf graphs didn't raise violations when they were validated.

### Changed
### Removed

## [v0.1.82] - 2025-08-20
### Added
- Updated oxigraph dependencies to 0.5.0-beta.2 which supports RDF 1.2
- Remove the feature `rdf-star` replacing `rdf-star` by `rdf-12`.
- Some examples with RDF 1.2 features
- Visualization of RDF graphs leveraging on PlantUML

### Fixed

### Changed
- Started implementing deactivated
- Added an UMLConverter trait to handle both ShEx2UML and RDF2UML

### Removed

## [v0.1.81] - 2025-07-13

Repaired a bug that was found when obtaining the neighbours of a node in an endpoint.

## [v0.1.80] - 2025-07-11

- Added the possibility to convert between ShEx to ShEx (with different formats) and SHACL to SHACL (with different formats) to the `convert` command in the command line.
- Refactor the SHACL Intermediate representation
- Added support to language ValueSetValue in ShEx, i.e. constraints like `[ @en ]` (issue #304)

## [v0.1.79] - 2025-06-30

- Internal refactor in SHACL validator to use SHACL Internal Representation with an independent representation from the `Rdf` trait which allows it to be applied to different implementations of the `Rdf` trait.

## [v0.1.77] - 2025-06-24

- Added support for (min/max)(in/ex)clusive
- Repaired bug in minLength
- Solved typo in documentation

## [v0.1.72] - 2025-06-14

- Removed dependency on lazy_static!
- Added `shacl_rdf` and `shacl_ir` crates
- Created a folder `oxrdf_impl` that contains the implementations for the traits defined at the top level using the `oxrdf` library
- Renamed internal srdf traits and files. Some conventions, we will prepend `S` to the concrete structs or enums defined by SRDF, so instead of `Literal` we use `SLiteral`, keeping `Literal` for the trait name.
   - file `srdf_basic.rs` => `rdf.rs`
   - trait `Query` => `NeighsRDF`
   - trait `Sparql` => `QueryRDF`
   - trait `SRDFBuilder` => `BuildRDF`
   - struct `Literal` => `SLiteral`
   - struct `Triple` => `STriple`



## [v0.1.71] - 2025-05-28

- Disabled Xlsx support given the problem with Calamine in order to publish Python version of rudof

## [v0.1.70] - 2025-05-26

- Added implementation of ShEx validator that follows the [paper](https://labra.weso.es/publication/2017_semantics-validation-shapes-schemas/)
- There is [a problem](https://github.com/rudof-project/rudof/issues/291) with calamine's dependency from DCTAP which doesn't allow us to publish in crates.io. We are waiting for calamine to publish an official release because it seems the patch only works to build the system, but prevents us to publish to crates.

## [v0.1.65] - 2025-05-14

- Set reqwest dependency on rustls to disable openssl which gives several problems

## [v0.1.64] - 2025-05-14

- Added check on recursion with negative cycles in ShEx
- Added different result formats in ShEx like JSON

## [v0.1.63] - Skipped

## [v0.1.62] - 2025-03-29

- Changed dependency from [serde_yaml_ng](https://github.com/acatton/serde-yaml-ng) to [toml](https://docs.rs/toml/latest/toml/)
- Removed dependency in rbe_tests from serde_yaml_ng to use plain JSON for the test_suite

## [v0.1.60] - 2025-03-11

- Changed dependency from [serde_yml](https://doc.serdeyml.com/serde_yml/index.html) to [serde_yaml_ng](https://github.com/acatton/serde-yaml-ng) according to #278
- Changed Iri trait to add Ord constraint so IRIs can be ordered solving issue #276

## [v0.1.59] - 2025-01-01

- Fixes bug in feature added to solve issue #227 for local files which are relative that it didn't generate an absolute IRI. Now it does.
- Added option to SHACL2ShEx converter to optionally add `rdf:type` declaration for each `sh:targetClass` declaration. Previously, this behaviour was not optional and now it can be disabled.
- Fixes option to generate `rdf:type` for `sh:targetClass` declarations when there are more than one (previously it generated one rdf:type for each target class, and not it generates a value set).

## [v0.1.58] - 2024-12-31

- Solves issue #227 to automatically generate a base URL from the local file name or URL.

## [v0.1.57] - 2024-11-14

- Simple release to bump a new version that solve a issue with pyrudof in Google Colab

## [v0.1.56] - 2024-11-14

- Added `variables()` and `find` to QuerySolution class in pyrudof

## [v0.1.55] - 2024-11-14

- Added methods to show query solutions in rudof and pyrudof

## [v0.1.54] - 2024-11-13

- Added query to rudof and pyrudof

## [v0.1.53] - 2024-11-13

- Added serialization of RDF data from rudof and pyrudof

## [v0.1.52] - 2024-11-1

- Added `endpoints` to `RdfDataConfig` to contain a list of built-in endpoints
- Added prefixmap as a parameter to create `SRDFPARQL` endpoints
- Solved problem when asking information about a node in wikidata endpoint
- Added `config()` method to obtain `rudof` config
- Improved `add_endpoint()` in pyrudof to search for the list of built-in endpoints in RDFDataConfig

## [v0.1.51] - 2024-10-31

- Added `read_data_path` to `pyrudof`

## [v0.1.50] - 2024-10-31

- Fix: We repaired some export issues on UmlGenerationMode and the `__repr__` methods which were not properly generated.

## [v0.1.49] - 2024-10-30

- Implemented Display for ShapeMap, ShEx-schema and SHACL-schema
- Added `__repr__` to ShapeMap, ShExSchema and SHACLSchema
- Added `update_config` to rudof and pyrudof

## [v0.1.48] - 2024-10-29

- Minor release to force re-publication

## [v0.1.47] - 2024-10-29

- Changed the way that we represent enums in Python to use proper enums with default values
- Added `read_shacl_str` and `read_shacl_path` to pyrudof

## [v0.1.46] - 2024-10-29

- Added default values to `pyrudof` to allow a more flexible API
- minor release to include RDFFormat and ReaderMode in export list of `pyrudof`

## [v0.1.45] - 2024-10-29

- Changed the order of parameters in `read_shex_str`, `read_data_str` in `pyrudof`
- `RDFFormat` added in `pyrudof`
- `ReaderMode` added in `pyrudof`
- `reset_all` added in `pyrudof`

## [v0.1.44] - 2024-10-29

- `add_endpoint` added in `rudof_lib` and `pyrudof_lib`
- `reset_shacl` added in `rudof_lib` and `pyrudof_lib`

## [v0.1.43] - 2024-10-28

Minor release to add DCTAP for pyrudof

## [0.1.40] - 2024-10-28

- Added more features to the rudof_lib like the serialization of ShEx, SHACL and Shapemaps which is also mirrored in the Python bindings.
- Added shex2uml python bindings

## [0.1.37] - 2024-10-28

- Added more features to the rudof_lib like the serialization of ShEx, SHACL and Shapemaps which is also mirrored in the Python bindings

## [0.1.36] - 2024-10-27

- Python bindings based on rudof_lib to validate ShEx and SHACL

## [0.1.35] - 2024-10-25

- More refactoring on main to depend on rudof_lib for SHACL, issue #201
- Implemented Display for SHACL Validation report which shows the results with colors

## [0.1.34] - 2024-10-23

-Some refactoring on main to depend on rudof_lib and check if it works

## [0.1.33] - 2024-10-22

- Internal release to just change the README in rudof_lib

## [0.1.32] - 2024-10-22

- Created crate [`rudof_lib`](https://crates.io/crates/rudof_lib) which will act as the main library entry point for `rudof`. In the future, this crate could be called `rudof`.
- Refactor of main to invoke `rudof_lib`
- Added [`ResultShapeMap`](https://docs.rs/shapemap/latest/shapemap/result_shape_map/struct.ResultShapeMap.html) as the result of ShEx validation. One improvement is that now the results can appear with colors.

## [0.1.31] - 2024-10-20

- Added more information to docs
- Implemented more features of Service description
- Added Accept headers to `InputSpec` so it provides basic content negotiation
- Added ShExConfig to improve configuration of options that involve ShEx
- Added literals to shape maps
- Improved aesthetics of docs #170

## [0.1.30] - 2024-08-10

- Added support for imports #159
- Solved typo xslsx -> xlsx #176

## [0.1.29] - 2024-09-30

- Added option to use xlsx directly in tap2shex conversion
- Updated version of serde_yml to 0.0.12

## [0.1.28] - 2024-09-30

- First version that handles directly Excel spreadsheets in DCTAP. Issue #82
- Repaired small bug in DCTAP where headers with leading or trailing whitespaces where not properly parsed
- Unified dependencies on serde-yml #160
- Expose API to retrieve SHACL validation reports #164
- Fixed github action that was giving errors when publishing Python bindings #151

## [0.1.27] - 2024-09-25

- Added support for picklist values in DCTAP
- Added support for picklist values in DCTAP2ShEx
- Added support for simple value set values in ShEx to UML

## [0.1.26] - 2024-09-20

- Added support for SPARQL query options. New command called: `query`
- Added support for handling SPARQL service descriptions. New command called: `service`
- Changed the TAPConfig parameter of command `dctap` so it can use the same config file as option `tap2shex`

## [0.1.25] - 2024-09-11

- Small change removing an empty config file to solve issue #155

## [0.1.24] - 2024-09-10

- Added more configuration parameters for RDF data and Shacl data which allow, for example to define a default base IRI which can be used to resolve relative IRIs solving issue #149

## [0.1.23] - 2024-09-09

- Added option for partial views of UML class diagrams which can be useful when visualizing large ShEx schemas
- Improved the templates so they show metadata about the generation and a navigation bar
- Repaired a bug in the behaviour of force-overwrite which was appending to the file instead of overwriting its contents

## [0.1.22] - 2024-09-07

- Added the possibility to embed the SVG diagram in the HTML pages that are generated

## [0.1.21] - 2024-09-05

- Small release with a small improvement in the way we handle empty rows in DCTAP

## [0.1.20] - 2024-09-01

- Added option to get schemas from files, URIs or stdin (-) which was also implemented to data, solving issue #135
- Small release after moving the project to a standalone rudof-project organization

## [0.1.19] - 2024-08-30

- Added option to generate simple information about ShEx schemas
- Repaired bug in strict/lax reader mode that was not being taken into account

## [0.1.18] - 2024-08-28

- Added support for nquads and RDF/XML as input data formats
- Added more flexibility for NQuads parser to continue parsing in case of errors
- Added more flexibility of RDF parser to parse RDF lists in case there are more than one rdf:first predicate.

## [0.1.17] - 2024-08-28

- Repaired bug in DCTAP when a row has an empty shape_id and it was creating an empty shape instead of assuming the previous one
- Added support for first version of SHACL to ShEx converter

## [0.1.16] - 2024-08-22

- This release only changes the name of the python bindings from rudof to pyrudof and adds a first submodule convert for checking if it works

## [0.1.15] - 2024-08-19

- Solves issue #115 adding annotations to the ShEx compact printer
- Takes into account annotations to generate labels in HTML and UML conversion from ShEx

## [0.1.14] - 2024-08-14

- Added support for using URLs in command line. The system attempts to dereference the URI and parses its content.
- Added support for parsing placeholders in DCTAP generating new properties for each one
- Added support for extends in DCTAP

## [0.1.13] - 2024-08-13

- `data` option now serializes the RDF data to one of the existing RDF data formats (previous version were generating an internal representation of the graph).
- Added support for using `-` as a marker for stdin so `rudof` can be used in a Linux pipe

## [0.1.12] - 2024-08-13

- Changed the one line description of the commands according to issue #77
- First version which allows several RDF data files in the command line #100
- Repaired small bug in the validate option because two options had the same long name: mode

## [0.1.11] - 2024-08-12

- Repaired error #91 adding a force-overwrite option to the command line
- Changed command line name from `rdfsx` to `rudof`

## [0.1.6] - 2024-08-09

- Added more features to SHACL validation: #94
- Added more control about syntax highlighting on terminal:
  - Avoiding to include colors when the output goes to a file in ShEx generation options
- Added config parameter to some of the options in the Command line tool so the user can configure the behaviour: validate, convert, dctap, node

## [0.1.5] - 2024-07-30

- Added options in command line to pass config files in YAML
- Repaired bug in DCTAP resolution of IRIs

## [0.1.4] - 2024-07-28

- Added 2 separate options for shacl-validate and shex-validate, keeping the generic validate option
- Repaired bug on UML visualization that didn't show link names
- Added direct SVG/JPG generation from DCTAP files

## [0.1.3] - 2024-07-27

- Generation of HTML views from ShEx based on Minininja templates which allow better customization
- Direct conversion from DCTAP to UML and HTML views
- Generation of UML visualizations in SVG and PNG
- Basic support for SHACL validation and added shacl-validation crate

## [0.1.2] - 2024-07-17

- Added descriptions to subcommands in command line
- Added more options in DCTAP: property and shape labels, and value constraints
- Added direct conversion from DCTAP to HTML and UML
- More options for HTML views

## [0.1.1] - 2024-07-12

- Added basic support for generating HTML views from ShEx schemas, #60

## [0.1.0] - 2024-07-05

- Added fields: mandatory, repeatable, valueDatatype and valueShape to DCTAP
- Repaired spelling errors in README issue #73

## [0.0.15] - 2024-07-04

- First version with support for conversion from ShEx schemas to UML

## [0.0.14] - 2024-07-02

- First version with initial support for DCTap to ShEx converter, issue #54
- Refactor on shapes converter to accomodate more conversions each of them in its own folder
- First version which publishes also Python bindings

## [0.0.13] - 2024-06-22

- First version with initial support for ShEx to SPARQL converter, issue #67

## [0.0.12] - 2024-06-17

- Changed CLI name from `sx` to `rdfsx`
- First attempt to added basic support for DCTap
- Code cleaned with Rustfmt and Clippy by [MarcAntoine-Arnaud](https://github.com/MarcAntoine-Arnaud).

## [0.0.11] - 2024-06-08

- This version in mainly a maintainance version updating some dependencies
- Started project DCTAP to handle DCTAP files
- Updated some dependency versions
  - oxrdf = "0.2.0-alpha.2"
  - regex = "1.10.4"

## [0.0.10] - 2024-01-29

- [issue 32](https://github.com/rudof-project/rudof/issues/32) ShEx parser works as an iterator per statement allowing to show debug information by statement. Debug information can be controlled by the environment variablt RUST_LOG. A value of "debug" for that variable will print more information.
- Updated dependency versions
    oxrdf = "0.2.0-alpha.2"
    oxttl = "0.1.0-alpha.2"
    oxrdfio = "0.1.0-alpha.2"

## [0.0.9] - 2024-01-19

- Removed `shex_pest`, `shex_antlr` and `validation_oxgraph` folders because their code is no longer used.
- Added time option to `sx_cli`
- Repaired bug in `shex_compact` that failed with node constraints followed by cardinality without space
- More support to read SHACL as RDF
- Merged [srdf_graph](https://crates.io/crates/srdf_graph) and [srdf_sparql](https://crates.io/crates/srdf_sparql) crates into [srdf](https://crates.io/crates/srdf), the former crates will no longer be maintained as their code is integrated in `srdf`.
- Added option `--output` to CLI so the users can choose if the output goes to terminal or to a file
- Changed dependency from [rio_api](https://crates.io/crates/rio) and [rio_turtle](https://crates.io/crates/rio_turtle) to [oxttl](https://crates.io/crates/oxttl) and [oxrdfio](https://crates.io/crates/oxrdfio) which seem to be more actively maintained now.

## [0.0.7] - 2024-01-07

In this release we added support for SHACL by defining the [`shacl_ast`](https://crates.io/crates/shacl_ast) crate.

Other changes:

- Renamed the project from shex_rs to shapes_rs to indicate that the project intends to support both ShEx and SHACL.
- Merged the [srdf_graph](https://crates.io/crates/srdf_graph) and [srdf_sparql](https://crates.io/crates/srdf_sparql) crates into [srdf](https://crates.io/crates/srdf).
- Added more combinators and documentation examples to rdf_parser in order to document the RDF parser combinators approach. See, for example, the doc for the [map method](https://docs.rs/srdf/latest/srdf/srdf_parser/trait.RDFNodeParse.html#method.map).
