from pyrudof import Rudof, RudofConfig, ShExFormatter, UmlGenerationMode

rudof = Rudof(RudofConfig())
dctap_str = """shapeId,propertyId,Mandatory,Repeatable,valueDatatype,valueShape
Person,name,true,false,xsd:string,
,birthdate,false,false,xsd:date,
,enrolledIn,false,true,,Course
Course,name,true,false,xsd:string,
,student,false,true,,Person
"""
rudof.read_dctap_str(dctap_str)

dctap = rudof.get_dctap()
print(f"DCTAP\n{dctap}")

rudof.dctap2shex()
result = rudof.serialize_current_shex(ShExFormatter())
print(f"DCTAP converted to ShEx\n{result}")

uml = rudof.shex2plantuml(UmlGenerationMode.all())
print(f"DCTAP converted to UML\n{uml}")