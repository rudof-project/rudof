// Validates RDF data with ShEx and SHACL, and queries it with SPARQL, from Node.js.
// Run `./build.sh` first, then: node examples/node.cjs
const { Rudof, RudofConfig, validateShex, validateShacl } = require("../pkg-node/rudof_wasm.js");

const data = `
prefix : <http://example.org/>
:alice :name "Alice" ; :age 23 .
:bob   :name "Bob"   ; :age "unknown" .
`;

const shexSchema = `
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string ; :age xsd:integer }
`;

const shaclShapes = `
prefix : <http://example.org/>
prefix sh: <http://www.w3.org/ns/shacl#>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:PersonShape a sh:NodeShape ;
  sh:targetSubjectsOf :name ;
  sh:property [ sh:path :age ; sh:datatype xsd:integer ] .
`;

// One-shot validation.
const shex = validateShex(data, shexSchema, ":alice@:Person, :bob@:Person");
console.log("ShEx conforms:", shex.conforms);
for (const e of shex.entries) {
  console.log(`  ${e.node} @ ${e.shape}: ${e.status}`);
}

const shacl = validateShacl(data, shaclShapes);
console.log("SHACL conforms:", shacl.conforms);
for (const e of shacl.entries) {
  console.log(`  ${e.focusNode} ${e.path}: ${e.severity} (${e.constraintComponent})`);
}

// A session, as in the Python bindings: loaded data and schemas persist
// across calls.
const rudof = new Rudof(RudofConfig.fromToml('base_iri = "http://example.org/"'));
console.log(`rudof ${rudof.getVersion()}`);
rudof.readData(data);
rudof.readShex(shexSchema);
rudof.readShapemap("{FOCUS :name _}@:Person");
const report = rudof.validateShex();
console.log(`Nodes with a name, validated as :Person (${report.violations.length} violation(s)):`);
console.log(rudof.serializeShexValidationResults("compact"));

rudof.readQuery("PREFIX : <http://example.org/> SELECT ?name ?age WHERE { ?p :name ?name ; :age ?age }");
const results = rudof.runQuery();
console.log("SPARQL:", results.kind, results.variables, results.rows);

// Schema conversion: ShEx to a PlantUML class diagram. Only in builds with the
// `conversion` feature (see the README).
if (typeof rudof.convertSchemas === "function") {
  const uml = rudof.convertSchemas(shexSchema, "shex", "uml", "shexc", "uml");
  console.log("UML:", uml.split("\n").slice(0, 3).join(" | "), "...");
}

try {
  rudof.readShex("not ShEx");
} catch (e) {
  console.log(`Errors are thrown as exceptions: ${e.name}: ${e.message.split("\n")[0]}`);
}
