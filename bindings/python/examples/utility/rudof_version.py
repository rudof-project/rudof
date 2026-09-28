"""Read the version of the rudof core the session wraps."""

from pyrudof import Rudof


def main() -> None:
    with Rudof() as rudof:
        version = rudof.get_version()
        print(f"Version: {version}")
        print(f"non-empty: {bool(version)}")


if __name__ == "__main__":
    main()
