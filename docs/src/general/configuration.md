# Configuration

Most commands can be customized either by passing parameters at run-time or through a
configuration file. Config files use [TOML syntax](https://toml.io/).

## Where configuration comes from

rudof builds its effective configuration by layering several sources. From lowest to
highest precedence:

1. **Built-in defaults** — every setting has a sensible default, so a config file only
   needs to mention what it wants to override.
2. **User config file** — a per-user `config.toml` in the platform config directory:
   - Linux: `~/.config/rudof/config.toml`
   - Windows: `%LOCALAPPDATA%\rudof\config.toml`
   - macOS: `~/Library/Application Support/rudof/config.toml`
3. **Project `rudof.toml` files** — starting from the current working directory and
   walking up to the filesystem root, every `rudof.toml` found is merged.
4. **CLI flags** — options passed on the command line always take precedence.

Sources 1–3 are merged **per key** ("deep merge"): a file only overrides the keys it
actually sets and inherits everything else from the layers below it.

### Explicit `--config` bypasses discovery

Passing an explicit file with `--config-file` / `-c` flag makes the tool use only that configuration file.
There is no discovery when the flag is present.

## Inspecting the effective configuration

To see exactly what rudof resolved from all of the above, use the `config` command, which
dumps the effective configuration as TOML:

```sh
rudof config                 # print to stdout
rudof config -o rudof.toml   # write to a file (a good starting template)
```

## Config file format

The configuration is a set of TOML tables, one per subsystem. A minimal example:

```toml
# Common settings live at the top level.
base_iri = "http://example.org/"
auto_base = false

# RDF data
[rdf]
base_iri = "http://example.org/"

# ShEx schema handling.
[shex]
show_imports = true

# ShEx validation
[shex_validator]
max_steps = 100
check_negation = true

# Other subsystems: [shacl], [tap], [tap2shex], [shex2uml], [shex2html],
# [shex2sparql], [service], [comparator].
```

A per-section `base_iri` (e.g. under `[rdf]`) overrides the top-level `base_iri` for that
section only; sections that don't set their own inherit the common one. Shared sections
such as `[rdf]`, `[shex]` and `[tap]` are automatically injected into the subsystems that
build on them (validation, conversion, SHACL), so you configure them **once**.

## Loading configs programmatically

Every configuration struct in rudof implements the `TomlConfig` trait from the
[`rudof_config`](https://github.com/rudof-project/rudof/tree/master/rudof_config) crate,
which provides a uniform API:

- `from_toml_str(&str)` — parse from a TOML string.
- `from_path(path)` — load from a TOML file.
- `to_toml_string()` — serialize back to TOML.

The aggregate [`RudofConfig`](https://github.com/rudof-project/rudof/blob/master/rudof_lib/src/config/rudof.rs)
additionally offers `discover()`, which implements the layered precedence described above.