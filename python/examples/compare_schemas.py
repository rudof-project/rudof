from pyrudof import ReaderMode, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

schema1 = """
PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
  :name xsd:string
}
"""

schema2 = """
PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
  :name xsd:string ;
  :age xsd:integer ?
}
"""

comparison = rudof.compare_schemas(
    schema1,
    schema2,
    "shex",
    "shex",
    "shexc",
    "shexc",
    None,
    None,
    "http://example.org/Person",
    "http://example.org/Person",
    ReaderMode.Lax,
)

print("COMPARE_SCHEMAS_OK")
print(f"Comparison chars: {len(comparison)}")
