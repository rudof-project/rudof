# Data Generator TODO

## Constraint Implementation Status

### ✅ **Currently Working Constraints**

**Basic Constraints:**
- `sh:datatype` - Datatype specifications (string, integer, decimal, boolean, dateTime, etc.)
- `sh:minCount` / `sh:maxCount` - Cardinality constraints
- `sh:nodeKind` - Node type constraints (IRI, Literal, BlankNode, etc.)
- Shape references for nested objects

**Framework:**
- Constraint parameter passing from schemas to field generators
- Field generators that can respect min/max, length, pattern parameters
- Unified constraint model supporting all constraint types
- Proper integration between SHACL converter and data generation

### ❌ **Constraints NOT Working (SHACL Parser Limitations)**

The following constraints are defined in the SHACL specification and have conversion code implemented, but the underlying `shacl_ast` crate parser does not extract them from SHACL schemas:

**Numeric Value Constraints:**
- `sh:minInclusive` - Minimum inclusive numeric value
- `sh:maxInclusive` - Maximum inclusive numeric value  
- `sh:minExclusive` - Minimum exclusive numeric value
- `sh:maxExclusive` - Maximum exclusive numeric value

**String Constraints:**
- `sh:minLength` - Minimum string length
- `sh:maxLength` - Maximum string length
- `sh:pattern` - Regular expression pattern matching

**Value Constraints:**
- `sh:hasValue` - Fixed/constant values
- `sh:in` - Enumerated value lists

**Advanced SHACL Features:**
- Closed shapes with proper validation
- Complex logical constraints (`sh:or`, `sh:and`, `sh:not`, `sh:xone`)
- `sh:equals`, `sh:disjoint`, `sh:lessThan`, `sh:lessThanOrEquals`
- Language constraints (`sh:languageIn`, `sh:uniqueLang`)

## Root Cause Analysis

The issue is in the SHACL parsing layer (`shacl_ast` crate), not in the data generator itself. Debug analysis shows:

1. **SHACL Schema Parsing**: Only `MinCount`, `MaxCount`, and `Datatype` components are being extracted from test schemas
2. **Missing Components**: `MinExclusive`, `MaxExclusive`, `MinLength`, `MaxLength`, `Pattern`, etc. are completely missing from parsed schemas
3. **Parser Limitation**: The `shacl_ast` implementation appears to be incomplete and doesn't support the full SHACL specification

## Technical Details

**What Works:**
- The constraint conversion code exists and is correct (`convert_component` method)
- The unified constraint model supports all constraint types
- Field generators can handle parameters when provided
- The constraint parameter passing framework is fully functional

**What's Missing:**
- Complete SHACL specification support in the `shacl_ast` crate
- Parsing of numeric facet constraints from RDF/Turtle SHACL schemas
- Parsing of string constraint properties
- Parsing of value enumeration constraints

## Recommendations

### Short Term
1. Document unsupported constraints clearly
2. Remove tests that depend on unsupported SHACL features
3. Focus on constraints that work with current parser capabilities

### Long Term
1. **Enhance SHACL Parser**: Extend `shacl_ast` crate to support missing SHACL constraint types
2. **Alternative Parser**: Consider using a different SHACL parsing library or implementing custom RDF property extraction
3. **Direct RDF Parsing**: Bypass `shacl_ast` and parse SHACL constraints directly from RDF triples for unsupported features

## Test Categories

### Passing Tests ✅
- Basic passthrough tests (datatype, cardinality)
- Constraint passthrough tests (framework verification)
- SHACL integration tests (basic functionality)
- Configuration tests
- Debug tests

### Failing Tests (Due to Parser Limitations) ❌
- Specific constraint tests (numeric ranges, string lengths, patterns)
- Constraint validation tests (comprehensive constraint enforcement)
- Advanced SHACL feature tests

## Implementation Priority

1. **High Priority**: Extend SHACL parser to support numeric value constraints (`minInclusive`, `maxInclusive`, etc.)
2. **Medium Priority**: Add string constraint support (`minLength`, `maxLength`, `pattern`)
3. **Low Priority**: Implement advanced logical constraints and closed shape validation

The constraint enforcement framework is ready and waiting for a more complete SHACL parser.
