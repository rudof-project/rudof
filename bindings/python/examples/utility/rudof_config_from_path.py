"""Create a RudofConfig from a TOML file and open a session with it.

The file declares its own ``[rdf.endpoints]`` table, which replaces the built-in
defaults rather than adding to them.
"""

from pathlib import Path

from pyrudof import RDFFormat, Rudof, RudofConfig

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    config = RudofConfig.from_path(HERE / "example.toml")

    with Rudof(config) as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)

        endpoints = dict(rudof.list_endpoints())
        print(f"endpoints from example.toml: {sorted(endpoints)}")


if __name__ == "__main__":
    main()
