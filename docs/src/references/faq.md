# FAQ

## Does `rudof` support ShEx?

Yes, indeed, rudof started as a ShEx implementation and its scope was later expanded to support SHACL and later other types of RDF data models in general. For more information about which ShEx features are supported you can follow [this issue](https://github.com/rudof-project/rudof/issues/8)

## Does `rudof` support SHACL?

Yes, you can follow [this issue](https://github.com/rudof-project/rudof/issues/94) for more information about SHACL support.

## Why did you choose `rudof` as the name?

The history of the name is the following:

- Initially, the project was called `shex-rs` because it was intended to support ShEx.
- Later, we expanded the scope to suport SHACl and it was renamed `shapes-rs`.
- The command line tool was initially called `sx` but it was considered [a bad name](https://github.com/rudof-project/rudof/issues/53) because it was already taken in linux.  
- We changed to a more specific name `rdfsx` but after recording a video tutorial about the tool, we realized that it was difficult to pronounce.
- After looking for alternatives, during a call with [Jonas Smedegaard](http://dr.jones.dk/blog/), he suggested `rudolf` because it was easy to pronounce, contained `rdf`, and was a nice character.
- However, `rudolf` was already taken by another `rdf` related project, so we decided to change it to `rudof` which is short and easy to pronounce, although it is not well spelled.

In [this issue](https://github.com/rudof-project/rudof/issues/53) you can follow the discussion and other alternatives we considered.

## What is the history of the logo?

The current logo was designed during the [DBCLS biohackathon 2024](https://2024.biohackathon.org/) in Fukushima by Yasunori Yamamoto and Masae Hosoda, with some input for Jose E. Labra.

The idea of the logo is that it combines [the `rudolf` character](https://en.wikipedia.org/wiki/Rudolph_the_Red-Nosed_Reindeer) with horns based on the [`rdf` logo](https://es.m.wikipedia.org/wiki/Archivo:Rdf_logo.svg)

More information about the logo can be tracked in [this issue](https://github.com/rudof-project/rudof/issues/107).

## What is the goal of the project?

The goal of `rudof` is to become a useful tool for RDF practitioners who want to check the quality of their RDF data, or want to process RDF data or RDF data shapes.

In [this page](https://github.com/rudof-project/rudof/wiki/%5BADR-04%5D-Scope-of-the-project)
