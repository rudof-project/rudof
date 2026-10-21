// Tests the remote SPARQL endpoint API (the `endpoints` feature) against a
// local SPARQL endpoint, served by another `Rudof` session over HTTP.
//
// Uses the build with the feature in pkg/endpoints, and needs a runtime with
// JSPI (Node.js 25+):
//   ./build.sh
//   node tests/endpoints.cjs
"use strict";
const assert = require("node:assert/strict");
const http = require("node:http");
const { Rudof, setFetch } = require("../pkg/endpoints/node/rudof_wasm.js");

const EX = "http://example.org/";
const DATA = `prefix : <${EX}>
:alice :name "Alice" ; :age 23 .
:bob   :name "Bob"   ; :age "unknown" .`;
const SCHEMA = `prefix : <${EX}>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string ; :age xsd:integer }`;
const SHAPES = `prefix : <${EX}>
prefix sh: <http://www.w3.org/ns/shacl#>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:PersonShape a sh:NodeShape ; sh:targetNode :alice, :bob ;
  sh:property [ sh:path :age ; sh:datatype xsd:integer ] .`;
const SHAPEMAP = `<${EX}alice>@<${EX}Person>, <${EX}bob>@<${EX}Person>`;

// The endpoint: a session with the data, whose results are written in the
// SPARQL 1.1 Query Results JSON Format.
const store = new Rudof();
store.readData(DATA);

function jsonTerm(term) {
  if (term.startsWith("<")) return { type: "uri", value: term.slice(1, -1) };
  if (term.startsWith("_:")) return { type: "bnode", value: term.slice(2) };
  const [, lexical, datatype, lang] = /^"((?:[^"\\]|\\.)*)"(?:\^\^<(.*)>|@(.*))?$/s.exec(term);
  const literal = { type: "literal", value: JSON.parse(`"${lexical}"`) };
  if (datatype) literal.datatype = datatype;
  if (lang) literal["xml:lang"] = lang;
  return literal;
}

function answer(query) {
  store.readQuery(query);
  const results = store.runQuery();
  if (results.kind === "ask") return { head: {}, boolean: results.boolean };
  const vars = results.variables.map((v) => v.replace(/^\?/, ""));
  const bindings = results.rows.map((row) =>
    Object.fromEntries(row.flatMap((term, i) => (term === null ? [] : [[vars[i], jsonTerm(term)]]))),
  );
  return { head: { vars }, results: { bindings } };
}

// Schemas imported by others, served under /shex/.
const SCHEMAS = {
  "/shex/person": SCHEMA,
};

let requests = 0;
const server = http.createServer((req, res) => {
  const url = new URL(req.url, "http://localhost");
  if (url.pathname.startsWith("/shex/")) {
    const schema = SCHEMAS[url.pathname];
    res.statusCode = schema === undefined ? 404 : 200;
    res.setHeader("content-type", "text/shex");
    res.end(schema ?? "Not found");
    return;
  }
  requests++;
  const query = url.searchParams.get("query");
  if (query.includes("BAD_REQUEST")) {
    res.statusCode = 400;
    res.end("Query rejected: BAD_REQUEST");
    return;
  }
  // Answer asynchronously, as a remote endpoint would.
  setTimeout(() => {
    res.setHeader("content-type", "application/sparql-results+json");
    res.end(JSON.stringify(answer(query)));
  }, 10);
});

const tests = [];
const test = (name, fn) => tests.push({ name, fn });

function session(endpoint) {
  const rudof = new Rudof();
  rudof.useEndpoint(endpoint);
  return rudof;
}

test("ShEx validation against an endpoint", async (endpoint) => {
  const rudof = session(endpoint);
  rudof.readShex(SCHEMA);
  rudof.readShapemap(SHAPEMAP);
  let ticks = 0;
  const timer = setInterval(() => ticks++, 1);
  const report = await rudof.validateShexAsync();
  clearInterval(timer);
  const status = Object.fromEntries(report.entries.map((e) => [e.node, e.status]));
  assert.deepEqual(status, { [`${EX}alice`]: "conformant", [`${EX}bob`]: "nonconformant" });
  assert.ok(ticks > 0, "the event loop runs while waiting for the endpoint");
});

test("SHACL validation against an endpoint", async (endpoint) => {
  const rudof = session(endpoint);
  rudof.readShacl(SHAPES);
  const report = await rudof.validateShaclAsync();
  assert.equal(report.conforms, false);
  assert.deepEqual(
    report.entries.map((e) => e.focusNode),
    [`${EX}bob`],
  );
});

test("SPARQL queries and node info against an endpoint", async (endpoint) => {
  const rudof = session(endpoint);
  rudof.readQuery(`SELECT ?name WHERE { ?p <${EX}name> ?name } ORDER BY ?name`);
  const select = await rudof.runQueryAsync();
  assert.deepEqual(select.rows, [['"Alice"'], ['"Bob"']]);
  rudof.readQuery(`ASK { <${EX}alice> <${EX}age> 23 }`);
  assert.equal((await rudof.runQueryAsync()).boolean, true);
  assert.match(await rudof.nodeInfoAsync(`<${EX}alice>`), /Alice/);
});

test("the synchronous API cannot wait for an endpoint", async (endpoint) => {
  const rudof = session(endpoint);
  rudof.readQuery("SELECT * WHERE { ?s ?p ?o }");
  assert.throws(() => rudof.runQuery(), /asynchronous/);
  // The session is still usable.
  assert.equal((await rudof.runQueryAsync()).rows.length, 4);
});

test("HTTP errors carry the endpoint's message", async (endpoint) => {
  const rudof = session(endpoint);
  rudof.readQuery(`SELECT * WHERE { <${EX}BAD_REQUEST> ?p ?o }`);
  await assert.rejects(rudof.runQueryAsync(), (e) => e.name === "QueryError" && /400.*BAD_REQUEST/s.test(e.message));
});

test("sessions work concurrently, each one call at a time", async (endpoint) => {
  const a = session(endpoint);
  const b = session(endpoint);
  a.readQuery("SELECT ?s WHERE { ?s ?p ?o }");
  b.readQuery(`ASK { ?s <${EX}age> 23 }`);
  const pending = a.runQueryAsync();
  // `a` is in use until its call settles.
  assert.throws(() => a.readQuery("ASK { ?s ?p ?o }"));
  const [ra, rb] = await Promise.all([pending, b.runQueryAsync()]);
  assert.equal(ra.rows.length, 4);
  assert.equal(rb.boolean, true);
  a.readQuery("ASK { ?s ?p ?o }");
  assert.equal((await a.runQueryAsync()).boolean, true);
});

test("setFetch replaces fetch", async (endpoint) => {
  const seen = [];
  setFetch((url, init) => {
    seen.push(init.headers.Accept);
    return fetch(url, init);
  });
  try {
    const rudof = session(endpoint);
    rudof.readQuery("ASK { ?s ?p ?o }");
    assert.equal((await rudof.runQueryAsync()).boolean, true);
    assert.deepEqual(seen, ["application/sparql-results+json"]);
  } finally {
    setFetch(null);
  }
});

test("network errors are reported after the retries", async (endpoint) => {
  setFetch(() => Promise.reject(new TypeError("fetch failed")));
  try {
    const rudof = session(endpoint);
    rudof.readQuery("ASK { ?s ?p ?o }");
    await assert.rejects(rudof.runQueryAsync(), /fetch failed/);
  } finally {
    setFetch(null);
  }
});

test("readShexAsync fetches the imported schemas", async (endpoint) => {
  const base = new URL("/shex/", endpoint).href;
  const rudof = new Rudof();
  rudof.readData(DATA);
  await rudof.readShexAsync(`prefix : <${EX}>\nIMPORT <person>\n:Named { :name . }`, null, base);
  rudof.readShapemap(SHAPEMAP);
  const report = rudof.validateShex();
  const status = Object.fromEntries(report.entries.map((e) => [e.node, e.status]));
  assert.deepEqual(status, { [`${EX}alice`]: "conformant", [`${EX}bob`]: "nonconformant" });
});

test("imports that cannot be fetched are reported", async (endpoint) => {
  const rudof = new Rudof();
  const missing = new URL("/shex/missing", endpoint).href;
  await assert.rejects(rudof.readShexAsync(`IMPORT <${missing}>`), /missing.*404/s);
});

test("readShex cannot wait for imports", async (endpoint) => {
  const rudof = new Rudof();
  const person = new URL("/shex/person", endpoint).href;
  assert.throws(() => rudof.readShex(`IMPORT <${person}>`), /asynchronous/);
});

test("in-memory data still works synchronously", async () => {
  const rudof = new Rudof();
  rudof.readData(DATA);
  rudof.readShex(SCHEMA);
  rudof.readShapemap(SHAPEMAP);
  assert.equal(rudof.validateShex().conforms, false);
});

server.listen(0, "127.0.0.1", async () => {
  const endpoint = `http://127.0.0.1:${server.address().port}/sparql`;
  let failed = 0;
  for (const { name, fn } of tests) {
    try {
      await fn(endpoint);
      console.log(`ok - ${name}`);
    } catch (error) {
      failed++;
      console.log(`not ok - ${name}\n${error.stack}`);
    }
  }
  console.log(`${tests.length - failed}/${tests.length} passed (${requests} requests to the endpoint)`);
  server.close();
  process.exitCode = failed ? 1 : 0;
});
