"""Stage triples, check whether they hold, and keep or discard them.

``staging()`` is a context manager over the loaded data: leaving its block normally
keeps what was staged, leaving it through an exception puts the data back as it was.
So "add these triples only if the graph still validates" is one block, with no copy of
the graph and no reload.

Rolling back costs what was staged, not the size of the graph. Only the **data** is
staged: the shapes stay loaded either way, and a report already in hand is a copy, so
it can still be read after the rollback. What a rollback does clear is what was
computed from the data it threw away (the validation results, the query results and
the loaded query).
"""

from pyrudof import RDFFormat, Rudof, ShaclFormat, ShaclValidationMode

SHAPES = """
PREFIX : <http://example.org/>
PREFIX sh: <http://www.w3.org/ns/shacl#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:PersonShape a sh:NodeShape ;
  sh:targetClass :Person ;
  sh:property [
    sh:path :name ;
    sh:datatype xsd:string ;
    sh:minCount 1 ;
    sh:maxCount 1
  ] .
"""

DATA = """
PREFIX : <http://example.org/>

:alice a :Person ;
  :name "Alice" .
"""


class ShapesViolated(Exception):
    """Raised to leave a staging block through an exception, rolling it back."""


def stage(rudof: Rudof, delta: list[tuple[str, str, str]]) -> bool:
    """Adds `delta` if the data still conforms, and discards it if it does not."""
    try:
        with rudof.staging():
            rudof.add_triples(delta)
            report = rudof.validate_shacl(ShaclValidationMode.Native)
            if not report.conforms:
                raise ShapesViolated(f"{len(report)} violation(s)")
        return True
    except ShapesViolated:
        return False


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shacl(SHAPES, ShaclFormat.Turtle)
        rudof.read_data(DATA, RDFFormat.Turtle)
        print(f"triples: {len(list(rudof.triples()))}")

        # A delta that conforms: the block is left normally, so it is committed.
        delta = [
            (":bob", "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>", ":Person"),
            (":bob", ":name", '"Bob"'),
        ]
        kept = stage(rudof, delta)
        print(f"bob kept: {kept}")
        print(f"triples: {len(list(rudof.triples()))}")

        # A delta that does not: a second `:name` breaks `sh:maxCount 1`, the
        # exception leaves the block, and the data goes back to what it was.
        kept = stage(rudof, [(":bob", ":name", '"Roberto"')])
        print(f"second name kept: {kept}")
        print(f"triples: {len(list(rudof.triples()))}")
        print(f"staging depth: {rudof.staging_depth()}")

        # The shapes are session state, not staged state: still loaded, still usable.
        report = rudof.validate_shacl(ShaclValidationMode.Native)
        print(f"conforms after the rollback: {report.conforms}")


if __name__ == "__main__":
    main()
