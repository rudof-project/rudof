# shacl-validate: Validating RDF data using SHACL

RDF data validation in SHACL is a key step for ensuring data quality (correctness and completeness) of a dataset.
We will make use of the [UserShape example in SHACL](https://book.validatingrdf.com/bookHtml011.html#ch050SHACLExample) from the [Validating RDF Data](https://book.validatingrdf.com/) book to demonstrate tha capabilities of the SHACL validator we propose.
For following the examples please download the following [file](https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple_shacl.ttl) from the Github repository.

```sh
curl -o shapes.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/book.ttl
curl -o data.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/book_conformant.ttl
```

The graph from the example is a simple one but will be sufficient for the purposes of this page.
Below you can see the contents of it.

```turtle
:alice a :User;                             #Passes as a :UserShape
       schema:name           "Alice" ;
       schema:gender         schema:Female ;
       schema:knows          :bob .

:bob   a :User;                             #Passes as a :UserShape
       schema:gender         schema:Male ;
       schema:name           "Robert";
       schema:birthDate      "1980-03-10"^^xsd:date .

:carol a :User;                             #Passes as a :UserShape
       schema:name           "Carol" ;
       schema:gender         schema:Female ;
       foaf:name             "Carol" .
```

> For SHACL validation, we can use the generic `validate` command and the specific `shacl-validate` command. The key difference is that the latter is less verbose, as it does not require the `--mode` argument to be specified.

## Using the generic `validate` command

In case you want to use the generic `validate` command, you need to specify the `--mode` argument and a `--schema`.
Refer to the [Example in the book](https://book.validatingrdf.com/bookHtml011.html#ch050GoodRDFGraph) for further details on validation.

> Note that the data graph is conforming against the shapes. And as such, a conforming Validation Report; e.g a Report with no Validation Results, is going to be generated.

```sh
rudof validate -M shacl -f turtle --schema shapes.ttl data.ttl
```

## Using the specific `shacl-validate` command

In case you want to use the specific `validate-validate` command, you need to specify the `--shapes`.
This is because of the naming conventions in the SHACL Recommendation, where schemas (ShEx) are called shapes (SHACL).

> Expect the same result as in the previous case.

```sh
rudof shacl-validate --shapes shapes.ttl data.ttl
```

## Non-conforming datasets

In case you want to try a non-conforming dataset, you can always download the one that is provided in the examples.

```sh
curl -o non-conformant.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/book_non-conformant.ttl
```

Which is the simple graph below.

```Turtle
:dave  a :User ;                        #Fails as a :UserShape
       schema:name       "Dave";
       schema:gender     :Unknown ;
       schema:birthDate  1980 ;
       schema:knows      :grace .

:emily a :User ;                        #Fails as a :UserShape
       schema:name       "Emily", "Emilee";
       schema:gender     schema:Female .

:frank a :User ;                        #Fails as a :UserShape
       foaf:name         "Frank" ;
       schema:gender     schema:Male .

_:x    a :User;                         #Fails as a :UserShape
       schema:name       "Unknown" ;
       schema:gender     schema:Male ;
       schema:knows      _:x .
```

We can perform the validation of the aforementioned graph against the same shape as in the previous examples using the specific `shacl-validate` command.
Refer to the [Example in the book](https://book.validatingrdf.com/bookHtml011.html#ch050SHACLBadDataGraph) for further details on validation.

> Expect a Report containing 6 different results. One per fault that was found by the SHACL processor.

```sh
rudof shacl-validate --shapes shapes.ttl non-conformant.ttl
```

## Controlling what the report contains

By default, a validation report lists every violation but no explanation of *why* the
conforming nodes conform. Two flags change that:

```sh
rudof shacl-validate --shapes shapes.ttl data.ttl --no-errors
rudof shacl-validate --shapes shapes.ttl data.ttl --with-evidences
```

- `--no-errors` drops the per-violation detail from the report, keeping only the overall
  conforms/does-not-conform verdict. Useful when you only need the boolean and want a
  smaller report.
- `--with-evidences` adds the opposite information: for every `(node, shape)` pair that
  *does* conform, a record of why. Off by default, since most validation runs only care
  about failures.

The two are independent, so all four combinations are available (default: errors only). In
the `-r details`/`-r compact` table output, evidence rows are marked **`Conforms`** in green
(rather than a severity) and sit alongside the violation rows, so a report with
`--with-evidences` tells the full story — what failed and what didn't — in one table.

With `--with-evidences`, evidence is recorded at two granularities: one entry per constraint
a node satisfies (`sh:datatype`, `sh:minCount`, ...) *and* one summary entry per shape it
conforms to as a whole (e.g. "conforms to `:PersonShape`"). That's a lot of rows for a shape
with several constraints — `--evidences-shapes-only` keeps just the per-shape summaries:

```sh
rudof shacl-validate --shapes shapes.ttl data.ttl --with-evidences --evidences-shapes-only
```

It has no effect without `--with-evidences`, and never affects violations.

These keys can be set persistently in `rudof.toml` — see [`[shacl]`](../references/config.md#shacl--shacl-validation) in the config reference.

## Which shape caused a violation

Each validation result names, in `sh:sourceShape`, the shape whose constraint failed. That
is the shape that declares the constraint, not the shape it refers to: if a property shape
of `:S` says `sh:node :T` and a value doesn't conform to `:T`, the result's source shape is
that property shape (a blank node, unless you give it an IRI), as the SHACL specification
requires.

To see *why* the value doesn't conform to `:T`, the results that made it fail are attached
to the result as `sh:detail`, each with its own focus node and source shape. rudof does this
for `sh:node`, `sh:and`, `sh:or` and `sh:xone`. For example, with these shapes:

```turtle
:S a sh:NodeShape ;
   sh:targetNode :a ;
   sh:property [ sh:path :p ; sh:node :T ] .

:T a sh:NodeShape ;
   sh:property [ sh:path :p ; sh:hasValue :c ] .
```

and the data `:a :p :b . :b :p :a .`, the report (`-r turtle`) contains:

```turtle
[ a sh:ValidationResult ;
  sh:focusNode :a ;
  sh:resultPath :p ;
  sh:value :b ;
  sh:sourceConstraintComponent sh:NodeConstraintComponent ;
  sh:sourceShape _:b2 ;                 # the property shape of :S
  sh:detail [
    a sh:ValidationResult ;
    sh:focusNode :b ;
    sh:resultPath :p ;
    sh:sourceConstraintComponent sh:HasValueConstraintComponent ;
    sh:sourceShape _:b4                 # the property shape of :T
  ]
] .
```

In the default `-r details` table, the nested results are listed under "Caused by
(sh:detail)" in the Details column. Nested results can themselves have details, for chains
of `sh:node` references.

## Recursive shapes

A shape may reference itself, directly or through other shapes — for example, a `Person`
shape whose `knows` property must itself point to a `Person`. Validating such a shape
against data that actually contains a cycle (`:alice knows :bob`, `:bob knows :alice`)
needs a rule for what happens when validation comes back around to a node it's already in
the middle of checking. `--recursion-semantics` picks that rule:

```sh
rudof shacl-validate --shapes recursive-shapes.ttl data.ttl --recursion-semantics lfp
rudof shacl-validate --shapes recursive-shapes.ttl data.ttl --recursion-semantics gfp
rudof shacl-validate --shapes recursive-shapes.ttl data.ttl --recursion-semantics none
```

- `lfp` (least fixed point, the default) assumes a node caught in a cycle does **not** conform unless
  that can be shown without relying on the cycle. A cycle with no independent way to
  ground it — like `:alice`/`:bob` above, if nothing else establishes either of them as a
  `Person` — ends up **not** conforming.
- `gfp` (greatest fixed point) assumes a node caught in a cycle **does** conform, as long as that assumption
  doesn't contradict anything else in the shape. The same cycle then **conforms**.
- `none` rejects the shapes graph outright, as soon as it's loaded, before any data is
  even checked:

  ```sh
  ❯ rudof shacl-validate --shapes recursive-shapes.ttl data.ttl --recursion-semantics none
  Error: SHACL error: ... Dependency graph has cycles: ...
  ```

A shape with no cycles in it validates identically under all three, so there's no harm in
leaving `--recursion-semantics` unset unless you actually have a recursive shape.

`cautious` and `brave`, the names these modes had up to 0.3.26, are still accepted as
aliases of `lfp` and `gfp`.

### Recursion and negation

A cycle built only from monotonic constraints (`sh:and`, `sh:or`, `sh:node`, `sh:property`,
`sh:minCount`, `sh:closed`, and similar) is always safe under `lfp`/`gfp`. A cycle
that also carries a negating constraint (`sh:not`, `sh:xone`, `sh:qualifiedMaxCount`,
`sh:qualifiedValueShapesDisjoint`) needs one more condition, *stratification*: every negating
constraint in the cycle must target a shape that doesn't itself depend on any recursion —
directly or transitively. Such a shape can always be resolved on its own, independently of
the cycle it's negated from, so there's no ordering problem. A schema like this:

```turtle
:PersonShape a sh:NodeShape ;
    sh:targetClass :Person ;
    sh:not :RobotShape ;                       # RobotShape is unrelated to the recursion
    sh:property [ sh:path :knows ; sh:node :PersonShape ] .

:RobotShape a sh:NodeShape ;
    sh:property [ sh:path :isRobot ; sh:hasValue true ] .
```

compiles and validates normally under both `lfp` and `gfp`: `sh:not :RobotShape` is
checked exactly as it would be outside any cycle, and the recursive `:knows` property is
still resolved per the chosen semantics.

What's still rejected, under every `--recursion-semantics` value including `gfp`, is a
negating constraint that reaches back into a cycle — its own, or a different recursive
shape's. There's no order left to evaluate it in: whichever fixpoint gets picked for the
negated shape, the negation's own meaning stops being simple to define. Use `rudof shacl -s
shapes.ttl -r internal` to see how each shape in a schema is classified — *not recursive*,
*positive recursive*, *stratified recursive*, or *non-stratified recursive* — including which
ones would need this to be resolved.

## Selecting the RDF backend

By default, validation data is loaded into an in-process `memory` graph. Use `--backend` to switch to a QLever Docker container or a remote SPARQL endpoint:

```sh
rudof shacl-validate --shapes shapes.ttl --backend qlever data.ttl
rudof shacl-validate --shapes shapes.ttl --endpoint https://my.sparql.server/sparql
```

See the [RDF backend (`--backend`) reference](./backend.md) for full documentation.

### Validating against a SPARQL endpoint

With an endpoint, rudof reads only the data the validation needs, through a cache, so that
no lookup is sent twice. (Data in memory is first indexed by class, but over an endpoint that
would ask for every triple, so it is skipped.) `--fetch-strategy` selects how the data is
fetched:

- `on-demand` (the default) fetches the data of each focus node when the validation needs
  it: about one request per focus node and path. Use it to validate a few nodes.
- `by-shape` fetches the data shape by shape before the validation starts, following the
  approach of Corman, Florenzano, Reutter and Savković, [*Validating SHACL constraints over a
  SPARQL endpoint*](https://jreutter.ing.uc.cl/SHACL_19.pdf) (ISWC 2019).
  rudof starts from the shapes with targets and moves to the shapes they reference, breadth
  first. For each shape, one query fetches the data of up to 50 nodes, so the number of
  queries depends on the shapes and not on the number of nodes. Use it to validate many
  nodes.

```sh
rudof shacl-validate -s shapes.ttl -e wikidata --fetch-strategy by-shape
```

The bindings take the same option: `rudof.validate_shacl(fetch_strategy=ShaclFetchStrategy.ByShape)`
in Python, and `await rudof.validateShaclAsync("native", "by-shape")` in WebAssembly.

In a test with 40 focus nodes and a recursive shape, `on-demand` sent 124 queries and
`by-shape` sent 6. Both give the same report as validating the same data in memory.

`by-shape` differs from the paper in two ways:

- Each query is restricted, with `VALUES`, to the nodes the validation can reach: the
  targets, then the values of their paths, and so on. The paper's queries range over the whole
  graph, which is impractical on an endpoint the size of Wikidata.
- rudof's own engine validates the fetched data in memory, instead of the paper's rule
  saturation. It applies the same semantics for recursive shapes (see `--recursion-semantics`),
  supports every constraint component, and gives the full report. Data the prefetch did not
  anticipate (for example deeper than `prefetch_max_rounds` shape references, or read by a
  SPARQL-based constraint) is fetched on demand, so the result does not change.

The targets themselves are still found with one query per target declaration: for
`sh:targetClass`, all the instances of the class. On a large endpoint, prefer `sh:targetNode`
or a narrow class. The `[shacl]` keys `fetch_strategy`, `prefetch_batch_size` and
`prefetch_max_rounds` set the same options in a config file (see the
[config reference](../references/config.md#shacl--shacl-validation)).
