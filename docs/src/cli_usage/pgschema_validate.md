# pgschema-validate: Validating Property graphs data with PGSchema

rudof has basic support for validating property graphs using [PGSchema](https://arxiv.org/abs/2211.10962). 

Currently, the validation requires a PGSchema file, a property graph using the [YARSPG](https://github.com/lszeremeta/yarspg) syntax and a typemap that associates nodes/edges with node/edge types.

The folder [examples/property_graphs](https://github.com/rudof-project/rudof/tree/master/examples/property_graphs) contains several examples of property graphs, property graph schemas and type maps. 

As an example, the following command:

```sh 
rudof pgschema-validate --schema demo.pgs --typemap demo.map demo.pg
```



