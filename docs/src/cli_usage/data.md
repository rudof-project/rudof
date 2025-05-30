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

## RDF Config file

The parameter `--config-file`  (`-c` in short form) can be used to pass a configuration file in [TOML](https://toml.io/) format.

The fields that it can contain are:

- base (IRI): Default base declaration to resulve relative IRIs
