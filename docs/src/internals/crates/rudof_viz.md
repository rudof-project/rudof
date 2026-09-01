# `rudof_viz`

`rudof_viz` holds every technology-agnostic visualization primitive used across Rudof: box/arrow styles, a small diagram model (boxes, connectors, style sheets), and the renderer traits a backend implements. It exists so that RDF-graph visualization ([`rudof_rdf`](./rudof_rdf.md)) and ShEx-to-UML visualization (`shapes_converter`) share one diagram model and one set of styling concepts, instead of each hand-rolling its own PlantUML text generation — and so a new rendering technology can be added in one place without touching either of those two domain crates.

Two backends exist today: **PlantUML** (the default, via `java -jar plantuml.jar`) and **Graphviz** (via the `dot` command, selectable with `--viz-engine graphviz` — see [convert: Choosing a visualization engine](../../cli_usage/convert.md#choosing-a-visualization-engine)).

## Architecture and Package Structure

- **`style`**: `Color` (a small named palette), `LineStyle` (bold/normal/dashed/dotted), `BoxStyle` (line/background color, thickness, corner rounding), `ArrowStyle` (line color, thickness, text color), and `StyleRule`/`StyleSheet` — a collection of named style rules a box's `stereotype` is matched against (used by the RDF visualizer's per-term-kind styling; ShEx UML diagrams use `ClassSkin` instead, below).
- **`model`**: the diagram itself.
  - `Diagram`: boxes, connectors, a `StyleSheet`, and a handful of opt-in presentation hints backends may or may not honor (`direction`, `line_type`, `hide_empty_members`, `hide_circles`, `shadowing`, `class_skin`) — a backend is free to ignore any hint it has no equivalent for (Graphviz has no drop-shadow, so it ignores `shadowing`, for example).
  - `DiagramBox`: one node — an id, a `Shape` (`Rectangle`, `Cloud`, or `Class`), a title, an optional `href`, an optional stereotype, and (for `Class` boxes) a list of pre-formatted attribute-line `compartments`.
  - `Connector`: one edge — source/target `BoxId`s, a `ConnectorKind` (`Association` or `Generalization`), an optional label, an optional `target_decoration` (e.g. a ShEx cardinality like `0..*`), and an optional `ArrowStyle`.
  - `ClassSkin`: default border/background/arrow colors for `Class` boxes, read by both backends so a UML diagram looks the same regardless of engine — see [`[shex2uml].class_skin`](../../references/config.md#shex2umlclass_skin).
  - `DiagramScope`: `All` or `Neighs(title)`, plus `Diagram::scoped`/`scoped_by_id` to restrict a diagram to one box and its immediate neighbours (used by `-l`/`--shape-label` on `shex`/`convert`).
- **`render`**: the two traits every backend implements.
  - `DiagramRenderer::render` — diagram → backend-specific text (PlantUML source, or DOT).
  - `ExternalToolRenderer::render_image` — a default-implemented method (shared by every backend) that writes the diagram to a temp file, spawns the backend's external tool via `build_command`, and copies the result to the caller's writer. A backend only implements `build_command` (how to invoke its tool) and `output_file_name`.
  - `RenderError` — the shared error type across both traits and both backends.
- **`backends`**: the concrete implementations.
  - `plantuml::PlantUmlBackend` — emits `@startuml`/`@enduml` PlantUML source; `Class` boxes become `class "Name" <<...>> { ... }` blocks, `Rectangle`/`Cloud` boxes become `rectangle`/`cloud` elements styled from the `StyleSheet`. Its `ExternalToolRenderer` impl shells out to `java -jar <plantuml_path>`.
  - `graphviz::GraphVizBackend` — emits a `digraph { ... }`; `Class` boxes become an HTML-like `<TABLE>` label (the standard Graphviz "UML class box" idiom — Graphviz doesn't have a native UML class shape), `Rectangle` boxes become `shape=box`, and `Cloud` boxes become `shape=ellipse` (Graphviz's shape catalog has no `cloud` — `dot` would silently substitute a plain box and print a warning, so `Cloud` uses the closest portable shape instead). Its `ExternalToolRenderer` impl shells out to `dot -T{svg,png}`.
- **`engine`**: `VizEngine` (`PlantUml`/`GraphViz`) plus `render_with_engine`/`render_image_with_engine` — the two free functions that pick a concrete backend by `match`, so callers that just have a `VizEngine` value (threaded from the CLI's `--viz-engine` flag) don't need to know about the backend types directly.

### A note on hyperlinks

Both `rudof_rdf` and `shapes_converter` build some label/compartment strings that embed PlantUML's own `[[url text]]` hyperlink syntax directly (a pre-existing convention from before `rudof_viz` existed, not a `rudof_viz` concept). `PlantUmlBackend` passes these through unchanged. `GraphVizBackend` parses and strips them instead — Graphviz's HTML-like labels only support links at the whole-cell/whole-table granularity, not arbitrary inline spans, so it keeps the first `url` found (as the box/edge's `URL`/`HREF` attribute) and renders the rest as plain text. This means a compartment line with several distinct links (e.g. a ShEx value set referencing more than one datatype) only keeps its first link in Graphviz output — a known, deliberately scoped limitation rather than a bug.

## Dependents and dependencies

`rudof_viz` has no dependency on any other Rudof crate — only `serde`, `thiserror`, and (off the `wasm` target) `tempfile` for the external-tool temp-file plumbing.

It's depended on by:

- [`rudof_rdf`](./rudof_rdf.md) — `VisualRDFGraph::to_diagram()` builds a `Diagram` from an RDF graph; `VisualRDFGraph::as_plantuml`/`as_image` render it.
- `shapes_converter` — `Uml::to_diagram()` builds a `Diagram` from a ShEx schema's UML model; `ShEx2Uml::as_plantuml`/`as_image` render it.

Neither domain crate depends on the other for visualization anymore — before `rudof_viz` existed, `shapes_converter` depended on `rudof_rdf` solely to reuse its PlantUML-rendering trait.

## Adding a new backend

A new visualization technology only needs a type in `rudof_viz::backends` implementing `DiagramRenderer` (and, for image output, `ExternalToolRenderer`) — see `backends/graphviz.rs` for a complete, relatively small example to copy. Wiring it up as a `--viz-engine` choice needs three more small, mechanical steps outside `rudof_viz`: a `VizEngine` variant plus a `match` arm in `engine.rs`'s two dispatch functions, a `VizEngine` parameter threaded through `VisualRDFGraph::as_image`/`ShEx2Uml::as_image` (already generic over the engine, so this is copy-paste), and a `cli_wrapper!`-generated CLI enum variant in `rudof_cli`.

## Documentation

The crate documentation can be found [here](https://docs.rs/rudof_viz).
