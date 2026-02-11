# validate property graphs

The `validate` command can be used to validate nodes or edges from a Property Graph to check if they conform with type node declarations in a Property Graph Schema. To trigger validation it is necessary to add a third parameter, which is a type map  association file which declares which nodes/edges should be checked against which type names.

An example validation can be run by:

```sh
pgschemapc validate --graph examples/course.pg --schema examples/course.pgs --map examples/course.map
```
