# convert: RDF Data model conversions

`rudof` supports conversion between different RDF Data modeling technologies using the `convert` command.
At this moment, we have implemented some converters listed in the table below.

| From  | To   |
|-------|------|
| DCTAP | ShEx |
| DCTAP | UML  |
| DCTAP | HTML |
| SHACL | ShEx |
| ShEx  | UML  |
| ShEx  | HTML |

The `convert` command requires 7 main arguments; namely:

- `--input-mode` (`-m` for short), where the user introduces the input technology.
- `--export-mode` (`-x` for short), where the user introduces the output technology.
- `--format` (`-f` for short), where the user defines the input file format (serialization-wise).
- `--result-format` (`-r` for short), where the user defines the output file format (serialization-wise).
- `--source-file` (`-s` for short), where the user passes the input file path.
- `--output-file` (`-o` for short), where the user passes the output file path.
- `--config` (`-c` for short), where the user passes the config file path.

> Note that there's a difference between **mode** and **format**. While the first allows users to select the RDF technology; namely, DCTAP, SHACL or ShEx, the second allows users to define the actual serialization format employed; e.g. ShEx can be serialized using compact syntax (ShExC) or JSON (ShExJ).

## Prerequisites

In case you want to generate the UML diagrams, [PlantUML](https://plantuml.com/) needs to be installed, as the generation of SVG or PNG images is based on it.
So you must download the [command line JAR file](https://plantuml.com/download).
Once downloaded, set the environment variable `PLANTUML` to point to that file.

PlantUML also requires [Java](https://www.oracle.com/java/technologies) 8 or higher to be installed. You can check if you Java is already installed by running the following command.

```sh
java --version
```

For sucessfully following the examples provided, download the files from the Github repository.

```sh
curl -o book.csv https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/dctap/book.csv
curl -o simple_shacl.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple_shacl.ttl
curl -o simple.shex https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple.shex
```

## Configuration files

Configuration files can be used to pass additional parameters to the conversion process.
In the Github repository, an example configuration file is provided.
Note that those are TOML files.

```sh
curl -o config.toml https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/dctap/book_converter_config.toml
```

Whose contents are described below.

```toml
[tap2shex]
base_iri = "http://example.org/"

[tap2shex.prefixmap]
dct = "http://purl.org/dc/terms/"
rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#"
foaf = "http://xmlns.com/foaf/0.1/"
xsd = "http://www.w3.org/2001/XMLSchema#"
sdo = "https://schema.org/"
ex = "http://example.org/"
```

## From DCTAP

[DCTAP](https://www.dublincore.org/specifications/dctap/) is an RDF data model technology developed at the [Dublin Core Application Profiles Working Group](https://github.com/dcmi/dctap) which aims for providing a way to represent application profiles in the form of tables.

### From DCTAP to ShEx

It is possible to convert from a DCTap (CSV) to ShEx.
In the first example, the most simple DCTap to ShEx schema conversion is described.

```sh
rudof convert -m dctap -s book.csv -f csv -x shex
```

However, the converter contains a parameter that can be used to add configuration information (`--config`).
For example, instead of the basic *prefix map*, we can use custom *prefix map* declarations as follows.
Refer to the [Configuration files](#configuration-files) section.

```sh
rudof convert -s book.csv -m dctap -x shex -f csv -c config.toml
```

### From DCTAP to UML

Conversion for DCTap (CSV) to UML-like visualizations is possible.
In fact, it is possible to serialize the resulting diagram in two possible formats, namely, `svg` and `png`.
For indicating `rudof` the desired format use the `--result-format` (`-r` for short) argument.

To generate `svg` visualizations it can be done as follows:

```sh
rudof convert -s book.csv -m dctap -x uml -f csv -r svg -o simple.svg
```

To generate `png` visualizations it can be done as follows:

```sh
rudof convert -s book.csv -m dctap -x uml -f csv -r png -o simple.png
```

### From DCTAP to HTML

TBD

## From SHACL

### From SHACL to ShEx

It is possible to convert between a subset of SHACL shapes to ShEx schemas.
As an example, the following command:

```sh
rudof convert -m shacl -x shex -s simple_shacl.ttl -f turtle -o simple.shex
```

> The converter only works for a subset of SHACL. This is a work-in-progress feature and [the following issue](https://github.com/rudof-project/rudof/issues/127) is expected to document the subset of SHACL supported.

### Specifying base IRI to handle relative IRIs

If the RDF data contains relative IRIs, it is necessary to resolve them by specifying a base IRI. It can be done by providing a base declaration in the `rdf_data` entry of the configuration file. For example, assuming the configuration file contains `examples/config/data_config.toml`

```toml
[rdf_data]
base = "http://example.org/"
```

it is possible to specify the conversion as:

```sh
rudof convert -m shacl -x shex -s simple_shacl.ttl -f turtle -o simple.shex -c examples/config/data_config.toml
```

## From ShEx

### From ShEx to UML

As in the case of DCTap, it is possible to convert from a simple ShEx schema to UML like visualization in `svg` and `png`.
To generate `svg` visualizations it can be done as follows:

```sh
rudof convert -s simple.shex -m shex -x uml -r svg -o simple.svg
```

To generate `png` visualizations it can be done as follows:

```sh
rudof convert -s simple.shex -m shex -x uml -r png -o simple.png
```

### From ShEx to HTML

It is possible to convert from ShEx schema to a set of HTML pages representing the schema.
The content of the HTML pages can be customized using [Jinja](https://docs.rs/minijinja/latest/minijinja/index.html) templates. The generated pages will be stored in an output folder that must be specified with the `--target` option. 

```sh
rudof convert -s simple.shex -m shex -x html -t output-folder -e templates-folder
```

> The HTML pages that are generated can be highly configured as `rudof`'s approach is based on templates. 
Thus, it takes a set of [default templates](https://github.com/rudof-project/rudof/tree/master/shapes_converter/default_templates) 
which define the appearance of the resulting HTML. However, it is possible to use customized templates based on the 
[minininja](https://docs.rs/minijinja/latest/minijinja/index.html) template engine.
