# Overview

`rudof` is a library that implements: [Shape Expressions](https://shex.io/), [SHACL](https://www.w3.org/TR/shacl/), [DCTAP](https://www.dublincore.org/specifications/dctap/) and other technologies in the [RDF](https://www.w3.org/RDF/) ecosystem.

The library is implemented in [Rust](https://www.rust-lang.org/) and it also provides Python bindings.

`rudof` can be used as a command line tool or can be embedded as a library.
It can be used to validate RDF data represented with different syntaxes like Turtle, NTriples, etc. as well as RDF data available through SPARQL endpoints like Wikidata.

`rudof` can also be used to convert between different RDF data validation technologies like ShEx, SHACL, DCTAP, etc. and to generate UML like visualizations and HTML views.

- We publish binaries in Linux, Windows and Apple which can be downloaded [here](https://github.com/weso/shex-rs/releases/).
- Source code: [https://github.com/rudof-project/rudof](https://github.com/rudof-project/rudof)
- [Paper about rudof](pdf/rudof_demo.pdf) presented at International Semantic Web Conference, Baltimore, USA, October 2024 in Demos and Posters session.
- [List of issues](https://github.com/rudof-project/rudof/issues)
- [Discussion](https://github.com/rudof-project/rudof/discussions)
- [How-to guides](https://github.com/rudof-project/rudof/wiki/How%E2%80%90to-guides)
- [FAQ](https://github.com/rudof-project/rudof/wiki/FAQ)

## Modules

rudof has been implemented using a modular structure and the different modules are available as Rust crates

- [ShEx validator](https://docs.rs/shex_validation/): ShEx Validator
- [ShEx compact](https://docs.rs/shex_compact): ShEx Compact syntax parser that follows [ShEx compact grammar](https://shex.io/shex-semantics/index.html#shexc)
- [ShEx ast](https://docs.rs/shex_ast): Represents [ShEx Abstract syntax](https://shex.io/shex-semantics/index.html#shape-expressions-shexj) based on ShExJ (JSON-LD)
- [ShapeMap](https://docs.rs/shapemap/): [ShapeMap](https://shexspec.github.io/shape-map/) implementation.
- [SRDF](https://docs.rs/srdf): Simple RDF interface
- [PrefixMap](https://docs.rs/prefixmap): Turtle based prefixMap representation
- [Conversions between different shapes formalisms](https://docs.rs/shapes_convert)
[](https://docs.rs/shapes_convert)*   [](https://docs.rs/shapes_convert)[SHACL ast](https://docs.rs/shacl_ast): Represents [SHACL core abstract syntax](https://www.w3.org/TR/shacl)
- [RBE](https://docs.rs/rbe): Regular Bag Expressions
- [ShEx testsuite](https://docs.rs/shex_testsuite/): Code that checks the [ShEx testsuite](https://shexspec.github.io/test-suite/)

Related projects
----------------

An incomplete list of projects which are related with `rudof` is the following:

- [ShEx-s](https://www.weso.es/shex-s/): Scala implementation of ShEx. This project started as a re-implementation of ShEx-s in Rust
- [SHACL-s](https://www.weso.es/shacl-s/): Scala implementation of SHACL.
- [ShEx.js](https://github.com/shexjs/shex.js): Javascript implementation of ShEx.
- [Oxigraph](https://github.com/oxigraph/oxigraph): SPARQL implementation in Rust which also contains RDF libraries.
- [Nemo](https://github.com/knowsys/nemo): An in-memory rule engile which also contains some nom parsers.

## Contributors

[List of contributors in Github](https://github.com/rudof-project/rudof/graphs/contributors)

## Supporters and adopters

The following is a list of `rudof` adopters and supporters:

- [WESO (WEb Semantics Oviedo)](http://www.weso.es/): Most of the contributors are part of this research group at the [University of Oviedo](http://www.uniovi.es)
- [USDA - United States Department of Agriculture](https://www.usda.gov/) has been partially funding part of the project through a Non-Assistance Cooperative Agreement.
