"""
Introspect the installed pyrudof package.
"""

from pathlib import Path

import pyrudof


def main() -> None:
    package_dir = Path(pyrudof.__file__).parent

    print(f"package: {pyrudof.__name__}")
    print(f"version: {bool(pyrudof.__version__)}")
    print(f"exported names: {len(pyrudof.__all__)}")
    print(f"Rudof lives in: {pyrudof.Rudof.__module__}")
    print(f"is typed: {(package_dir / 'py.typed').exists()}")


if __name__ == "__main__":
    main()
