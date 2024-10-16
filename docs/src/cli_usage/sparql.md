# SPARQL

`rudof` has some basic support for SPARQL queries, which can be interacted with by means of the `query` command.
It is possible not only to perform SPARQL queries on SPARQL endpoints, but also on local files.

## Querying over local files

For querying over local files you need both a SPARQL file and a data graph.
The idea is that we will apply the SPARQL query over the provided graph.
For us to do so you just need to use the `query` command.

> For executing the examples you can execute the instructions below to obtain the required files. 

```sh
curl -o user.sparql https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/user.sparql
curl -o user.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/user.ttl
```
Then you can run the instruction below.

```sh
rudof query -q user.sparql user.ttl
```

## Querying over SPARQL endpoints

It is also possible to run a SPARQL query that obtains its data from a SPARQL endpoint.
However, the idea is the exact same as in the case above, with the difference that the SPARQL endpoint should be provided by means of the `-e` argument.

> For executing the examples you can execute the instructions below to obtain the required files. 

```sh
curl -o wikidata.sparql https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/wikidata.sparql
```

Then you can run the instruction below.

```sh
rudof query -q wikidata.sparql -e wikidata
```

> The Wikidata endpoint is registered within the `rudof` tool, for ease of use.