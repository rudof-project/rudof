"""Select elements of a ShEx schema with ShExPath.

``evaluate_shex_path`` evaluates a `ShExPath <https://shexspec.github.io/spec/ShExPath>`_
against the loaded schema, written with the schema's prefixes and base. Each selected
element comes back as a dict with its ``location`` in the ShExJ form of the schema (a
JSON Pointer) and its ``value`` as ShExJ.
"""

from pyrudof import Rudof

SCHEMA = """
PREFIX : <http://ex.example/ns#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
BASE <http://ex.example/schema>

<#IssueShape> {
  :name xsd:string MINLENGTH 4 ;
  :category ["bug" "feature request"] ;
  $<#reportedBy> :postedBy @<#UserShape>
}

<#UserShape> IRI
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shex(SCHEMA)

        # The :category triple constraint of #IssueShape.
        for match in rudof.evaluate_shex_path("@<#IssueShape>~:category"):
            print(match["location"])
            print(match["value"]["predicate"])

        # A reference to a shape is an IRI, not the shape.
        [reference] = rudof.evaluate_shex_path("$<#reportedBy>/valueExpr")
        print(reference["value"])

        # Every triple constraint of #IssueShape.
        constraints = rudof.evaluate_shex_path("@<#IssueShape>/expression/expressions/*")
        print(f"{len(constraints)} constraints")


if __name__ == "__main__":
    main()
