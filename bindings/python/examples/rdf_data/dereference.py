"""Fetch RDF data over HTTP(S) and merge it into the current graph.
"""

from pyrudof import ReaderMode, Rudof


def main() -> None:
    with Rudof() as rudof:
        rudof.dereference("https://www.w3.org/People/Berners-Lee/card", ReaderMode.Lax)
        print(rudof.serialize_data())


if __name__ == "__main__":
    main()
