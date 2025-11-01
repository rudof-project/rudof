[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.16744091.svg)](https://doi.org/10.5281/zenodo.16744091)
[![CI](https://github.com/weso/pgschemapc/actions/workflows/ci.yml/badge.svg)](https://github.com/weso/pgschemapc/actions/workflows/ci.yml)

# PG-Schema_PC

This repository contains a prototype implementation for the PG-Schema with Property Constraints extension.


## Overview

PG-Schema_PC is a formal extension of the PG-Schema language, designed to support constraints over property sets in property graphs. 
It introduces structural, cardinality, and range constraints that enhance the precision and expressiveness of schema definitions. 
This implementation serves as a reference interpreter for the abstract grammar and semantics presented in the associated publication.


## Installation and building

Binary releases in Windows, Mac and Linux are published in [relases](https://github.com/weso/pgschemapc/releases).

If you prefer to build the code, you should install [cargo](https://doc.rust-lang.org/cargo/) and the following command compiles and generates an executable called `pgschemapc` in the `target/release` folder:

```sh
cargo build --release
```

which will create a binary in `target/release/pgschemapc`.

Once you have the `pgschemapc` binary, you can add it to your executable path. 

The command `pgschemapc --help` gives information about the available commands. 

```sh
pgschemapc --help
A simple prototype tool to process and validate PG-Schemas with property constraints

Usage: pgschemapc [COMMAND]

Commands:
  pgs       Process and validate property graph schemas
  pg        Process and validate property graphs
  map       Process and validate type map associations
  validate  Validate a property graph with a property graph schema and some associated type map
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Running a simple validation

The following command shows how the tool can be used to validate a simple property graph with a PG-Schema_PC schema according to some type maps:

```sh
pgschemapc validate --graph examples/simple.pg --schema examples/simple.pgs --map examples/simple.map
```

## Running examples from paper

The different examples from the submitted paper can be run using the following commands: 

```sh
pgschemapc validate --graph examples/[EXAMPLE_NAME].pg --schema examples/[EXAMPLE_NAME].pgs --map examples/[EXAMPLE_NAME].map
```

where `[EXAMPLE_NAME]` can be any of `simple`, `course`, `person`, `product`, `user`, etc. 

Of course, you can play with the tool by creating your own property graph, property graph schema and association maps.

## Running the tests

To run a test suite use the command:

```sh
cargo test
```

At this stage, the tool is a prototype for PGSchema with property constraints validation. 
Further integration with graph databases or external datasets will require extending the parsing and validation layers.

## Related Resources

- Railroad diagrams: [https://domel.github.io/pg-schema-pc/rr-diagrams.html](https://domel.github.io/pg-schema-pc/rr-diagrams.html)
- Citation metadata: see `CITATION.cff`
- GitHub repository for grammar: [https://github.com/domel/pg-schema-pc](https://github.com/domel/pg-schema-pc)

## Citing

If you use this software or refer to its underlying formalism in academic work, please cite it using the metadata in the `CITATION.cff` file.

---

Licensed under the MIT License.
