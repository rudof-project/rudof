# compare: Compare Shapes and RDF data

`rudof` supports comparison between different schemas and shapes (`--mode schema`, the default) and between RDF graphs (`--mode data`, see [below](#comparing-rdf-data)).

The `compare` has the following structure:

```
$ rudof compare --help
Compare two schemas or shapes (which can be in different formats), or two RDF graphs (--mode data)

Usage: rudof compare [OPTIONS] --schema2 <INPUT>

Options:
      --mode <MODE>
          What to compare: two schemas, or two RDF graphs

          Possible values:
          - schema: Two schemas (ShEx, SHACL, ...), shape by shape
          - data:   Two RDF graphs, up to blank node renaming

          [default: schema]

      --mode1 <MODE>
          Input mode first schema (schema mode)

          [default: shex]
          [possible values: shacl, shex, dctap, service]

      --mode2 <MODE>
          Input mode second schema (schema mode)

          [default: shex]
          [possible values: shacl, shex, dctap, service]

      --schema1 <INPUT>
          First schema or RDF graph (URI, file or - for stdin). In data mode, if omitted, the loaded data (in the shell)

          [alias: --data1]

      --schema2 <INPUT>
          Second schema or RDF graph (URI, file or - for stdin)

          [alias: --data2]

      --format1 <FORMAT>
          Format of the first input [default: shexc in schema mode, turtle in data mode]

          [possible values: shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads, jsonld]

      --format2 <FORMAT>
          Format of the second input [default: shexc in schema mode, turtle in data mode]

          [possible values: shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads, jsonld]

      --base1 <IRI>
          Base IRI of the first input

      --base2 <IRI>
          Base IRI of the second input

      --subgraph-mode <MODE>
          Data mode: how blank nodes may be mapped when checking containment: one-to-one onto blank nodes (injective), onto blank nodes (homomorphic), or onto any term, i.e. simple entailment (entailment)

          [default: injective]
          [possible values: injective, homomorphic, entailment]

  -r, --result-format <FORMAT>
          Result format

          [default: internal]
          [possible values: internal, json]

  -t, --target-folder <FOLDER>
          Target folder

      --shape1 <LABEL>
          shape1 (default = START, schema mode)

      --shape2 <LABEL>
          shape2 (default = START, schema mode)

      --reader-mode <MODE>
          RDF Reader mode

          [default: strict]
          [possible values: lax, strict]

      --show-time <SHOW_TIME>
          Show processing time

          [possible values: true, false]

  -c, --config-file <FILE>
          Config file name

  -o, --output-file <FILE>
          Output file name, default = terminal

      --force-overwrite
          Force overwrite to output file if it already exists

  -h, --help
          Print help (see a summary with '-h')
```

## Example

The following command can be used to compare the shape `http://example.org/Person` in the schemas `examples/shex/compare1.shex` and `examples/shex/compare2.shex`:

```sh
$ rudof compare --schema1 examples/shex/compare1.shex --format1 shexc --mode1 shex --schema2 examples/shex/compare2.shex --format2 shexc --mode2 shex --shape1 "http://example.org/Person" --shape2 "http://example.org/Person"
Shapes Comparison:
 Equal properties:
  - http://example.org/knows:
  - descr1:   - value: http://example.org/knows
  - datatype: _

  - descr2:   - value: http://example.org/knows
  - datatype: _


  - http://example.org/name:
  - descr1:   - value: http://example.org/name
  - datatype: _

  - descr2:   - value: http://example.org/name
  - datatype: _


 Properties in shape 1 that are not in shape 2:
  - http://example.org/worksFor:
  - descr:   - value: http://example.org/worksFor
  - datatype: _


  - http://example.org/age:
  - descr:   - value: http://example.org/age
  - datatype: _


 Properties in shape 2 that are not in shape 1:
  - http://example.org/email:
  - descr:   - value: http://example.org/email
  - datatype: _


  - http://example.org/birthDate:
  - descr:   - value: http://example.org/birthDate
  - datatype: _
```

The output of the comparison can be in compact form (by default) or in JSON using the option `-r json`:

```sh
$ rudof compare --schema1 examples/shex/compare1.shex --format1 shexc --mode1 shex --schema2 examples/shex/compare2.shex --format2 shexc --mode2 shex --shape1 "http://example.org/Person" --shape2 "http://example.org/Person" -r json
{
  "equal_properties": {
    "http://example.org/knows": {
      "description1": {
        "iri_ref": "http://example.org/knows"
      },
      "description2": {
        "iri_ref": "http://example.org/knows"
      }
    },
    "http://example.org/name": {
      "description1": {
        "iri_ref": "http://example.org/name"
      },
      "description2": {
        "iri_ref": "http://example.org/name"
      }
    }
  },
  "properties1": {
    "http://example.org/worksFor": {
      "description": {
        "iri_ref": "http://example.org/worksFor"
      }
    },
    "http://example.org/age": {
      "description": {
        "iri_ref": "http://example.org/age"
      }
    }
  },
  "properties2": {
    "http://example.org/email": {
      "description": {
        "iri_ref": "http://example.org/email"
      }
    },
    "http://example.org/birthDate": {
      "description": {
        "iri_ref": "http://example.org/birthDate"
      }
    }
  }
}
```

### Python

In `Python` there is an equivalent method in the class [Rudof](https://pyrudof.readthedocs.io/en/latest/library.html#rudof) called [`compare_schemas_str`](https://pyrudof.readthedocs.io/en/latest/library.html#pyrudof.Rudof.compare_schemas_str).

## Comparing RDF data

With `--mode data`, `compare` checks whether two RDF graphs, given with `--data1` and `--data2` (aliases of `--schema1` and `--schema2`), are the same up to blank node renaming (isomorphic), or whether one is contained in the other. RDF 1.2 triple terms are supported: blank nodes inside triple terms are mapped like any other blank node.

The result is one of:

| Result | Meaning |
|--------|---------|
| `isomorphic` | Both graphs are equal up to blank node renaming |
| `equivalent` | Each graph is contained in the other, but they are not isomorphic (only with `--subgraph-mode homomorphic` or `entailment`) |
| `subgraph` | The first graph is contained in the second, but not the other way round |
| `supergraph` | The second graph is contained in the first, but not the other way round |
| `different` | Neither graph is contained in the other |

### Subgraph modes

`--subgraph-mode` says what the blank nodes of a graph may be mapped to when checking whether it is contained in the other one:

- `injective` (default): one-to-one onto blank nodes, so the first graph is isomorphic to a subgraph of the second.
- `homomorphic`: onto blank nodes, several possibly onto the same one.
- `entailment`: onto any term (IRI, literal, blank node or triple term), several possibly onto the same one. This is [simple entailment](https://www.w3.org/TR/rdf12-semantics/#simpleentailment): `subgraph` means the second graph entails the first.

For example, with `general.ttl`:

```turtle
prefix : <http://example.org/>
:alice :knows [ :name "Bob" ] .
```

and `specific.ttl`:

```turtle
prefix : <http://example.org/>
:alice :knows :bob .
:bob :name "Bob" .
```

```sh
$ rudof compare --mode data --data1 general.ttl --data2 specific.ttl
different: neither the first graph nor the second graph is contained in the other (mode: injective)
$ rudof compare --mode data --data1 general.ttl --data2 specific.ttl --subgraph-mode entailment
subgraph: the first graph is contained in the second graph, but not the other way round (mode: entailment)
```

A graph with redundant blank nodes (`:a :p [], [] .`) and its lean version (`:a :p [] .`) are `subgraph` with the default mode, and `equivalent` with the other two.

<div class="warning">These checks are NP-complete in general: the worst case is exponential in the number of blank nodes. Ground triples are checked directly and the search is pruned, so typical data is fast.</div>

### Formats and base IRIs

Both graphs are read as Turtle unless `--format1`/`--format2` say otherwise (`turtle`, `ntriples`, `rdfxml`, `trig`, `n3`, `nquads` or `jsonld`); `--base1` and `--base2` set their base IRIs. `--shape1`, `--shape2`, `--mode1`, `--mode2` and `--target-folder` only apply to schemas.

```sh
$ rudof compare --mode data --data1 data.ttl --data2 data.nt --format2 ntriples
isomorphic: the first graph and the second graph are equal up to blank node renaming (mode: injective)
```

### JSON output

`-r json` prints the result for programs:

```sh
$ rudof compare --mode data --data1 general.ttl --data2 specific.ttl --subgraph-mode entailment -r json
{
  "mode": "entailment",
  "relation": "subgraph"
}
```

### In the shell

In the [shell](./shell.md), `--data1` can be left out: the loaded data is then the first graph, compared with `--data2`:

```
rudof> data general.ttl
2 triple(s) loaded
rudof> compare --mode data --data2 specific.ttl --subgraph-mode entailment
subgraph: the loaded data is contained in the given graph, but not the other way round (mode: entailment)
```
