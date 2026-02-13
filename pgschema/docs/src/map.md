# map - Type map associations

Type map associations are used to trigger validation. They associate nodes/edges in the property graph with type names in the property graph schema.

The full grammar of type maps is available [here](https://github.com/weso/pgschemapc/blob/main/src/parser/map.rustemo)

An example type map association is:

```
n1:PersonType,
n2:StudentType,
n3:CourseType
```

Notice that type maps can also be used to specify the expected result of validation which can be positive or negative. For negative validation results, we use a `!` after the colon. These result type maps are employed in the test-suite which is available [here](https://github.com/weso/pgschemapc/tree/main/tests).
