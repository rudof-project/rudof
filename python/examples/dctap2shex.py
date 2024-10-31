from pyrudof import Rudof, RudofConfig, ShExFormat, ShExFormatter, DCTapFormat



rudof = Rudof(RudofConfig())
dctap_str = """shapeId,propertyId,Mandatory,Repeatable,valueDatatype,valueShape
Person,name,true,false,xsd:string, 
,birthdate,false,false,xsd:date, 
,worksFor,false,true,,Company
Company,name,true,false,xsd:string,   
,employee,false,true,,Person
"""
rudof.read_dctap_str(dctap_str)

dctap = rudof.get_dctap()
print(f"DCTAP\n{dctap}")

rudof.dctap2shex()
result = rudof.serialize_shex(ShExFormatter())
print(f"DCTAP converted to ShEx\n{result}")

