# The `--backend` flag

The `--backend` flag controls where RDF data is loaded and how it is queried. It is available on all commands that consume RDF data: `data`, `node`, `query`, `shacl`, `shacl-validate` and `shex-validate`.

## Accepted values

| Value | What it does |
|---|---|
| `memory` | Default. Parses the input(s) into an in-process `OxigraphInMemory` graph backed by `oxrdf::Graph`. |
| `qlever` | Launches a local QLever Docker container, builds an on-disk index from the input(s), and serves SPARQL via the container's HTTP endpoint. |
| `endpoint=<URL_OR_NAME>` | Queries a remote SPARQL endpoint by URL, or by the name of an endpoint registered in the TOML config. |

## The `memory` backend (default)

All input files are parsed eagerly into an in-process RDF graph. This is the simplest option and works with local files, remote URLs, and stdin.

```sh
rudof data simple.ttl                       # memory is the implicit default
rudof data --backend memory simple.ttl     # same, made explicit
```

## The `endpoint` backend

Routes all SPARQL queries to an external SPARQL endpoint rather than loading data locally. The endpoint can be given as:

- a full URL: `--backend endpoint=https://query.wikidata.org/sparql`
- a short name registered in the TOML config: `--backend endpoint=wikidata`

```sh
rudof query -q my.sparql --backend endpoint=wikidata
rudof node -n :a --backend endpoint=https://my.sparql.server/sparql
```

## The `qlever` backend

The `qlever` backend routes data loading through a locally-launched QLever Docker container. QLever builds a compact on-disk index once and then answers SPARQL queries against it, so memory usage on subsequent runs is bounded by QLever's server cache rather than the dataset size. It is well-suited for large, read-heavy workloads.

### Requirements

- `rudof` must be built with the `qlever` feature:

  ```sh
  cargo install rudof_cli --features qlever
  ```

- A running Docker daemon. The first run will pull the `adfreiburg/qlever` image (~1 GB).
- Every input must be a file-system path (remote URLs and stdin are not supported for this backend).
- *(Optional, only if you pass compressed dumps)* A decompressor binary on `$PATH` for each compression family you intend to use:
  - **bzip2 (`.bz2`)**: install `lbzip2` (parallel, recommended) or fall back to `bzip2` (single-threaded).
  - **xz (`.xz`)**: `xz`.
  - **gzip (`.gz`)**: install `pigz` (parallel, recommended) or fall back to `gzip` (single-threaded).

### What happens under the hood

A QLever run has two phases, **build the index** (once) and **serve it** (every time). The build phase is idempotent: if the on-disk index already exists, it is skipped and the serve phase starts immediately.

1. **Resolve the index directory.** Either `[qlever].index_dir` from the TOML config, or a per-input fingerprint under the platform cache dir (typically `~/.cache/rudof/qlever/<fingerprint>/`). The fingerprint is derived from each input's path, format, graph IRI, and compression family, so different inputs get different directories and unrelated runs don't collide.
2. **Skip if already built.** QLever writes a `<name>.meta-data.json` marker last; if it's there, `rudof` jumps straight to step 6.
3. **Docker pre-flight.** Connect to the local Docker daemon, ping it, and pull the `adfreiburg/qlever` image if it isn't present locally (~1 GB on first run).
4. **Probe the image's CLI.** The upstream image has shipped two generations of binaries — v1 (`IndexBuilderMain` / `ServerMain`) and v2 (`qlever-index` / `qlever-server`). `rudof` runs each candidate with `-h` once to pick the one that works for the pinned image tag.
5. **Build the index.** A one-shot container runs the index builder with the right `-f` / `-F` / `-g` triples. Each input takes one of three paths:
   - **Native (`.ttl`, `.nt`, `.nq`)**: the file's parent directory is bind-mounted read-only into the container under `/inputs/<n>/`, and the file is passed as `-f /inputs/<n>/file -F <fmt>`. No copying, no parsing on the host.
   - **Compressed-native (`.nt.bz2`, `.ttl.xz`, …)**: no bind-mount. `rudof` spawns a host-side decompressor (see *Streaming compressed dumps below) and pipes its stdout into the container's stdin via Docker attach, with `-f -` on the QLever side. Nothing decompressed ever hits disk.
   - **Non-native (`.rdf`, `.jsonld`, `.trig`, `.n3`)**: transparently converted via `oxrdfio` into a shared conversion directory and then bind-mounted as in the native case. Quad-bearing sources (`TriG`, `JSON-LD`) are written as **N-Quads** to preserve graph info; triple-only sources (`RDF/XML`, `N3`) are written as **N-Triples**.
6. **Serve the index.** A long-running container is started, the QLever HTTP port is mapped to an ephemeral host port (or to `host_port` if configured), and `rudof` polls a SPARQL readiness probe until it succeeds.
7. **Query.** All subsequent `rudof` operations go over SPARQL-over-HTTP against the running container.

The index is **persisted by default** across `rudof` invocations, so the next run with the same inputs short-circuits at step 2 and goes straight to serving. Set `auto_delete_if_created = true` in the `[qlever]` config to wipe the index on shutdown when the current run created it.

### Streaming compressed dumps

The QLever backend can ingest compressed RDF dumps **without ever materialising the decompressed form on disk**.

When you pass a compressed file, `rudof`:

1. Probes `$PATH` for a decompressor for that family (preferring parallel implementations).
2. Spawns it on the host with the compressed file as input.
3. Pipes its stdout straight into the QLever container's stdin, where `IndexBuilderMain` reads it via `-f -`.

Nothing decompressed ever touches disk; the decompressor uses host CPU cores (not the container's), so multi-threaded `lbzip2` / `pigz`-style binaries get full benefit of your machine.

**Supported compression families** and the decompressors `rudof` looks for, in priority order:

| Extension | Tries (parallel first)           | Fallback (single-threaded) |
|-----------|----------------------------------|----------------------------|
| `.bz2`    | `lbzip2 -dc`, `lbzcat -dc`       | `bzip2 -dc`                |
| `.xz`     | `xz -dc -T0`, `xzcat -T0`        | *(same; `-T0` is harmless when the file isn't parallelisable)* |
| `.gz`     | `pigz -dc`                       | `gzip -dc`, `zcat`         |

**Accepted inner formats** are the QLever-native ones — `.nt`, `.ttl`, `.nq` — so the file must look like `dump.nt.bz2`, `data.ttl.xz`, etc. Non-native compressed inputs (e.g. `dump.jsonld.bz2`) are rejected with a clear error; decompress or pre-convert them to a native format first.

**Usage** is identical to the uncompressed flow, `rudof` notices the suffix automatically:

```sh
rudof data latest-truthy.nt.bz2 --backend qlever
rudof query -q my.sparql data.nt.xz --backend qlever
```

On startup, `rudof` logs which decompressor it picked:

```
streaming compressed input (bz2) via /usr/bin/lbzip2 (parallel)
```

**Limitations of the compressed-dump path:**

- **At most one compressed input per index build.** You can mix one compressed file with any number of uncompressed files in the same invocation, but two compressed inputs in the same build are rejected (the index builder reads at most one stream via stdin).
- **Native inner format only.** As above: `.jsonld.bz2`, `.trig.xz`, etc. are rejected.
- **Decompressor must be on `$PATH`.** If no decompressor for the input's family is found, `rudof` errors with the list of binaries it tried so you know what to install.

### Configuring the QLever backend

QLever-specific settings live in a `[qlever]` section of the TOML config file passed via `--config-file`:

```toml
[qlever]
# Docker image. Defaults: adfreiburg/qlever : commit-a307781.
image_name = "adfreiburg/qlever"
image_tag  = "commit-a307781"

# Where on the host the index lives. If unset, defaults to <XDG_CACHE_HOME>/rudof/qlever/<input-fingerprint>.
# index_dir = "/var/lib/rudof/qlever"

# Index base name (the -i argument to QLever).
index_name = "default"

# Wipe the index on Drop if this run created it. Default: false.
auto_delete_if_created = false

# Pin the host port. If unset, Docker picks an ephemeral one.
# host_port = 7001

# Container-side port the QLever server binds to.
container_port = 7001

# QLever server flags (see IndexBuilderMain -h / ServerMain -h).
memory_max_size           = "5G"  # -m
cache_max_size            = "2G"  # -c
cache_max_size_single_entry = "1G" # -e
# stxxl_memory            = "2G"  # -m on the index builder
# parser_buffer_size      = "10M"
# access_token            = "…"   # -a (for SPARQL UPDATE)
# num_simultaneous_queries = 4    # -j
# default_query_timeout   = "30s" # -s
# no_patterns             = false # -P
# no_pattern_trick        = false # -T
# text                    = false # -t
# only_pso_and_pos_permutations = false # -o
# throw_on_unbound_variables    = false

# Run the container as the host UID/GID so index files end up owned by you
# (Linux only). Default: true.
run_as_host_user = true

# Seconds to wait for the server to answer a SPARQL probe before giving up.
server_readiness_timeout_secs = 60
```

All `[qlever]` fields are optional (an empty `[qlever]` section is enough to start QLever with sensible defaults).

> **Note:** The `[qlever]` section in the config is only recognised when `rudof` is built with the `qlever` feature.

### Limitations

- **Read-only:** `add_triple`, `remove_triple`, and `add_bnode` return errors.
- **File paths only:** remote URLs and stdin are not accepted as inputs; mixed input sets produce an error.
- **`--data-format pg` is rejected:** the property-graph format is incompatible with QLever's indexer.
- **Strict Turtle syntax:** QLever's index builder rejects SPARQL-style prefix declarations (`prefix : <…>` without `@` and without a trailing `.`) even though oxigraph accepts them. Use Turtle-conformant `@prefix : <…> .` declarations (or run files through a normalizer) before passing them to `--backend qlever`.
