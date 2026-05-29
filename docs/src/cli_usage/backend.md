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

### What happens under the hood

1. The `adfreiburg/qlever` image is pulled if not already present locally.
2. An index directory is created under the platform cache dir (typically `~/.cache/rudof/qlever/<fingerprint>/`), unless a directory is explicitly configured.
3. If the input is in a non-native format (anything other than `ttl`, `nt`, `nq`), it is transparently converted to N-Triples before indexing.
4. A QLever container is started, mapped to an ephemeral host port, and a readiness probe waits until it answers SPARQL queries.
5. Subsequent `rudof` commands targeting the same input reuse the existing index (the indexing step is skipped when the on-disk index files are already present).

The index is **persisted by default** across `rudof` invocations so repeated runs skip indexing. See `auto_delete_if_created` below to change that.

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
