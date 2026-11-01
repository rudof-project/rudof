"""Compare two RDF graphs up to blank node renaming: isomorphic, equivalent,
subgraph, supergraph or different. RDF 1.2 triple terms are supported.

``mode`` says how blank nodes may be mapped when checking containment:
``Injective`` (the default), ``Homomorphic`` or ``Entailment``.
"""

from pyrudof import GraphRelation, RDFFormat, Rudof, SubgraphMode

ALICE = """
PREFIX : <http://example.org/>
:alice :knows [ :name "Bob" ] .
"""

# The same graph with another blank node label and another syntax
ALICE_NT = """
<http://example.org/alice> <http://example.org/knows> _:someone .
_:someone <http://example.org/name> "Bob" .
"""

ALICE_MORE = ALICE + ':alice :name "Alice" .'

ANNOTATED = """
PREFIX : <http://example.org/>
:alice :knows :bob {| :source :facebook |} .
"""

REIFIED = """
PREFIX : <http://example.org/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
:alice :knows :bob .
_:r rdf:reifies <<( :alice :knows :bob )>> ; :source :facebook .
"""

# Bob, named: an instance of ALICE, where the blank node is :bob
ALICE_BOB = """
PREFIX : <http://example.org/>
:alice :knows :bob . :bob :name "Bob" .
"""


def main() -> None:
    with Rudof() as rudof:
        same = rudof.compare_data(ALICE, ALICE_NT, format2=RDFFormat.NTriples)
        print(f"turtle vs n-triples: {same}")
        print(f"subgraph: {rudof.compare_data(ALICE, ALICE_MORE)}")
        print(f"supergraph: {rudof.compare_data(ALICE_MORE, ALICE)}")
        print(f"rdf 1.2 annotation vs reification: {rudof.compare_data(ANNOTATED, REIFIED)}")
        different = rudof.compare_data(ALICE, ANNOTATED)
        print(f"is different: {different == GraphRelation.Different}")

        # Blank nodes may be instantiated by any term with the entailment mode
        for mode in SubgraphMode.all():
            print(f"{mode} mode: {rudof.compare_data(ALICE, ALICE_BOB, mode=mode)}")

        # The first graph can also be the loaded data
        rudof.read_data(ALICE)
        print(f"loaded data: {rudof.compare_loaded_data(ALICE_MORE)}")


if __name__ == "__main__":
    main()
