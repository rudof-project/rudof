# Getting started

For us to verify that the installation has been successful, you can run the following command to check that everything is working as expected.
Note that it should run with no failures.

```sh
rudof --version
```

Once we have `rudof` [installed](./installation.md) and verified, the next step is to start using the different subcommands that are bundled within the tool.
`rudof` is a tool to process and validate RDF data using shapes, as well as converting between different RDF data models.

As a command line tool, it contains several subcommands which can be decomposed in two main types:

- _Commands about some technology_, which take some input from that technology and provide information about it. Examples are: `shex`, `shacl`, `dctap`, `shapemap`, `service`, `node` and `data`. Which are usually nouns or the name of the corresponding technology.
- _Commands that do some actions_. Examples are: `query`, `validate` or `convert`, which are usually verbs.

Most of the commands can be customized either providing parameters at run-time or by passing a config file parameter. 

Config files use TOML syntax. By default, rudof uses the following [default_config.toml](https://github.com/rudof-project/rudof/blob/master/config/default_config.toml) file which can be used as a template if you want to customize it.