from pyrudof import Rudof, RudofConfig, ShExFormatter, 

rudof = Rudof(RudofConfig())

schema1 = """
 PREFIX : <http://example.org/>
 PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
 :Person {
    :name xsd:string ;
    :age xsd:integer ;
    :weight xsd:float ;
    :worksFor @:Company
 } 
 :Company {
    :name xsd:string ;
    :employee @:Person
 }"""

# rudof.read_data_str(schema1)

schema2 = """
 PREFIX ex: <http://example.org/> 
 PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
 ex:Person {
    ex:name xsd:string ;
    ex:birthDate xsd:date ;
    ex:worksFor @ex:Company
}
ex:Company {
   ex:name xsd:string
}
"""
print("Comparing schemas:");
result = result = rudof.compare_schemas_str(
    schema1, schema2, 
    "shex", "shex", 
    "shexc", "shexc", 
    None, None, 
    "http://example.org/Person", "http://example.org/Person",

    )

print(f"Schemas compared: {result.as_json()}")