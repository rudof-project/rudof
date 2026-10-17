// Validates and queries data from a remote SPARQL endpoint (Wikidata), from Node.js.
//
// Uses the build with the `endpoints` feature in pkg/endpoints (in the npm
// package, `require("@rudof/rudof/endpoints")`), which relies on WebAssembly
// JSPI (Node.js 25+):
//   ./build.sh
//   node examples/endpoint.cjs
const { Rudof, setFetch } = require("../pkg/endpoints/node/rudof_wasm.js");

// Wikimedia asks clients to identify themselves. Natively, rudof sends a
// User-Agent; from JavaScript, `setFetch` can add headers to its requests.
setFetch((url, init) =>
  fetch(url, {
    ...init,
    headers: { ...init.headers, "User-Agent": "rudof-wasm-example (https://github.com/rudof-project/rudof)" },
  }),
);

// Douglas Adams (Q42): a human (P31 Q5) with a date of birth (P569).
const schema = `
prefix wd:  <http://www.wikidata.org/entity/>
prefix wdt: <http://www.wikidata.org/prop/direct/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
prefix :    <http://example.org/>

:Human {
  wdt:P31 [ wd:Q5 ] ;
  wdt:P569 xsd:dateTime ;
}
`;

async function main() {
  const rudof = new Rudof();
  rudof.useEndpoint("https://query.wikidata.org/sparql");

  rudof.readShex(schema);
  rudof.readShapemap("<http://www.wikidata.org/entity/Q42>@<http://example.org/Human>");
  const report = await rudof.validateShexAsync();
  for (const e of report.entries) {
    console.log(`${e.node} @ ${e.shape}: ${e.status}`);
  }

  rudof.readQuery(`
    PREFIX wd: <http://www.wikidata.org/entity/>
    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
    SELECT ?label WHERE { wd:Q42 rdfs:label ?label FILTER(lang(?label) = "en") }`);
  const results = await rudof.runQueryAsync();
  console.log("Label:", results.rows.map((row) => row[0]).join(", "));
}

main().catch((error) => {
  console.error(`${error.name}: ${error.message}`);
  process.exit(1);
});
