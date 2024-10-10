# Data model conversion

`rudof` supports conversion between different RDF Data modeling technologies.
At this moment, we have implemented some converters listed in the table below.

| From  | To   |
|-------|------|
| DCTAP | ShEx |
| DCTAP | UML  |
| DCTAP | HTML |
| SHACL | ShEx |
| ShEx  | UML  |
| ShEx  | HTML |

## Prerequisites

In case you want to generate the UML diagrams, [PlantUML](https://plantuml.com/) needs to be installed, as the generation of SVG or PNG images is based on it.
So you must download the [command line JAR file](https://plantuml.com/download).
Once downloaded, set the environment variable `PLANTUML` to point to that file.

PlantUML also requires [Java](https://www.oracle.com/java/technologies) 8 or higher to be installed. You can check if you Java is already installed by running the following command.

```sh
java --version
```

## DCTAP → ShEx

It is possible to convert a DCTap file in CSV to ShEx.

For example, the `user.csv` file in [`examples/user.csv`](https://github.com/rudof-project/rudof/blob/master/examples/user.csv) can be converted to a ShEx schema running:

```sh
rudof convert -m dctap -s examples/user.csv -f csv -x shex
```

The converter contains a parameter that can be used to add configuration information.

For example, instead of the basic prefix map, we can use custom prefix map declarations as follows.

```sh
rudof convert -s examples/dctap/book.csv -m dctap -x shex -f csv -c examples/dctap/book_converter_config.yml
```

Where the contents of `book_converter_config.yml` are:

```yaml
tap2shex:
  base_iri: "http://example.org/"
  prefixmap:
    dct: "http://purl.org/dc/terms/"
    rdf: "http://www.w3.org/1999/02/22-rdf-syntax-ns#"
    foaf: "http://xmlns.com/foaf/0.1/"
    xsd: "http://www.w3.org/2001/XMLSchema#"
    sdo: "https://schema.org/"
    ex: "http://example.org/"
```

## DCTAP → UML

Convert a CSV file in DCTap to an UML-like visualization in SVG

```sh
rudof convert -s examples/simple.csv -m dctap -x uml -f csv -r svg -o target/simple.svg
```

To generate PNG, replace `svg` by `png`.

```sh
rudof convert -s examples/simple.csv -m dctap -x uml -f csv -r png -o target/simple.png
```

## DCTAP → HTML

TBD

## SHACL → ShEx

Convert a simple SHACL shapes graph to ShEx

It is possible to convert a SHACL shapes graphs to ShEx schemas.  

```sh
rudof convert -m shacl -x shex -s examples/simple_shacl.ttl -f turtle -o target/simple.shex
```

### Limitations

The converter only works for a subset of SHACL. We should still document what are the features supported and the features that are not yet supported but this is still work in progress.

## ShEx → UML

It is possible to convert a simple ShEx schema to a UML like visualization in SVG, PNG

```sh
rudof convert -s examples/simple.shex -m shex -x uml -r svg -o target/simple.svg
```

To generate PNG, replace `svg` by `png`.

```sh
rudof convert -s examples/simple.shex -m shex -x uml -r png -o target/simple.png
```

## ShEx → HTML

It is possible to convert a ShEx schema to a set of HTML pages that represent the schema. The content of the HTML pages can be customized using Jinja templates.

```sh
rudof convert -s examples/simple.shex -m shex -x html -t target/simple
```

The HTML pages that are generated can be highly configured because the approach that `rudof` follows is based on templates.
It takes a set of [default templates](https://github.com/rudof-project/rudof/tree/master/shapes_converter/default_templates) which define the appearance of the HTML result but it is possible to pass a different set of templates.
The templates are based on the [minininja](https://docs.rs/minijinja/latest/minijinja/index.html) template engine.
