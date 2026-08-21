# Property Graph Schemas

`rudof` has recently added support for `PGSchema`, a language that is intended to validate Labeled Property Graphs and that has been described in [this paper](https://arxiv.org/abs/2211.10962). Rudof supports an extension of PGSchema that includes also property constraints.

Property graphs can be defined with the syntax provided by [YARSPG](https://github.com/lszeremeta/yarspg).

An example PGSchema is the following:

```gql
CREATE NODE TYPE ( PersonType : Person {
    name: STRING,
    OPTIONAL age: INTEGER
}) ;
CREATE NODE TYPE ( StudentType : Person & Student {
    name: STRING,
    OPTIONAL age: INTEGER CHECK (> 18)
}) ;
CREATE NODE TYPE ( CourseType: Course {
    name: STRING
}) ;
CREATE EDGE TYPE (:PersonType)
                 -[KnowsType : Knows { since: INTEGER }]->
                 (:PersonType);
CREATE EDGE TYPE (:StudentType)
                 -[EnrolledInType : EnrolledIn { start: INTEGER, end: INTEGER }]->
                 (:CourseType)
```

This schema is available at [examples/property_graphs/demo.pgs](https://github.com/rudof-project/rudof/tree/master/examples/property_graphs/demo.pgs).

```sh
rudof pgschema -s examples/property_graphs/demo.pgs
```
