# pgs - Property Graph Schemas

A property graph schema contains a list of `CREATE` node/edge/graph `TYPE` statements. It follows the grammar from the [PGSchema](https://arxiv.org/abs/2211.10962) paper with some extensions for property constraints.

The full grammar is available [here](https://github.com/weso/pgschemapc/blob/main/src/parser/pgs.rustemo).

An example property graph schema is:

```cypher
CREATE NODE TYPE ( PersonType : Person {
    name: STRING,
    OPTIONAL age: INTEGER
}) ;
CREATE NODE TYPE ( StudentType : Person & Student {
    name: STRING,
    OPTIONAL age: INTEGER
}) ;
CREATE NODE TYPE ( CourseType: Course {
    name: STRING
})
```
