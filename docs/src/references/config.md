# Config reference

This page is an exhaustive, field-by-field reference for `rudof.toml` (the file read by
[`--config`/`-c` and the config discovery process](../general/configuration.md)). For an
introduction to *how* configuration is loaded and merged, see [Configuration](../general/configuration.md);
this page only documents *what* can be set.

> **Note on how this page is generated:** this reference is hand-written from the Rust
> source of `RudofConfig` and the per-subsystem config structs it aggregates (see the
> "Source" line under each section below). `rudof` doesn't currently have tooling that
> extracts doc comments from these structs automatically (no `schemars`/`documented`-style
> derive is used for this purpose — `schemars` is used elsewhere, for the MCP server's tool
> schemas), so if a field is added, renamed, or its default changes, this page needs a
> matching manual update rather than being regenerated.

`rudof config` prints the *effective* configuration (defaults merged with whatever files
were discovered) as plain TOML, which is useful as a starting template — but the TOML crate
doesn't carry Rust doc comments into that output, so it isn't self-documenting. This page is
the missing piece.

## Top-level keys

Source: [`rudof_lib/src/config/rudof.rs`](https://github.com/rudof-project/rudof/blob/master/rudof_lib/src/config/rudof.rs), [`rudof_lib/src/config/common.rs`](https://github.com/rudof-project/rudof/blob/master/rudof_lib/src/config/common.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `version` | string (semver) | current `rudof` version | Stamped automatically; a missing `version` in a loaded file just logs a warning, and a major-version mismatch is a hard error. You normally don't set this by hand. |
| `base_iri` | IRI string | unset | Default base IRI used to resolve relative IRIs, shared across sections that don't set their own (see [Base IRI propagation](#base-iri-propagation) below). |
| `auto_base` | boolean | `false` | If `true` and no `base_iri` is set anywhere, `http://base` is used as the base automatically instead of leaving relative IRIs unresolved. |

Every other key is a `[table]` for one subsystem, listed below.

### Base IRI propagation

After loading, `RudofConfig` resolves cross-section values (`RudofConfig::resolve()`):

- Any section with its own `base_iri` unset (`[rdf]`, `[shex]`, `[service]`, `[tap2shex]`)
  inherits the top-level `base_iri` (or `http://base` if `auto_base = true`).
- A handful of fields exist in the Rust structs only because a section embeds another
  section's config (e.g. `[shex_validator]` embeds a copy of `[rdf]` and `[shex]`,
  `[shacl]` embeds a copy of `[rdf]`, `[tap2shex]` embeds a copy of `[tap]`,
  `[shex2uml]`/`[shex2html]`/`[shex2sparql]` embed a copy of `[shex]`). These embedded
  copies are **always overwritten** with the real top-level section on resolve, so they
  aren't independently configurable — setting them directly in TOML has no effect. They're
  omitted from the tables below; only the tables actually worth setting are listed.

> There is no `[pgschema]` table — PGSchema loading is controlled entirely through CLI
> flags / the `rudof_lib` API, not through `rudof.toml`.

## `[rdf]` — RDF data

Source: [`rudof_rdf/src/rdf_core/rdf_data_config.rs`](https://github.com/rudof-project/rudof/blob/master/rudof_rdf/src/rdf_core/rdf_data_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `base_iri` | IRI string | unset | Base IRI to resolve relative IRIs in RDF data. If unset (and `auto_base` is off), relative IRIs are an error. |
| `local_base` | boolean | `true` | Automatically use the local file/URI being read as the base IRI. |
| `endpoints` | table of tables | `wikidata`, `dbpedia`, `uniprot` pre-registered | Named SPARQL endpoints — see [`[rdf.endpoints.<name>]`](#rdfendpointsname) below. |
| `visualization` | table | see [`[rdf.visualization]`](#rdfvisualization) | Styling for RDF graph visualizations (`svg`/`png`/`plantuml` output). |
| `qlever` | table, optional | unset | QLever Docker backend settings — see [`[rdf.qlever]`](#rdfqlever) below. Only compiled in with the `qlever` cargo feature, and not available on wasm. |

### `[rdf.endpoints.<name>]`

Each key under `endpoints` names one registered endpoint (used by `-e NAME`/`--endpoint NAME`
and the shell's `endpoint NAME` command). `rudof` pre-registers three by default —
`wikidata`, `dbpedia`, `uniprot` — which you can override or add to.

| Key | Type | Default | Description |
|---|---|---|---|
| `query_url` | IRI string | *(required)* | URL of the SPARQL query endpoint. |
| `update_url` | IRI string, optional | unset | URL for SPARQL update operations, if the endpoint supports them. |
| `prefixmap` | table (`alias = "iri"`) | empty | Prefixes to abbreviate IRIs in this endpoint's query results. |

```toml
[rdf.endpoints.wikidata]
query_url = "https://query.wikidata.org/sparql"

[rdf.endpoints.wikidata.prefixmap]
wd = "http://www.wikidata.org/entity/"
wdt = "http://www.wikidata.org/prop/direct/"
```

### `[rdf.visualization]`

Controls the appearance of `--result-format svg`/`png`/`plantuml` graph output.

| Key | Type | Default | Description |
|---|---|---|---|
| `uri_style` | [node style](#node-style-fields) | blue outline, white fill, rounded | Style for IRI nodes. |
| `bnode_style` | [node style](#node-style-fields) | blue outline, gray fill, rounded | Style for blank nodes. |
| `literal_style` | [node style](#node-style-fields) | black outline, cyan fill, square | Style for literal nodes. |
| `reifier_style` | [node style](#node-style-fields) | black outline, yellow fill, square | Style for RDF 1.2 reifier nodes. |
| `asserted_style` | [node style](#node-style-fields) | black outline, white fill, square | Style for asserted triple nodes. |
| `non_asserted_style` | [node style](#node-style-fields) | blue outline, white fill, square | Style for non-asserted (quoted) triple nodes. |
| `subject_arrow_style` | [arrow style](#arrow-style-fields) | blue, dashed | Style for subject-role arrows. |
| `predicate_arrow_style` | [arrow style](#arrow-style-fields) | red, dashed | Style for predicate-role arrows. |
| `object_arrow_style` | [arrow style](#arrow-style-fields) | green, dashed | Style for object-role arrows. |
| `subject_label` | string | `"subject"` | Label used for a triple term's subject role. |
| `predicate_label` | string | `"predicate"` | Label used for a triple term's predicate role. |
| `object_label` | string | `"object"` | Label used for a triple term's object role. |
| `reifies_label` | string | `"reifies"` | Label used for the reification relationship. |
| `subject_text` | string | `"subj"` | Short text on the subject arrow. |
| `predicate_text` | string | `"pred"` | Short text on the predicate arrow. |
| `object_text` | string | `"obj"` | Short text on the object arrow. |
| `unasserted_triple_shape` | `"Cloud"` \| `"Rectangle"` | `"Cloud"` | Node shape for non-asserted triples. |
| `asserted_triple_shape` | `"Cloud"` \| `"Rectangle"` | `"Rectangle"` | Node shape for asserted triples. |

*Note:* `unasserted_triple_shape`/`asserted_triple_shape` take the values `"Cloud"` or
`"Rectangle"` capitalized as shown — unlike most other enum-valued settings on this page,
this one isn't lowercased/snake_case.

#### Node-style fields

Each `*_style` above (except the arrow ones) is a table with these keys:

| Key | Type | Description |
|---|---|---|
| `line_color` | color name | Outline color. |
| `line_thickness` | integer | Outline thickness. |
| `background_color` | color name | Fill color. |
| `round_corner` | integer | Corner radius (`0` = square corners). |

Valid color names (snake_case): `white`, `black`, `cyan`, `gray`, `red`, `green`, `blue`,
`yellow`, `light_blue`, `light_green`, `light_coral`.

#### Arrow-style fields

| Key | Type | Description |
|---|---|---|
| `line_color` | color name | Arrow line color (see the color list above). |
| `line_thickness` | `"bold"` \| `"normal"` \| `"dashed"` \| `"dotted"` | Arrow line style. |
| `text_color` | color name | Color of the arrow's label text. |

```toml
[rdf.visualization.uri_style]
line_color = "blue"
line_thickness = 1
background_color = "white"
round_corner = 25
```

### `[rdf.qlever]`

Only relevant when running RDF data through a [QLever](https://github.com/ad-freiburg/qlever)
Docker container backend (`--backend qlever`). Every field is optional; `rudof` only sets a
`--flag` on the QLever process when you set the corresponding key.

| Key | Type | Default | Description |
|---|---|---|---|
| `image_name` | string | `"adfreiburg/qlever"` | Docker image to run. |
| `image_tag` | string | `"commit-a307781"` | Docker image tag. |
| `index_dir` | path, optional | `<cache dir>/rudof/qlever/<hash>` | Host directory for the QLever index. |
| `index_name` | string | `"default"` | Index base name (QLever's `-i`). |
| `auto_delete_if_created` | boolean | `false` | Delete the index directory when the container is dropped, if this run created it. |
| `stxxl_memory` | string, optional | unset | STXXL memory for index building. |
| `parser_buffer_size` | string, optional | unset | `--parser-buffer-size`. |
| `parser_parallel` | boolean, optional | unset (QLever default: on) | `--parse-parallel`. |
| `container_memory` | string, optional | unset | Docker container `--memory` cap. |
| `container_memory_swap` | string, optional | unset | Docker container `--memory-swap` cap. |
| `host_port` | integer, optional | unset (ephemeral port) | Pin the host-side port instead of picking one automatically. |
| `container_port` | integer | `7001` | Container-side port. |
| `access_token` | string, optional | unset | QLever `-a` admin access token. Never written back out by `rudof config` when unset. |
| `num_simultaneous_queries` | integer, optional | unset | QLever `-j`. |
| `memory_max_size` | string, optional | `"5G"` | Server memory limit (QLever `-m`). |
| `cache_max_size` | string, optional | `"2G"` | Cache size (QLever `-c`). |
| `cache_max_size_single_entry` | string, optional | `"1G"` | Max cacheable single-entry size (QLever `-e`). |
| `lazy_result_max_cache_size` | string, optional | unset | QLever `-E`. |
| `cache_max_num_entries` | integer, optional | unset | QLever `-k`. |
| `no_patterns` | boolean | `false` | QLever `-P`. |
| `no_pattern_trick` | boolean | `false` | QLever `-T`. |
| `text` | boolean | `false` | QLever `-t`. |
| `only_pso_and_pos_permutations` | boolean | `false` | QLever `-o`. |
| `default_query_timeout` | string, optional | unset | QLever `-s`. |
| `service_max_value_rows` | integer, optional | unset | QLever `-S`. |
| `throw_on_unbound_variables` | boolean | `false` | QLever `--throw-on-unbound-variables`. |
| `run_as_host_user` | boolean | `true` | Run the container as the host UID/GID instead of root. |
| `container_label` | string, optional | unset | Extra Docker label on the spawned container. |
| `server_readiness_timeout_secs` | integer | `60` | How long to wait for the QLever server to become ready. |

## `[shex]` — ShEx schema handling

Source: [`shex_validation/src/shex_config.rs`](https://github.com/rudof-project/rudof/blob/master/shex_validation/src/shex_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `base_iri` | IRI string | unset | Default base IRI for resolving relative IRIs in a ShEx schema; falls back to the top-level `base_iri` if unset. |
| `shex_format` | `"shexc"` \| `"shexj"` \| an RDF format name | `"shexc"` | Format of ShEx schemas read from disk (`shexc`/`shex` for ShExC, `shexj`/`json` for ShExJ, or any RDF serialization name — e.g. `turtle` — when the schema is embedded as RDF). |
| `check_well_formed` | boolean | `true` | Check that the schema is well-formed after parsing. |
| `show_extends` | boolean | `true` | Include EXTENDS relationships when showing a schema (`shex` command). |
| `show_imports` | boolean | `true` | Include import information when showing a schema. |
| `show_shapes` | boolean | `true` | Include the shape list when showing a schema. |
| `show_dependencies` | boolean | `false` | Include shape dependency information when showing a schema. |
| `show_ir` | boolean | `false` | Show the compiled Schema Internal Representation instead of the source syntax. |

## `[shex_validator]` — ShEx validation

Source: [`shex_validation/src/validator_config.rs`](https://github.com/rudof-project/rudof/blob/master/shex_validation/src/validator_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `max_steps` | integer, optional | unset (unbounded) | Maximum number of validation steps before giving up. |
| `check_negation` | boolean | `true` | Check the ShEx negation requirement during validation. |
| `width` | integer | `80` | Line width used when pretty-printing validation output. |
| `shapemap` | table | see below | ShapeMap display settings. |

External-shape resolvers (used to resolve `EXTERNAL` shape declarations) are configured
programmatically (`add_external_resolver`) or via CLI flags, not through `rudof.toml`.

### `[shex_validator.shapemap]`

| Key | Type | Default | Description |
|---|---|---|---|
| `nodes_pm` | table (`alias = "iri"`) | empty | Prefix map used to abbreviate node IRIs in ShapeMap output. |
| `shapes_pm` | table (`alias = "iri"`) | empty | Prefix map used to abbreviate shape-label IRIs in ShapeMap output. |
| `ok_text` | string | `"OK"` | Text shown for a passing result. |
| `fail_text` | string | `"FAIL"` | Text shown for a failing result. |

## `[shacl]` — SHACL validation

Source: [`shacl/src/validator/config.rs`](https://github.com/rudof-project/rudof/blob/master/shacl/src/validator/config.rs)

Not compiled in on the `wasm` target.

This section currently has no independently-settable keys of its own — its only field is
an embedded copy of `[rdf]`, which is always overwritten by the top-level `[rdf]` on
resolve. It's included here for completeness / forward compatibility (e.g. `rudof config`
will still show a `[shacl]` table).

## `[shex2uml]` — ShEx → UML/PlantUML conversion

Source: [`shapes_converter/src/shex_to_uml/shex2uml_config.rs`](https://github.com/rudof-project/rudof/blob/master/shapes_converter/src/shex_to_uml/shex2uml_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `plantuml_path` | path | `$PLANTUML` env var, else `"plantuml.jar"` | Path to the PlantUML jar used to render diagrams. |
| `annotation_label` | list of IRIs | `[rdfs:label]` | IRIs used as annotation labels in the diagram. |
| `replace_iri` | boolean | `false` | Replace IRIs with their labels (from `annotation_label`) in the diagram. |
| `shadowing` | boolean | `true` | Use shadowing for shapes in the diagram. |
| `line_type` | `"orthogonal"` \| `"polyline"` \| `"default"` | `"default"` | Connector line style. |
| `direction` | `"left_to_right"` \| `"top_to_bottom"` | `"top_to_bottom"` | Diagram layout direction. |

## `[shex2html]` — ShEx → HTML conversion

Source: [`shapes_converter/src/shex_to_html/shex2html_config.rs`](https://github.com/rudof-project/rudof/blob/master/shapes_converter/src/shex_to_html/shex2html_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `title` | string | `"ShEx schema"` | Title used in the generated HTML pages. |
| `landing_page` | string | `"index.html"` | Filename of the generated landing page. |
| `shape_template` | string | `"shape.html"` | Filename of the per-shape page template. |
| `template_folder` | string, optional | unset (built-in templates) | Folder with custom Tera/HTML templates, if overriding the built-in ones. |
| `css_file` | string | `"shex2html.css"` | Generated stylesheet filename. |
| `target_folder` | path | `"."` | Output directory for the generated site. |
| `property_color` | string | `"blue"` | Color used for property names in the generated pages. |
| `annotation_label` | list of IRIs | `[rdfs:label]` | Same as `[shex2uml].annotation_label`, used for the embedded diagrams. |
| `replace_iri_by_label` | boolean | `true` | Replace IRIs with their labels in the generated pages. |
| `embed_svg_schema` | boolean | `true` | Embed an SVG diagram of the whole schema on the landing page. |
| `embed_svg_shape` | boolean | `true` | Embed an SVG diagram on each shape's own page. |

## `[shacl2shex]` — SHACL → ShEx conversion

Source: [`shapes_converter/src/shacl_to_shex/shacl2shex_config.rs`](https://github.com/rudof-project/rudof/blob/master/shapes_converter/src/shacl_to_shex/shacl2shex_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `starting_shapes_mode` | `"non-bnodes"` | `"non-bnodes"` | How to pick the ShEx schema's starting shapes; currently the only mode is "shapes which aren't blank nodes". |
| `embed_bnodes` | boolean | `false` | Embed blank-node shapes inline instead of giving them their own top-level shape. |
| `add_target_class` | boolean | `false` | Add an `rdf:type`/`ex:class` triple constraint for each SHACL `sh:targetClass` declaration. |

> Unlike the other embedded-subsystem fields described in [Base IRI propagation](#base-iri-propagation),
> `[shacl2shex].shacl` (an embedded copy of `[shacl]`) is **not** overwritten by `resolve()`
> — if you need to point `shacl2shex` at different SHACL settings than the top-level
> `[shacl]`, you can set `[shacl2shex.shacl]` directly.

## `[tap]` — DCTAP (CSV) handling

Source: [`dctap/src/tap_config.rs`](https://github.com/rudof-project/rudof/blob/master/dctap/src/tap_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `delimiter` | single character | `,` | Column delimiter in the DCTAP CSV. |
| `quote` | single character | `"` | Quote character for CSV fields. |
| `flexible` | boolean | `true` | Allow rows to have a varying number of fields instead of erroring on mismatch. |
| `picklist_delimiter` | single character | `\|` | Delimiter between values inside a picklist cell. |
| `value_shape_delimiter` | single character | `' '` (space) | Delimiter between values inside a value-shape cell. |
| `property_placeholders` | table, optional | empty | *Experimental.* Per-property-ID value generators — see below. |
| `empty_property_placeholder` | table, optional | unset | *Experimental.* Value generator used for rows with an empty property ID. |

Placeholder resolvers are tables of the form `{ stem = "..." }` (currently the only kind is
`stem`, which generates `"<stem><row number>"`-style values):

```toml
[tap.property_placeholders.x]
stem = "pending"
```

## `[tap2shex]` — DCTAP → ShEx conversion

Source: [`shapes_converter/src/tap_to_shex/tap2shex_config.rs`](https://github.com/rudof-project/rudof/blob/master/shapes_converter/src/tap_to_shex/tap2shex_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `base_iri` | IRI string | unset (inherits the top-level `base_iri`) | Base IRI for the generated ShEx schema. |
| `datatype_base_iri` | IRI string, optional | unset | Base IRI used to resolve bare datatype names from the DCTAP `valueDatatype` column. |
| `prefixmap` | table (`alias = "iri"`) | `dc`, `rdf`, `rdfs`, `sh`, `xsd`, and an empty-alias prefix for `http://example.org/` | Prefixes used in the generated ShEx schema. |

## `[shex2sparql]` — ShEx → SPARQL conversion

Source: [`shapes_converter/src/shex_to_sparql/shex2sparql_config.rs`](https://github.com/rudof-project/rudof/blob/master/shapes_converter/src/shex_to_sparql/shex2sparql_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `this_variable_name` | string | `"this"` | SPARQL variable name used for "the focus node" in generated queries. |

## `[service]` — SPARQL service description

Source: [`sparql_service/src/service_config.rs`](https://github.com/rudof-project/rudof/blob/master/sparql_service/src/service_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `base_iri` | IRI string | unset (inherits the top-level `base_iri`) | Base IRI used when resolving relative IRIs in a SPARQL service description. |

## `[comparator]` — shape comparison

Source: [`shapes_comparator/src/comparator_config.rs`](https://github.com/rudof-project/rudof/blob/master/shapes_comparator/src/comparator_config.rs)

| Key | Type | Default | Description |
|---|---|---|---|
| `ignore_value_constraints` | boolean | `false` | Treat two shapes as equivalent even if their value constraints differ. |
| `prefixes_equivalences` | list of `[iri, iri]` pairs | empty | Pairs of IRIs to treat as equivalent prefixes when comparing shapes, e.g. `[["http://a/", "http://b/"]]`. |

## Full example

Combining several sections above into one file (based on
[`bindings/python/examples/example.toml`](https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/example.toml)):

```toml
base_iri = "http://example.org/"
auto_base = false

[rdf]
base_iri = "http://example.org/"

[rdf.endpoints.wikidata]
query_url = "https://query.wikidata.org/sparql"

[rdf.endpoints.wikidata.prefixmap]
wd = "http://www.wikidata.org/entity/"
wdt = "http://www.wikidata.org/prop/direct/"

[shex]
show_imports = true
show_dependencies = true

[shex_validator]
max_steps = 100
check_negation = true

[tap]
delimiter = ","
flexible = true

[tap2shex]
base_iri = "http://example.org/"

[tap2shex.prefixmap]
dc = "http://purl.org/dc/terms/"
```

> Generate a real starting point for your own project with `rudof config -o rudof.toml`,
> then edit in the keys you need from the tables above — you only ever need to set what
> you want to override.
