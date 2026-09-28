<!-- WARNING: This file has been generated automatically from the
module's option declarations — do not edit directly. -->

# Nix module options

These options are shared by the `rudof.nixosModules.default` and
`rudof.homeModules.default` flake outputs.

## programs\.rudof\.enable

Whether to enable rudof\.



*Type:*
boolean



*Default:*

```nix
false
```



*Example:*

```nix
true
```



## programs\.rudof\.package



The rudof package to use



*Type:*
package



*Default:*

```nix
rudof.packages.<system>.rudof
```



## programs\.rudof\.extraArgs



Additional arguments passed to rudof



*Type:*
list of string



*Default:*

```nix
[ ]
```



*Example:*

```nix
[
  "--config-file"
  "/etc/rudof/config.toml"
  "--force-overwrite"
]
```



## programs\.rudof\.settings



Configuration for rudof in nix



*Type:*
open submodule of (TOML value)



*Default:*

```nix
{ }
```



*Example:*

```nix
{
  base_iri = "http://base_iri/";
  rdf = {
    base_iri = "http://new_base_iri/";
  };
  version = "0.3.24";
}
```


