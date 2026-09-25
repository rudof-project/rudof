"""Inspect a node in the loaded RDF data.

``node_info`` renders the neighbourhood as a tree for a human to read;
``node_neighborhood`` yields the same arcs as objects for a program to walk.
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


if __name__ == "__main__":
    main()
