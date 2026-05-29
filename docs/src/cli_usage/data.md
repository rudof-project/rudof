# Information about RDF data

The `data` command can be used to parse one or more RDF data files in different formats.

## Obtaining information about an RDF data file

Assuming the following content appears in a file called `simple.ttl`:

```turtle
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:a :name "Alice" ;
   :birthdate "1990-05-02"^^xsd:date ;
   :enrolledIn :cs101 .

:b :name "Bob", "Robert" .

:cs101 :name "Computer Science" .
```

The following command parses the file `simple.ttl` and shows its contents:

```sh
rudof data simple.ttl
```

## Converting between different RDF data syntaxes

It is possible to specify the output format to serialize the RDF data using the `-r` option. Possible values are: rdfxml, ntriples, trig, etc.:

```sh
rudof data -r rdfxml simple.ttl
```

The output would be something like:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:="http://example.org/"
   xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
   xmlns:xsd="http://www.w3.org/2001/XMLSchema#">
        <rdf:Description rdf:about="http://example.org/cs101">
                <name>Computer Science</name>
        </rdf:Description>
        <rdf:Description rdf:about="http://example.org/b">
                <name>Bob</name>
                <name>Robert</name>
        </rdf:Description>
        <rdf:Description rdf:about="http://example.org/a">
                <enrolledIn rdf:resource="http://example.org/cs101"/>
                <birthdate rdf:datatype="http://www.w3.org/2001/XMLSchema#date">1990-05-02</birthdate>
                <name>Alice</name>
        </rdf:Description>
</rdf:RDF>
```

It is possible to convert RDF data to a visual representation using the options `svg`, `png` or `plantuml` (see [RDF visualization](##RDF-visualization) section).

## Obtaining information about an RDF data located remotely

It is also possible to get RDF data from files which are remotely available through URIs like:

```sh
rudof data https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple.ttl
```

## Merging RDF data

The `data` command can also be used to parse more than one RDF data files, merge them and serialize them to any of the RDF formats supported.

We carse several files, merge and serialize them in any of the RDF supported formats.

```sh
rudof data user.ttl simple.ttl -r rdfxml -o output.rdf
```

> It is possible to serialize the files using a different format, like `ntriples`, `rdfxml`, etc.

## RDF visualization

It is possible to generate a visual representation of simple RDF graphs by using the `--result-format` option and selecting a visual format like `svg` or `png`.

The visualization is leveraged on PlantUML so it is necessary to have the PlantUML binary downloaded and available through the `PLANTUML` variable.

Another alternative is to use the `plantuml`  result format to generate an intermediate file and pass that file to some PlantUML processor.

As an example, the following command generates a `plantuml` file:

```sh
rudof data examples/simple.ttl -r plantuml -o file.plantuml
```

If you have PLANT_UML available you can use directly:

```sh
rudof data examples/simple.ttl -r svg -o file.svg
```


## Selecting the RDF backend

By default, `rudof` parses the input file(s) into an in-process RDF graph (the `memory` backend). The `--backend` flag lets you change where the data lives and how it is queried:

| Value                          | What it does                                                                                                                       |
|--------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| `memory`                       | Default. Parses the input(s) into an in-process `OxigraphInMemory` graph backed by `oxrdf::Graph`. |
| `qlever`                       | Launches a local `QLever` Docker container, builds an on-disk index from the input(s), and serves SPARQL via the container's HTTP endpoint. |
| `endpoint=<URL_OR_NAME>`       | Query a remote SPARQL endpoint by URL, or by the name of an endpoint registered in the TOML config. |

### The QLever Docker backend

The `qlever` value routes data loading through a locally-launched QLever container. QLever builds a compact on-disk index once and then answers SPARQL queries against it from a long-running container, so memory usage on subsequent runs is bounded by QLever's server cache (not by the dataset size).


#### Requirements

- The `rudof` binary must be built with the `qlever` feature:

  ```sh
  cargo install rudof_cli --features qlever
  ```

- A running Docker daemon. The first run will pull the `adfreiburg/qlever` image (~1 GB).
- Every input must be a file-system path (remote URLs and stdin are not supported for this backend; mixing them in produces a clear error).

#### Basic usage

```sh
rudof data --backend qlever simple.ttl
```

Multiple inputs are merged into a single QLever index in one pass:

```sh
rudof data --backend qlever people.ttl orgs.nt extras.ttl
```

The `--data-format` flag, when given, overrides extension-based detection (handy for files without a recognised extension):

```sh
rudof data --backend qlever --data-format turtle dump.dat
```

What happens under the hood:

1. The image is pulled if it is not already present locally.
2. An index directory is materialized under the platform cache dir (typically `~/.cache/rudof/qlever/<fingerprint>/`), unless a directory is explicitly configured.
3. If the input is in a non-native format (anything other than `ttl`, `nt`, `nq`), it is transparently converted to N-Triples before indexing.
4. A QLever container is started, mapped to an ephemeral host port, and a readiness probe waits until it answers SPARQL queries.
5. Subsequent `rudof` commands targeting the same input reuse the existing index (the indexing step is skipped when the on-disk index files are already present).

The index is **persisted by default** across `rudof` invocations so repeated runs skip indexing. See `auto_delete_if_created` below to change that.

#### Configuring the QLever backend

QLever-specific settings live in a `[qlever]` section of the TOML config file passed via `--config-file`:

```toml
[qlever]
# Docker image. Defaults: adfreiburg/qlever : commit-a307781.
image_name = "adfreiburg/qlever"
image_tag = "commit-a307781"

# Where on the host the index lives. If unset, defaults to <XDG_CACHE_HOME>/rudof/qlever/<input-fingerprint>.
# index_dir = "/var/lib/rudof/qlever"

# Index base name (the `-i` argument to QLever).
index_name = "default"

# Wipe the index on Drop if this run created it. Default: false.
auto_delete_if_created = false

# Pin the host port. If unset, Docker picks an ephemeral one.
# host_port = 7001

# Container-side port the QLever server binds to.
container_port = 7001

# QLever server flags (see `IndexBuilderMain -h` / `ServerMain -h`).
memory_max_size = "5G"               # -m
cache_max_size = "2G"                # -c
cache_max_size_single_entry = "1G"   # -e
# stxxl_memory = "2G"                # -m on the index builder
# parser_buffer_size = "10M"
# access_token = "…"                 # -a (for SPARQL UPDATE)
# num_simultaneous_queries = 4       # -j
# default_query_timeout = "30s"      # -s
# no_patterns = false                # -P
# no_pattern_trick = false           # -T
# text = false                       # -t
# only_pso_and_pos_permutations = false  # -o
# throw_on_unbound_variables = false

# Run the container as the host UID/GID so index files end up owned by you
# (Linux only). Default: true.
run_as_host_user = true

# Seconds to wait for the server to answer a SPARQL probe before giving up.
server_readiness_timeout_secs = 60
```

All `[qlever]` fields are optional (an empty section is enough to bring QLever up with sensible defaults).

#### Limitations

- The backend is **read-only**: `add_triple`, `remove_triple` and `add_bnode` all return errors. Use `memory` for write-heavy workflows.
- Only file-system paths are accepted as inputs (no URLs / stdin). Mixed input sets produce a clear error rather than a silently partial index.
- The non-RDF `pg` data format is rejected up front (`--data-format pg` is incompatible with `--backend qlever`).
- QLever's index builder is **strict about Turtle syntax**: SPARQL-style header declarations (`prefix : <…>` without `@` and without a trailing `.`) are rejected even though oxigraph accepts them. Use Turtle-conformant `@prefix : <…> .` declarations (or run files through a normalizer) before passing them to `--backend qlever`.
- The `[qlever]` section in the config is recognized only when `rudof` is built with the `qlever` feature.

## RDF Config file

The parameter `--config-file`  (`-c` in short form) can be used to pass a configuration file in [TOML](https://toml.io/) format.

The fields that it can contain are:

- base (IRI): Default base declaration to resulve relative IRIs
