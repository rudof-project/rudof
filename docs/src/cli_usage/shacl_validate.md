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
