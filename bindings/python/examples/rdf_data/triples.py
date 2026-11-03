"""Walk the loaded RDF data triple by triple, and change it.

``triples`` matches a pattern: every position left out matches anything, so no
arguments at all walks the whole graph. ``add_triple`` and ``remove_triple`` change the
graph in place, which is what building a graph programmatically (or applying what a
validation report says is missing) takes, with no serialize-edit-reload round trip.

Terms are written as text, in RDF 1.2 Turtle syntax: ``<IRI>``, ``ex:alice``, ``_:b1``,
``"Alice"``, ``"Alice"@en``, ``"Shalom"@he--rtl``, ``23``, ``"23"^^xsd:integer``, and the
triple term ``<<( :alice :name "Alice" )>>``. The terms a triple is read back in are
written the same way, so a triple that comes out of ``triples`` goes into ``add_triple``
and ``remove_triple`` unedited.
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

        # The RDF 1.2 terms are written as text too: a triple term, and a string with a
        # base direction, which tells it apart from the same string without one.
        triple_term = '<<( :alice :name "Alice" )>>'
        rudof.add_triple(":bob", ":believes", triple_term)
        rudof.add_triple(":alice", ":greeting", '"Shalom"@he--rtl')
        rudof.add_triple(":alice", ":greeting", '"Shalom"@he')
        print(f"what bob believes: {len(list(rudof.triples(subject=':bob', predicate=':believes')))}")
        # The triple term is a term of the graph, so it matches as a pattern of its own.
        print(f"found by the triple term: {len(list(rudof.triples(object=triple_term)))}")
        print(f"greetings: {len(list(rudof.triples(predicate=':greeting')))}")
        print(f"directional only: {len(list(rudof.triples(object='\"Shalom\"@he--rtl')))}")

        # A triple read from the graph goes back in as it came out, with no editing and
        # whatever its terms are: an IRI comes back bracketed and a literal with the
        # lexical form it was written with, which is what `strict_iris=True` demands.
        read = list(rudof.triples())
        for subject, predicate, obj in read:
            rudof.remove_triple(subject, predicate, obj, strict_iris=True)
        print(f"triples after removing every one of them: {len(list(rudof.triples()))}")
        for subject, predicate, obj in read:
            rudof.add_triple(subject, predicate, obj, strict_iris=True)
        print(f"triples after adding them back: {len(list(rudof.triples()))}")


if __name__ == "__main__":
    main()
