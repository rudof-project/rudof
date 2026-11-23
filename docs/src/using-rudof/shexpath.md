# ShExPath

[ShExPath](https://shexspec.github.io/spec/ShExPath) is a path language that addresses the
elements of a ShEx schema: shape expressions, triple constraints, node constraints and their
value sets, semantic actions and annotations. It addresses a schema, not a graph. What a path
selects is a list of schema elements, which an application can use to report a validation result
against one, to attach information to one, or simply to find one in a large schema.

`rudof` implements the language as the specification defines it, from the command line, the Rust
library, Python and JavaScript.

## The model

A path is evaluated over the ShExJ form of the schema, the JSON document that ShExJ writes down.
An *item* is any node of that document:

- an object: the schema, a `ShapeDecl`, a shape expression, a triple expression, a `SemAct`, an
  `Annotation`, a value set value, ...;
- an array: `shapes`, `expressions`, `extends`, ... An array is an item in its own right, so
  `extends` selects the array and `extends/*` its members;
- an atomic value: an IRI, a string, a number or a boolean. A reference to a shape is an IRI, not
  the shape it names.

`rudof` reports each selected item with its *location*, a
[JSON Pointer](https://www.rfc-editor.org/rfc/rfc6901) into the ShExJ document, such as
`/shapes/0/shapeExpr/expression/expressions/1`, and its *value*, the item as ShExJ. The items come
in the order the path selects them, with their duplicates.

## The language at a glance

The examples use the example schema of the specification:

```shex
PREFIX : <http://ex.example/ns#>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
BASE <http://ex.example/schema>

<#IssueShape> {
  :name xsd:string MINLENGTH 4 ;
  :category ["bug" "feature request"] ;
  $<#reportedBy> :postedBy @<#UserShape> ;
  :processing {
    :reproduced [true false] ;
    :priority xsd:integer
  } ?
}

<#UserShape> IRI /User\?id=[0-9]+/ {
  (  foaf:name xsd:string
   |  foaf:givenName xsd:string + ;
      foaf:familyName xsd:string
  ) ;
  foaf:mbox IRI
}

<#AgentShape> { foaf:mbox IRI }

<#EmployeeShape> EXTENDS @<#UserShape> EXTENDS @<#AgentShape> {
  :employeeId xsd:integer
}
```

| ShExPath | Selects |
|---|---|
| `@<#IssueShape>` | The shape expression declared as `#IssueShape`. |
| `@<#IssueShape>~:category` | The `:category` triple constraint in it. |
| `$<#reportedBy>` | The `:postedBy` triple constraint, by the label given to it. |
| `$<#reportedBy>/valueExpr` | The reference to `#UserShape` in it: an IRI. |
| `@<#UserShape>~foaf:mbox` | The `foaf:mbox` constraint, through the `ShapeAnd` that `IRI /re/ {…}` is. |
| `/shapes/*` | The four shape declarations. |
| `@<#IssueShape>/expression/expressions/*[1]` | The second constraint of `#IssueShape`, `:category`. |
| `@<#IssueShape>/expression/expressions/*[predicate = :name]` | The `:name` constraint. |
| `@<#IssueShape>/expression/expressions/*[assert count() = 4]` | The four constraints, and an error if there are not four. |
| `@<#IssueShape>~:category/ancestor::Shape` | The shape that constraint is in. |
| `@<#EmployeeShape>/extends/*[0]` | The reference to `#UserShape` in the `EXTENDS` list. |
| `@<#IssueShape>~:name , @<#IssueShape>~:category` | Both constraints. |

The parts of a path:

- **Steps** are separated by `/` and applied left to right, each to every item the previous one
  selected. A child step names a ShExJ property (`shapes`, `expression`, `valueExpr`, ...) or is
  `*`, every property value of an object or member of an array.
- **Axes**: `self::`, `parent::`, `ancestor::`, `thisShapeExpr::` (the item and the shape
  expressions a `ShapeAnd`, `ShapeOr` or `ShapeNot` combines) and `thisTripleExpr::` (the item
  and the triple expressions an `EachOf` or `OneOf` groups, inclusions among them).
- **Term types** keep the items of a ShExJ type: `Shape`, `TripleConstraint`, `NodeConstraint`, ...
- **Filters** in brackets: `[N]` (the item at position `N`, from 0), `[index() = N]`,
  `[count() = N]`, `[path]`, `[path = value]` and `[assert ...]`.
- **Shortcuts**: `@label` (a shape expression), `~iri` (the triple constraints on a predicate in
  this shape) and `$label` (a labelled triple expression).
- **Junctions**: `,` (every item of each operand, keeping duplicates), `union` (without
  duplicates) and `intersection`.

The IRIs in a path are written with the prefixes and base of the schema: `:category` and
`<#IssueShape>` above resolve as they do in the schema.

## Using ShExPath

### From the command line

The [`shex-path`](../cli_usage/shex_path.md) command prints what a path selects as JSON:

```sh
rudof shex-path -s issue.shex -p "@<#IssueShape>~:category"
```

### From Rust

`Rudof::evaluate_shex_path` evaluates a path against the loaded schema and returns a list of
`ShExPathMatch`, each with its `location` and `value`:

```rust,ignore
let matches = rudof.evaluate_shex_path("@<#IssueShape>~:category").execute()?;
for m in matches {
    println!("{} {}", m.location, m.value);
}
```

The language itself is in `shex_ast::shexpath`: `ShExPathParser` reads a path, and
`ShExJDocument` evaluates it over a schema, also from items other than the schema, for relative
paths.

### From Python

```python
from pyrudof import Rudof

with Rudof() as rudof:
    rudof.read_shex(schema)
    for match in rudof.evaluate_shex_path("@<#IssueShape>~:category"):
        print(match["location"], match["value"]["predicate"])
```

### From JavaScript

```js
const rudof = new Rudof();
rudof.readShex(schema);
for (const { location, value } of rudof.evaluateShexPath("@<#IssueShape>~:category")) {
  console.log(location, value.predicate);
}
```

In every case a path that is not a ShExPath, or an `assert` that does not hold, is an error
(`ShExError` in Python and JavaScript).

## Points the specification leaves open

The specification marks some points as unsettled. `rudof` follows the text of the specification
and, where it is silent, the reference implementation,
[ShapePath.js](https://github.com/shexSpec/ShapePath.js):

| Point | What `rudof` does |
|---|---|
| A leading `/` | Has no effect on where the path begins: `/shapes/*` from an item that is not the schema selects nothing. |
| `//` | The same as `/`. |
| `<` and `>` | Are parsed, and select nothing. |
| A parenthesized step, `(path)` | Is parsed, and evaluating it is an error, as in the reference implementation. |
| `union` and `intersection` | Compare items by value, as the specification says: two identical constraints in different shapes are one item to them. |
| The members of an object | JSON objects are unordered; `*` over an object gives its members in the order of their keys. |
| A blank node label after `@`, `~` or `$` | Is not accepted, as the grammar takes only an IRI. |

The ShExJ document is the schema as `rudof` writes it as ShExJ, without the `prefixmap` and
`base` keys `rudof` adds, and with the `"type": "SemAct"` of every semantic action.

ShExPath addresses a schema. Evaluating a path against data is not supported.

## ShExPaths in validation results

When a node does not conform to a shape, the ShEx validation result names the parts of the schema
that fail with ShExPaths. Each line of the result that names a triple constraint ends with
`(at PATH)`, and the JSON result gives the paths in the `path` and `paths` fields of each error.
See [Reading the results](../cli_usage/shex_validate.md#reading-the-results).

The validation writes the paths with the shortcuts, and with the prefixes of the schema:

| Path | Names |
|---|---|
| `@:Person` | The shape declared as `:Person`. |
| `@:Person~:age` | The triple constraint on `:age` in that shape. |
| `@:Product~:code/self::[1]` | The second triple constraint on `:code`, when the shape has more than one. The first one is `/self::[0]`. |
| `$:knowsLabel` | The triple constraint with the label `:knowsLabel`. |
| `@:Order~:item/valueExpr` | The anonymous shape in the triple constraint on `:item`. |
| `@:Contact/shapeExprs/*[0]` | The first operand of the `AND` or `OR` that `:Contact` is. |
| `@:N/shapeExpr` | The shape expression that the `NOT` of `:N` negates. |
| `/start~:p` | The triple constraint on `:p` in the `start` shape expression. |

Each path selects exactly one item when you evaluate it against the schema:

```sh
rudof shex-path -s schema.shex -p "@:Product~:code/self::[1]"
```

A shape declared in an imported schema is named by its label too. The `shex-path` command evaluates
a path against the ShExJ form of one schema only. Thus, you must evaluate the path against the
imported schema. Validation results do not use paths for the shapes inside a shape that has a
blank node label, because a ShExPath cannot name such a label.
