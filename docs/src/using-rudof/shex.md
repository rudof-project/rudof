# ShEx

[ShEX](https://shex.io/shex-semantics/) is a language for validating and describing RDF data.
In `rudof` there exist several supported operations regarding ShEx, namely, obtaining information about the ShEx schema, or validating an RDF graph using ShEx.

For executing the examples in this page you can download the following files:

```sh
curl -o user.shex https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/user.shex
curl -o user.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/user.ttl
```

## Information about the ShEx schema

You can read a ShEx schema in compact syntax and show its JSON representation using the instruction below.

```sh
rudof shex -s user.shex
```

## ShEx-based validation

It is also possible to use `rudof` to validate ShEx schemas using the following instruction:

```sh
rudof validate --schema user.shex --node :a --shape-label :User user.ttl
```
