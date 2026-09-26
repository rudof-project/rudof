// Validates the same RDF data with ShEx and with SHACL from Node.js.
// Run `./build.sh` first, then: node examples/node.cjs
const { validateShex, validateShacl } = require("../pkg-node/rudof_wasm.js");

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

const shex = JSON.parse(validateShex(data, shexSchema, ":alice@:Person, :bob@:Person"));
console.log("ShEx conforms:", shex.conforms);
for (const r of shex.results) {
  console.log(`  ${r.node} @ ${r.shape}: ${r.status}`);
}

const shacl = JSON.parse(validateShacl(data, shaclShapes));
console.log("SHACL conforms:", shacl.conforms);
for (const r of shacl.results) {
  console.log(`  ${r.focusNode} ${r.path}: ${r.severity} (${r.constraintComponent})`);
}

try {
  validateShex(data, "not ShEx", ":alice@:Person");
} catch (e) {
  console.log("Errors are thrown as exceptions:", e.message.split("\n")[0]);
}
