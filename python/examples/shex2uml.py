from pyrudof import Rudof, RudofConfig, UmlGenerationMode, ShExFormat
rudof = Rudof(RudofConfig())

rudof.read_shex_str("""
prefix : <http://example.org/>
prefix xsd:    <http://www.w3.org/2001/XMLSchema#>

:Person {
 :name xsd:string  ;
 :knows @:Person * ;
 :worksFor @:Company
}

:Company {
  :name xsd:string     ;
  :employee @:Person * ;
}
""")

uml_str = rudof.shex2plantuml(UmlGenerationMode.all())

print(uml_str)

