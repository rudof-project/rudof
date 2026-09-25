"""Read an inline RDF-config YAML specification and serialize it back."""

from pyrudof import RdfConfigFormat, ResultRdfConfigFormat, Rudof

CONFIG = """
- Person ex:person1 ex:person2:
  - a: ex:Person
  - rdfs:label:
    - name: "Alice"
  - ex:age?:
    - age_value: 32
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_rdf_config(CONFIG, RdfConfigFormat.Yaml)
        print(rudof.serialize_rdf_config(ResultRdfConfigFormat.Internal))


if __name__ == "__main__":
    main()
