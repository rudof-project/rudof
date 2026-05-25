# Information about RDF nodes

`rudof` can be used to obtain information about nodes in an RDF graph.

For example, it is possible to show information about a an RDF node in a graph.
Given the following RDF graph:

```turtle
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:a :name       "Alice"                  ;
   :birthdate  "1990-05-02"^^xsd:date   ;
   :enrolledIn :cs101                   .

:b :name "Bob", "Robert" .

:cs101 :name "Computer Science" .
```

You can directly download the file with:

```sh
curl -o simple.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple.ttl
```

We can obtain information about the node `:a` (or any other node) by running the following command:

```sh
rudof node --node "<http://example.org/a>" simple.ttl
```

You can simplify the previous command using '-n' instead of '--node' and using the prefixed version of the URL, i.e. ':a' instead of the full URL as:

```sh
rudof node -n :a simple.ttl
```

## Obtaining information from URLs

Most of the commands that require a filename can also be used with dereferentiable URLs. In case the filename starts by `http://` or `https://`, `rudof` will try to get the contents of those URLs and process them.

In this way, the previous example could also be run as:

```
rudof node -n :a https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple.ttl
```

## Obtaining information from stdin

It is also possible to get the information directly from stdin by replacing the filename by a hyphen ('-') .

For example, if you type:

```
rudof node -n :a -
```

You can type the contents of an RDF file, followed by CTRL-D and rudof will process that as the input.

## IRI normalization modes

The `--node` value is parsed as a ShapeMap node selector, which normally requires IRIs to be enclosed in angle brackets (`<http://example.org/a>`). For convenience, `rudof` supports two modes that control how plain strings are treated.

### Lax mode (default)

In lax mode any string that contains `://` and is not already wrapped in `<>` is automatically wrapped before parsing. This means the following two commands are equivalent:

```sh
rudof node -n "http://example.org/a" simple.ttl
rudof node -n "<http://example.org/a>" simple.ttl
```

**Limitations.** The `://` heuristic is intentionally simple and will fail in some cases:

- **URNs** (`urn:isbn:0451450523`), **`mailto:`**, and **`data:`** URIs do not contain `://` and will *not* be auto-wrapped, causing a parse error. Use angle brackets explicitly for these schemes.
- A prefixed local name that happens to contain `://` (e.g. `ex:path//resource`) would be incorrectly treated as a full IRI.

See [IRI normalization internals](../internals/iri-normalization.md) for details and planned improvements.

### Strict mode

Pass `--strict-iris` to disable normalization entirely. Bare IRIs produce a clear parse error; angle brackets are always required. Use this in production pipelines or whenever non-`http` IRI schemes appear.

```sh
rudof node -n "<http://example.org/a>" simple.ttl --strict-iris
```
