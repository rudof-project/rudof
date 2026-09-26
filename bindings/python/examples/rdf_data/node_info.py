"""Inspect a node in the loaded RDF data.

``node_info`` renders the neighbourhood as a tree for a human to read;
``node_neighborhood`` yields the same arcs as objects for a program to walk.

The arcs are materialized before the iterator is handed back, so ``limit`` — not breaking
out of the loop — is what bounds the work on a node with a large fanout.
"""

from pathlib import Path

from pyrudof import RDFFormat, Rudof

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)

        print(rudof.node_info(":alice", [":name"], "outgoing", False, 1))

        for arc in rudof.node_neighborhood(":alice", depth=1):
            print(f"{arc.direction} {arc.predicate} -> {arc.neighbor}")

        # `limit` caps the arcs collected, and `truncated` says whether it cut the
        # neighbourhood short — which is how you tell "that was all of them" from
        # "there is more where that came from".
        capped = rudof.node_neighborhood(":alice", depth=1, limit=2)
        shown = [f"{arc.predicate} -> {arc.neighbor}" for arc in capped]
        print(f"showing {len(shown)} arcs, more available: {capped.truncated}")

        # A limit at or above the fanout is not a truncation.
        whole = rudof.node_neighborhood(":alice", depth=1, limit=10)
        print(f"showing {len(list(whole))} arcs, more available: {whole.truncated}")


if __name__ == "__main__":
    main()
