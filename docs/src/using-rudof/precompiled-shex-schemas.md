# Precompiled ShEx schemas

`rudof` can compile a ShEx schema down to its internal representation
(`SchemaIR`) once and reuse that artefact across many validations
without paying the parsing and compilation cost every time. This guide
explains what the precompiled cache is, when it is worth using, and
how to drive it from the CLI.

## Why precompile a schema?

Every `rudof shex-validate` invocation does two things before it can
touch a single triple of data:

1. Parses the ShEx source (ShExC, ShExJ, Turtle, …) into an AST.
2. Compiles the AST into `SchemaIR`, the shape of the schema that the
   validator actually runs against.

For small schemas both steps are essentially free. For larger schemas
(hundreds of shapes, many imports, deep inheritance) the compilation
step is expensive, and it is *exactly the same work* every time you validate the same
schema. The precompiled cache lets you do that work once and skip it
on every subsequent run.

## The compile - validate loop

### 1. Compile the schema

Use the `shex` command with `--compile-to`:

```sh
rudof shex --schema examples/user.shex --compile-to user.ircache
```

`rudof` loads the schema exactly as it would for validation
and writes the resulting `SchemaIR` to `user.ircache`. The file is a pure
binary artefact: it starts with the magic bytes `RSIR`, followed by an on-disk
envelope version, a length-prefixed [bincode](https://docs.rs/bincode)
header (body format, `rudof` version, negation-cycle flag), and the
bincode-encoded `SchemaIR` body. Because the file is not text, do not open it in
editors that may rewrite line endings or normalise encoding.

You can also compile as a side-effect of a validation run:

```sh
rudof shex-validate \
  --schema examples/user.shex \
  --compile-to user.ircache \
  --shapemap examples/user.sm \
  examples/user.ttl
```

### 2. Validate against the cache

Point `shex-validate` at the cache with `--compiled-schema` instead
of `--schema`:

```sh
rudof shex-validate \
  --compiled-schema user.ircache \
  --shapemap examples/user.sm \
  examples/user.ttl
```

## Other Considerations

- **The cache is bound to a version.** Loading a cache produced by an incompatible
  version fails with an error message.
- **Strict-mode compiles are already verified.** If the schema was
  compiled under `--reader-mode strict`, the negation-cycle check
  ran at compile time and its result is recorded in the header, so
  loading under strict mode does not repeat the check.
- **No source-schema flags.** `--compiled-schema` conflicts with
  `--schema`, `--schema-format`, and `--base-schema`. The cache
  already carries a fully-resolved schema; there is nothing left for
  those flags to do.
- **No external-shape resolvers.** `--compiled-schema` conflicts
  with `--external-resolver`. Some `EXTERNAL` shape declarations are
  substituted during AST to IR compilation, so a resolver passed at
  validation time would arrive too late. If your schema uses
  `EXTERNAL` shapes, register the resolvers at compile time.
- **Semantic actions.** The cache does not embed semantic-action
  implementations. If your schema references a `SemAct` IRI, the
  extension that handles that IRI must be registered in the running
  `rudof` process when the cache is loaded.


## Full end-to-end example

Starting from the [`examples/`](https://github.com/rudof-project/rudof/tree/master/examples)
folder in the repo:

```sh
# 1. Compile the schema once.
rudof shex --schema examples/user.shex --compile-to user.ircache

# 2. Validate any number of data sets against the same cache.
rudof shex-validate \
  --compiled-schema user.ircache \
  --shapemap examples/user.sm \
  examples/user.ttl

rudof shex-validate \
  --compiled-schema user.ircache \
  --shapemap examples/user.sm \
  examples/user_fail.ttl
```

## Related reference pages

- [`shex` command](../cli_usage/shex.md)
- [`shex-validate` command](../cli_usage/shex_validate.md)
