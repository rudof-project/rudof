from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())

valid_schema = """
PREFIX ex: <http://example.org/>
ex:PersonShape {
    ex:name .
}
"""
is_valid, message = rudof.check_shex(valid_schema)
print(is_valid)
print(message)

invalid_schema = """
PREFIX ex: <http://example.org/>
ex:Shape1 {
    ex:prop1 @ex:Shape2
}
ex:Shape2 {
    ex:prop2 NOT @ex:Shape1
}
"""
is_valid2, message2 = rudof.check_shex(invalid_schema)
print(is_valid2)
print(message2)
