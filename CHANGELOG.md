# CHANGE LOG

## Current changes without release yet

## [v0.1.80] - 2025-07-11

- Added the possibility to convert between ShEx to ShEx (with different formats) and SHACL to SHACL (with different formats) to the `convert` command in the command line.
- Refactor the SHACL Intermediate representation
- Added support to language ValueSetValue in ShEx, i.e. constraints like `[ @en ]` (issue #304)

## [v0.1.79] - 2025-06-30

- Internal refactor in SHACL validator to use SHACL Internal Representation with an independent representation from the `Rdf` trait which allows it to be applied to different implementations of the `Rdf` trait. 

## [v0.1.77] - 2025-06-24

- Added support for (min/max)(in/ex)clusive
- Repaired bug in minLength
- Solved typo in documentation

## [v0.1.72] - 2025-06-14

- Removed dependency on lazy_static!
- Added `shacl_rdf` and `shacl_ir` crates
- Created a folder `oxrdf_impl` that contains the implementations for the traits defined at the top level using the `oxrdf` library
- Renamed internal srdf traits and files. Some conventions, we will prepend `S` to the concrete structs or enums defined by SRDF, so instead of `Literal` we use `SLiteral`, keeping `Literal` for the trait name.
   - file `srdf_basic.rs` => `rdf.rs`
   - trait `Query` => `NeighsRDF`
   - trait `Sparql` => `QueryRDF`
   - trait `SRDFBuilder` => `BuildRDF`
   - struct `Literal` => `SLiteral`
   - struct `Triple` => `STriple`
   


## [v0.1.71] - 2025-05-28

- Disabled Xlsx support given the problem with Calamine in order to publish Python version of rudof

## [v0.1.70] - 2025-05-26

- Added implementation of ShEx validator that follows the [paper](https://labra.weso.es/publication/2017_semantics-validation-shapes-schemas/)
- There is [a problem](https://github.com/rudof-project/rudof/issues/291) with calamine's dependency from DCTAP which doesn't allow us to publish in crates.io. We are waiting for calamine to publish an official release because it seems the patch only works to build the system, but prevents us to publish to crates.

## [v0.1.65] - 2025-05-14

- Set reqwest dependency on rustls to disable openssl which gives several problems

## [v0.1.64] - 2025-05-14

- Added check on recursion with negative cycles in ShEx
- Added different result formats in ShEx like JSON

## [v0.1.63] - Skipped

## [v0.1.62] - 2025-03-29

- Changed dependency from [serde_yaml_ng](https://github.com/acatton/serde-yaml-ng) to [toml](https://docs.rs/toml/latest/toml/)
- Removed dependency in rbe_tests from serde_yaml_ng to use plain JSON for the test_suite

## [v0.1.60] - 2025-03-11

- Changed dependency from [serde_yml](https://doc.serdeyml.com/serde_yml/index.html) to [serde_yaml_ng](https://github.com/acatton/serde-yaml-ng) according to #278
- Changed Iri trait to add Ord constraint so IRIs can be ordered solving issue #276

## [v0.1.59] - 2025-01-01

- Fixes bug in feature added to solve issue #227 for local files which are relative that it didn't generate an absolute IRI. Now it does.
- Added option to SHACL2ShEx converter to optionally add `rdf:type` declaration for each `sh:targetClass` declaration. Previously, this behaviour was not optional and now it can be disabled.
- Fixes option to generate `rdf:type` for `sh:targetClass` declarations when there are more than one (previously it generated one rdf:type for each target class, and not it generates a value set).

## [v0.1.58] - 2024-12-31

- Solves issue #227 to automatically generate a base URL from the local file name or URL.

## [v0.1.57] - 2024-11-14

- Simple release to bump a new version that solve a issue with pyrudof in Google Colab

## [v0.1.56] - 2024-11-14

- Added `variables()` and `find` to QuerySolution class in pyrudof

## [v0.1.55] - 2024-11-14

- Added methods to show query solutions in rudof and pyrudof

## [v0.1.54] - 2024-11-13

- Added query to rudof and pyrudof

## [v0.1.53] - 2024-11-13

- Added serialization of RDF data from rudof and pyrudof

## [v0.1.52] - 2024-11-1

- Added `endpoints` to `RdfDataConfig` to contain a list of built-in endpoints
- Added prefixmap as a parameter to create `SRDFPARQL` endpoints
- Solved problem when asking information about a node in wikidata endpoint
- Added `config()` method to obtain `rudof` config
- Improved `add_endpoint()` in pyrudof to search for the list of built-in endpoints in RDFDataConfig

## [v0.1.51] - 2024-10-31

- Added `read_data_path` to `pyrudof`

## [v0.1.50] - 2024-10-31

- Fix: We repaired some export issues on UmlGenerationMode and the `__repr__` methods which were not properly generated.

## [v0.1.49] - 2024-10-30

- Implemented Display for ShapeMap, ShEx-schema and SHACL-schema
- Added `__repr__` to ShapeMap, ShExSchema and SHACLSchema
- Added `update_config` to rudof and pyrudof

## [v0.1.48] - 2024-10-29

- Minor release to force re-publication

## [v0.1.47] - 2024-10-29

- Changed the way that we represent enums in Python to use proper enums with default values
- Added `read_shacl_str` and `read_shacl_path` to pyrudof

## [v0.1.46] - 2024-10-29

- Added default values to `pyrudof` to allow a more flexible API
- minor release to include RDFFormat and ReaderMode in export list of `pyrudof`

## [v0.1.45] - 2024-10-29

- Changed the order of parameters in `read_shex_str`, `read_data_str` in `pyrudof`
- `RDFFormat` added in `pyrudof`
- `ReaderMode` added in `pyrudof`
- `reset_all` added in `pyrudof`

## [v0.1.44] - 2024-10-29

- `add_endpoint` added in `rudof_lib` and `pyrudof_lib`
- `reset_shacl` added in `rudof_lib` and `pyrudof_lib`

## [v0.1.43] - 2024-10-28

Minor release to add DCTAP for pyrudof

## [0.1.40] - 2024-10-28

- Added more features to the rudof_lib like the serialization of ShEx, SHACL and Shapemaps which is also mirrored in the Python bindings.
- Added shex2uml python bindings

## [0.1.37] - 2024-10-28

- Added more features to the rudof_lib like the serialization of ShEx, SHACL and Shapemaps which is also mirrored in the Python bindings

## [0.1.36] - 2024-10-27

- Python bindings based on rudof_lib to validate ShEx and SHACL

## [0.1.35] - 2024-10-25

- More refactoring on main to depend on rudof_lib for SHACL, issue #201
- Implemented Display for SHACL Validation report which shows the results with colors

## [0.1.34] - 2024-10-23

-Some refactoring on main to depend on rudof_lib and check if it works

## [0.1.33] - 2024-10-22

- Internal release to just change the README in rudof_lib

## [0.1.32] - 2024-10-22

- Created crate [`rudof_lib`](https://crates.io/crates/rudof_lib) which will act as the main library entry point for `rudof`. In the future, this crate could be called `rudof`.
- Refactor of main to invoke `rudof_lib`
- Added [`ResultShapeMap`](https://docs.rs/shapemap/latest/shapemap/result_shape_map/struct.ResultShapeMap.html) as the result of ShEx validation. One improvement is that now the results can appear with colors.

## [0.1.31] - 2024-10-20

- Added more information to docs
- Implemented more features of Service description
- Added Accept headers to `InputSpec` so it provides basic content negotiation
- Added ShExConfig to improve configuration of options that involve ShEx
- Added literals to shape maps
- Improved aesthetics of docs #170

## [0.1.30] - 2024-08-10

- Added support for imports #159
- Solved typo xslsx -> xlsx #176

## [0.1.29] - 2024-09-30

- Added option to use xlsx directly in tap2shex conversion
- Updated version of serde_yml to 0.0.12

## [0.1.28] - 2024-09-30

- First version that handles directly Excel spreadsheets in DCTAP. Issue #82
- Repaired small bug in DCTAP where headers with leading or trailing whitespaces where not properly parsed
- Unified dependencies on serde-yml #160
- Expose API to retrieve SHACL validation reports #164
- Fixed github action that was giving errors when publishing Python bindings #151

## [0.1.27] - 2024-09-25

- Added support for picklist values in DCTAP
- Added support for picklist values in DCTAP2ShEx
- Added support for simple value set values in ShEx to UML

## [0.1.26] - 2024-09-20

- Added support for SPARQL query options. New command called: `query`
- Added support for handling SPARQL service descriptions. New command called: `service`
- Changed the TAPConfig parameter of command `dctap` so it can use the same config file as option `tap2shex`

## [0.1.25] - 2024-09-11

- Small change removing an empty config file to solve issue #155

## [0.1.24] - 2024-09-10

- Added more configuration parameters for RDF data and Shacl data which allow, for example to define a default base IRI which can be used to resolve relative IRIs solving issue #149

## [0.1.23] - 2024-09-09

- Added option for partial views of UML class diagrams which can be useful when visualizing large ShEx schemas
- Improved the templates so they show metadata about the generation and a navigation bar
- Repaired a bug in the behaviour of force-overwrite which was appending to the file instead of overwriting its contents

## [0.1.22] - 2024-09-07

- Added the possibility to embed the SVG diagram in the HTML pages that are generated

## [0.1.21] - 2024-09-05

- Small release with a small improvement in the way we handle empty rows in DCTAP

## [0.1.20] - 2024-09-01

- Added option to get schemas from files, URIs or stdin (-) which was also implemented to data, solving issue #135
- Small release after moving the project to a standalone rudof-project organization

## [0.1.19] - 2024-08-30

- Added option to generate simple information about ShEx schemas
- Repaired bug in strict/lax reader mode that was not being taken into account

## [0.1.18] - 2024-08-28

- Added support for nquads and RDF/XML as input data formats
- Added more flexibility for NQuads parser to continue parsing in case of errors
- Added more flexibility of RDF parser to parse RDF lists in case there are more than one rdf:first predicate.

## [0.1.17] - 2024-08-28

- Repaired bug in DCTAP when a row has an empty shape_id and it was creating an empty shape instead of assuming the previous one
- Added support for first version of SHACL to ShEx converter

## [0.1.16] - 2024-08-22

- This release only changes the name of the python bindings from rudof to pyrudof and adds a first submodule convert for checking if it works

## [0.1.15] - 2024-08-19

- Solves issue #115 adding annotations to the ShEx compact printer
- Takes into account annotations to generate labels in HTML and UML conversion from ShEx

## [0.1.14] - 2024-08-14

- Added support for using URLs in command line. The system attempts to dereference the URI and parses its content.
- Added support for parsing placeholders in DCTAP generating new properties for each one
- Added support for extends in DCTAP

## [0.1.13] - 2024-08-13

- `data` option now serializes the RDF data to one of the existing RDF data formats (previous version were generating an internal representation of the graph).
- Added support for using `-` as a marker for stdin so `rudof` can be used in a Linux pipe

## [0.1.12] - 2024-08-13

- Changed the one line description of the commands according to issue #77
- First version which allows several RDF data files in the command line #100
- Repaired small bug in the validate option because two options had the same long name: mode

## [0.1.11] - 2024-08-12

- Repaired error #91 adding a force-overwrite option to the command line
- Changed command line name from `rdfsx` to `rudof`

## [0.1.6] - 2024-08-09

- Added more features to SHACL validation: #94
- Added more control about syntax highlighting on terminal:
  - Avoiding to include colors when the output goes to a file in ShEx generation options
- Added config parameter to some of the options in the Command line tool so the user can configure the behaviour: validate, convert, dctap, node

## [0.1.5] - 2024-07-30

- Added options in command line to pass config files in YAML
- Repaired bug in DCTAP resolution of IRIs

## [0.1.4] - 2024-07-28

- Added 2 separate options for shacl-validate and shex-validate, keeping the generic validate option
- Repaired bug on UML visualization that didn't show link names
- Added direct SVG/JPG generation from DCTAP files

## [0.1.3] - 2024-07-27

- Generation of HTML views from ShEx based on Minininja templates which allow better customization
- Direct conversion from DCTAP to UML and HTML views
- Generation of UML visualizations in SVG and PNG
- Basic support for SHACL validation and added shacl-validation crate

## [0.1.2] - 2024-07-17

- Added descriptions to subcommands in command line
- Added more options in DCTAP: property and shape labels, and value constraints
- Added direct conversion from DCTAP to HTML and UML
- More options for HTML views

## [0.1.1] - 2024-07-12

- Added basic support for generating HTML views from ShEx schemas, #60

## [0.1.0] - 2024-07-05

- Added fields: mandatory, repeatable, valueDatatype and valueShape to DCTAP
- Repaired spelling errors in README issue #73

## [0.0.15] - 2024-07-04

- First version with support for conversion from ShEx schemas to UML

## [0.0.14] - 2024-07-02

- First version with initial support for DCTap to ShEx converter, issue #54
- Refactor on shapes converter to accomodate more conversions each of them in its own folder
- First version which publishes also Python bindings

## [0.0.13] - 2024-06-22

- First version with initial support for ShEx to SPARQL converter, issue #67

## [0.0.12] - 2024-06-17

- Changed CLI name from `sx` to `rdfsx`
- First attempt to added basic support for DCTap
- Code cleaned with Rustfmt and Clippy by [MarcAntoine-Arnaud](https://github.com/MarcAntoine-Arnaud).

## [0.0.11] - 2024-06-08

- This version in mainly a maintainance version updating some dependencies
- Started project DCTAP to handle DCTAP files
- Updated some dependency versions
  - oxrdf = "0.2.0-alpha.2"
  - regex = "1.10.4"

## [0.0.10] - 2024-01-29

- [issue 32](https://github.com/rudof-project/rudof/issues/32) ShEx parser works as an iterator per statement allowing to show debug information by statement. Debug information can be controlled by the environment variablt RUST_LOG. A value of "debug" for that variable will print more information.
- Updated dependency versions
    oxrdf = "0.2.0-alpha.2"
    oxttl = "0.1.0-alpha.2"
    oxrdfio = "0.1.0-alpha.2"

## [0.0.9] - 2024-01-19

- Removed `shex_pest`, `shex_antlr` and `validation_oxgraph` folders because their code is no longer used.
- Added time option to `sx_cli`
- Repaired bug in `shex_compact` that failed with node constraints followed by cardinality without space
- More support to read SHACL as RDF
- Merged [srdf_graph](https://crates.io/crates/srdf_graph) and [srdf_sparql](https://crates.io/crates/srdf_sparql) crates into [srdf](https://crates.io/crates/srdf), the former crates will no longer be maintained as their code is integrated in `srdf`.
- Added option `--output` to CLI so the users can choose if the output goes to terminal or to a file
- Changed dependency from [rio_api](https://crates.io/crates/rio) and [rio_turtle](https://crates.io/crates/rio_turtle) to [oxttl](https://crates.io/crates/oxttl) and [oxrdfio](https://crates.io/crates/oxrdfio) which seem to be more actively maintained now.

## [0.0.7] - 2024-01-07

In this release we added support for SHACL by defining the [`shacl_ast`](https://crates.io/crates/shacl_ast) crate.

Other changes:

- Renamed the project from shex_rs to shapes_rs to indicate that the project intends to support both ShEx and SHACL.
- Merged the [srdf_graph](https://crates.io/crates/srdf_graph) and [srdf_sparql](https://crates.io/crates/srdf_sparql) crates into [srdf](https://crates.io/crates/srdf).
- Added more combinators and documentation examples to rdf_parser in order to document the RDF parser combinators approach. See, for example, the doc for the [map method](https://docs.rs/srdf/latest/srdf/srdf_parser/trait.RDFNodeParse.html#method.map).
