"""Show the triples of each RDF graph that the other one does not have, up to
blank node renaming. When one graph is contained in the other the diff is
exact; otherwise the best blank node mapping is searched for.
"""

from pyrudof import GraphRelation, Rudof, SubgraphMode

BOB = """
PREFIX : <http://example.org/>
:alice :knows _:b .
_:b :name "Bob" .
"""

ROBERT = """
PREFIX : <http://example.org/>
:alice :knows _:c .
_:c :name "Robert" .
"""

NAMED_BOB = """
PREFIX : <http://example.org/>
:alice :knows :bob .
:bob :name "Bob" ; :age 30 .
"""


def main() -> None:
    with Rudof() as rudof:
        diff = rudof.diff_data(BOB, ROBERT)
        print(f"relation: {diff.relation}, exact: {diff.exact}")
        print(f"only in first: {diff.only_in_first}")
        print(f"only in second: {diff.only_in_second}")
        print(f"blank nodes: {diff.first_to_second}")

        # With simple entailment, _:b is :bob, so only the age is extra
        diff = rudof.diff_data(BOB, NAMED_BOB, mode=SubgraphMode.Entailment)
        print(f"entailment: {diff.relation == GraphRelation.Subgraph}, extra: {diff.only_in_second}")

        # The first graph can also be the loaded data
        rudof.read_data(BOB)
        print(f"loaded data: {rudof.diff_loaded_data(ROBERT)}")


if __name__ == "__main__":
    main()
