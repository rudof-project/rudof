# Property Graph Schemas

`rudof` has recently added support for `PGSchema`, a language that is intended to validate Labeled Property Graphs and that has been described in [this paper](https://arxiv.org/abs/2211.10962). Rudof supports an extension of PGSchema that includes also property constraints.

Property graphs can be defined with the syntax provided by [YARSPG](https://github.com/lszeremeta/yarspg).

An example PGSchema is the following:

```gql
CREATE GRAPH TYPE Course {
( personType : Person { name : STRING , OPTIONAL age: INT })
( studentType : Person & Student { name STRING , age INT })
( courseType : Course ) { name STRING })
(: personType OPEN ) -[: knows { since DATE } ]->
(: personType OPEN )
(: studentType ) -[: enrolled { start DATE ,
OPTIONAL end DATE } ] - >(: courseType )
}
```

```sh
rudof pgschema examples/pgs/person.pgs
```
