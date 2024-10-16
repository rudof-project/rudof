# Scope of the project

Given that the command line tool requires a name, it seems reasonable to define the scope of the project, specially the command line tool, so we can decide which features we add to the tool and which features we left out of the project.

The project started as a library for ShEx (the repo was initially called `shex-rs`), but we soon realized that the scope should be bigger as we also wanted to implement SHACL, so we renamed the repository as `rudof`.

Recently, there was a need to support DCTAP and we included support for DCTAP also.

At this moment, the command line tool can also be used to do some tasks related with RDF like obtaining information about RDF data, SPARQL endpoints and the neighbourhood of nodes in RDF data and SPARQL endpoints, and we are considering to add another option to query data from SPARQL endpoints, which can be useful when defining SHACL shapes and for RDF data processing in general.

So the scope of the project is to be a useful tool for people interested in:

- RDF data (which can be obtained from files as well as from SPARQL endpoints)
- RDF data models, shapes or schemas, which can be represented as ShEx, SHACL, DCTap, etc.
- Tasks related with the previous two like:
  - Obtaining information about RDF data
  - Validating RDF data
  - Validating RDF data models
  - Converting between different data, data formats and data models.

## Out of scope

There are some topics which are related with this project and they are currently out of scope like:

- Different types of data, like XML or JSON
- Other types of schemas like XML Schema or JSON Schema
- Ontologies and OWL reasoning
- Rules and inference
- RDF triple stores ([Oxigraph](https://github.com/oxigraph/oxigraph) already gives support for that in Rust)
- ???

In the future, we may reconsider this decision and include support for some of those topics, for example, it would be nice to combine validation and inference, either performing inference on the RDF data before validation, after validation or during the validation.
