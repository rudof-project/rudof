# shapemap

[Shape Maps](https://shexspec.github.io/shape-map/) is a specification that can be used to select a set of nodes that can be used to validate against a set of shapes. Shape Maps can also be used to obtain information about the result of a ShEx validation.
The command `shapemap` can be used to obtain information about shape maps.

Assuming that the following shapemap is stored in the file `user.sm`:

```shex
:a@:User
```

The following command can be run to obtain information about the shapemap.

```sh
rudof shapemap -m examples/user.sm
```
