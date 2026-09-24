"""Add, rename, copy and remove entries in the default prefix map."""

from pyrudof import Rudof


def main() -> None:
    with Rudof() as rudof:
        rudof.add_prefix("ex", "http://example.org/")
        rudof.add_prefix("foaf", "http://xmlns.com/foaf/0.1/")
        print(sorted(alias for alias, _iri in rudof.prefixes()))

        rudof.rename_prefix("foaf", "f")
        rudof.copy_prefix("ex", "example")
        rudof.remove_prefix("f")
        print(sorted(alias for alias, _iri in rudof.prefixes()))


if __name__ == "__main__":
    main()
