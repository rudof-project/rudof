"""Catch the exceptions rudof raises.

Every error is an exception rooted at ``RudofError``, one class per failure
domain, so a caller can be as specific or as broad as it likes.
"""

from pyrudof import InputError, Rudof, RudofError, ShExError, ShExFormat


def main() -> None:
    rudof = Rudof()

    try:
        rudof.read_shex("this is not a schema", ShExFormat.ShExC)
    except ShExError as e:  # specific: the schema itself is at fault
        print(f"schema error: {type(e).__name__}")

    try:
        rudof.read_data("/no/such/file.ttl")
    except InputError as e:  # specific: the input could not be resolved
        print(f"input error: {type(e).__name__}")
        # The #[source] chain survives the boundary as exception notes.
        for note in getattr(e, "__notes__", []):
            print(f"  caused by: {note}")

    try:
        rudof.validate_shex()
    except RudofError as e:  # the base class catches every rudof error
        print(f"rudof error: {type(e).__name__}")
        print(f"  is a RudofError: {isinstance(e, RudofError)}")


if __name__ == "__main__":
    main()
