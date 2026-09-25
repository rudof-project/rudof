"""Replace the configuration of a session that is already open.

``update_config`` swaps the configuration the session will use from now on. Data
already loaded keeps the settings it was read with, so the graph has to be reset
and read again for the new endpoint table to take effect.
"""

from pathlib import Path

from pyrudof import RDFFormat, Rudof, RudofConfig

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    # Rudof() with no argument uses the default configuration.
    with Rudof() as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)
        before = sorted(dict(rudof.list_endpoints()))
        print(f"default endpoints: {before}")

        rudof.update_config(RudofConfig.from_path(HERE / "example.toml"))
        rudof.reset_data()
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)
        after = sorted(dict(rudof.list_endpoints()))
        print(f"after update_config: {after}")
        print(f"configuration changed: {before != after}")


if __name__ == "__main__":
    main()
