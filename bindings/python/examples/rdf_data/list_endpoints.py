"""List the SPARQL endpoints the session knows about.

The list comes from the configuration, but it is read against the loaded graph,
so data must be in the session first.
"""

from pathlib import Path

from pyrudof import RDFFormat, Rudof

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)

        endpoints = rudof.list_endpoints()
        print(f"endpoints: {len(endpoints) > 0}")
        for name, url in sorted(endpoints):
            print(f"{name}: {url}")


if __name__ == "__main__":
    main()
