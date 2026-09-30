# ShExMap examples

Inputs for `rudof shexmap` and for the tests in `shex_ast/tests/shexmap_examples.rs`.
Each directory holds a `manifest.json` in the format of shex.js's extension-map test
runner: an input schema (`schemaURL` or inline `schema`), input data, a `queryMap`
(`<focus>@<shape>` or `@START`), an output schema, an `outputShapeMap` (`<root>@<shape>`),
the expected output graph and, where shex.js recorded them, the expected bindings JSON.

* `shexjs/` -- the examples of shex.js's `@shexjs/extension-map` package: blood pressure
  between a FHIR-like and a flat schema, in both directions; a patient with several
  readings (one list of iterations); two-level nesting; a symmetric schema; a `OneOf` that
  splits contacts into phones and mailboxes; an ambiguous card.
* `pyshex/` -- PyShEx's additions: an ambiguous reading (two parses, two outputs), coded
  components, a greedy trap the partition search must avoid, inverse constraints in the
  input and in the output, and `EXTENDS` in both schemas.  See its `README.md`.

Relative IRIs in the schemas resolve against `http://a.example/schema/`, and in the data
against `http://a.example/turtle/`, as in shex.js's runner; pass `--base-schema` and
`--base-data` accordingly, for instance:

```sh
cd examples/shexmap/shexjs
rudof shexmap BPfhir-instance.ttl -s BPfhir-schema.shex -n '<tag:BPfhir123>' \
  -O BPdam-schema.shex --root '<tag:b0>' --output-shape BPunitsDAM \
  --base-schema http://a.example/schema/
```

The bindings JSON (`*-bindings.json`) is shared with shex.js and PyShEx: a scope is an
object of variable IRIs to terms, or an array `[object, list, ...]` with one list per
repeated constraint or group, each list holding one scope per iteration, and `"@node"` on
the iterations of repeated shape-valued constraints naming the input node they matched.
