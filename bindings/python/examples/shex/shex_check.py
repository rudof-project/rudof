"""Check the well-formedness of a valid and an invalid ShEx schema.

``check_shex`` does not load the schema into the session: it reports whether the
schema compiles, and why not when it does not.
"""

from pyrudof import Rudof

VALID_SCHEMA = """
PREFIX ex: <http://example.org/>
ex:PersonShape {
    ex:name .
}
"""

# Shape1 and Shape2 are mutually dependent through a negation: a negative cycle.
CYCLIC_SCHEMA = """
PREFIX ex: <http://example.org/>
ex:Shape1 {
    ex:prop1 @ex:Shape2
}
ex:Shape2 {
    ex:prop2 NOT @ex:Shape1
}
"""


def main() -> None:
    with Rudof() as rudof:
        is_valid, message = rudof.check_shex(VALID_SCHEMA)
        print(is_valid)
        print(message)

        is_valid, message = rudof.check_shex(CYCLIC_SCHEMA)
        print(is_valid)
        print(message)


if __name__ == "__main__":
    main()
