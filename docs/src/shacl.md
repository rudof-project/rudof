# SHACL

Show information about a SHACL shapes graph

Example:

```sh
rudof shacl -s examples/simple_shacl.ttl 
```

It is also possible to read a SHACL shapes graph and convert it to some format

Example: Read a SHACL file in Turle and convert to RDF/XML

```sh
rudof shacl -s examples/simple_shacl.ttl -r rdfxml
```

## Validating RDF data using SHACL

We will make use of the [UserShape example in SHACL](https://book.validatingrdf.com/bookHtml011.html#ch050SHACLExample) from the [Validating RDF Data](https://book.validatingrdf.com/) book to demonstrate tha capabilities of the SHACL validator we propose.

According to this, an RDF graph conforming to the example above is:

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

For SHACL validation, we can use the generic `validate` command and the specific `shacl-validate` command.

### Using the generic `validate` command

```sh
rudof validate -M shacl -f turtle --schema examples/book.ttl examples/book_conformant.ttl
```

### Using the specific `shacl-validate` command

There is one `shacl-validate` command which can also be used. The difference is that instead of `schema`, it uses `shapes` and it doesn't require to specify a `Mode`:

```sh
rudof shacl-validate --shapes examples/book.ttl examples/book_conformant.ttl
```

An example of a non-conforming data graph is the following:

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

And can be run as:

```sh
rudof shacl-validate --shapes examples/book.ttl examples/book_non-conformant.ttl
```
