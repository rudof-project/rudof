# bench-issue730

End-to-end timing of the ShEx validation pipeline on the corpus from [issue #730](https://github.com/rudof-project/rudof/issues/730).

## What it measures

Total validation cost as a function of **nested chained shape depth**. Each case `N_steps/` contains a schema with a chain of `N` shapes where each links to the next via a triple constraint (`step_0 → step_1 → ... → step_{N-1}`), an RDF graph shaped as the matching chain of nodes, and a shapemap that pins one starting node to the head shape. Validating that node forces the engine to recurse the full chain. `N` doubles per case from 4 to 1024.

Each iteration goes through the public `rudof_lib` API: `load_shex_schema` → `load_data` → `load_shapemap` → `validate_shex`, on a fresh `Rudof` instance.

## Usage

```sh
cargo run --release -p bench-issue730
```

The bundled `corpus/shex_shapes.zip` is auto-extracted on first run under `target/bench-issue730-corpus/`.

## Output

Prints a table to stdout with two columns: `steps` and `total (ms)`, the **mean of 5 runs** of the full pipeline for each chain length.
