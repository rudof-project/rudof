from rudof import parse
from rudof import validate

ans = parse("examples/simple_shacl.ttl", "simple_shacl.rdf")
ans = validate("examples/book.ttl", "examples/book_conformant.ttl")