import sys
import unittest

from pyrudof import convert

dctap = """shapeId,shapeLabel,propertyId,Mandatory,Repeatable,valueDatatype,valueShape
Person,Shape or person,name,true,false,xsd:string, 
,,birthdate,false,false,xsd:date"""

