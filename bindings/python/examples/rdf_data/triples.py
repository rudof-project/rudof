"""Walk the loaded RDF data triple by triple, and change it.

``triples`` matches a pattern: every position left out matches anything, so no
arguments at all walks the whole graph. ``add_triple`` and ``remove_triple`` change the
graph in place, which is what building a graph programmatically (or applying what a
validation report says is missing) takes, with no serialize-edit-reload round trip.

Terms are written as text, in the syntax the rest of the API accepts: ``<IRI>``,
``ex:alice``, ``_:b1``, ``"Alice"``, ``"Alice"@en``, ``23``, ``"23"^^xsd:integer``.
"""

from pathlib import Path

from pyrudof import RDFFormat, ResultDataFormat, Rudof

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)
        # A prefixed name is resolved against the prefixes of the data, and then the
        # session's. `person.ttl` declares `:`, so `xsd:` has to come from here.
        rudof.add_prefix("xsd", "http://www.w3.org/2001/XMLSchema#")

        # No pattern walks the whole graph. A triple unpacks into its three terms.
        for subject, predicate, obj in rudof.triples():
            print(f"{subject} {predicate} {obj}")

        # A pattern narrows it, in any combination of positions.
        names = list(rudof.triples(predicate=":name"))
        print(f"names: {[triple.object for triple in names]}")
        print(f"alice's triples: {len(list(rudof.triples(subject=':alice')))}")

        # Build on the graph without going through its serialization.
        rudof.add_triple(":alice", ":knows", ":bob")
        rudof.add_triple(":bob", ":name", '"Bob"')
        rudof.add_triple(":bob", ":age", '"19"^^xsd:integer')
        print(f"bob's triples: {len(list(rudof.triples(subject=':bob')))}")

        # The additions are in the graph itself, so every other operation sees them.
        turtle = rudof.serialize_data(ResultDataFormat.Turtle)
        print(f"serialized has bob: {'bob' in turtle}")

        # Removing a triple that is not there is not an error, so a removal needs no
        # existence check first.
        rudof.remove_triple(":bob", ":name", '"Bob"')
        rudof.remove_triple(":carol", ":name", '"Carol"')
        print(f"bob's triples after the removal: {len(list(rudof.triples(subject=':bob')))}")

        # `limit` caps the triples collected, and `truncated` says whether it cut the
        # result short, which is how you tell "that was all of them" from "there is
        # more where that came from".
        capped = rudof.triples(limit=2)
        print(f"showing {len(list(capped))} triples, more available: {capped.truncated}")
        whole = rudof.triples(limit=100)
        print(f"showing {len(list(whole))} triples, more available: {whole.truncated}")


if __name__ == "__main__":
    main()
