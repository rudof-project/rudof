# test-suite 

The project contains a list of tests in the folder [tests](https://github.com/weso/pgschemapc/tree/main/tests). 

The test cases are defined with four files:

- A property graph, usually with the extension `.pg`
- A property graph schema, usually with the extension `.pgs`
- An input type map association file: `.map` which declares which nodes/edges in the property graph should validate with which type names in the property graph schema. 
- A result type map association file, which declares the expected result. 
