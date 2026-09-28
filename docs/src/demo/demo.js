// rudof online demo: converting, validating (ShEx, SHACL, PGSchema) and
// querying (SPARQL) RDF data and property graphs with the @rudof/rudof npm
// package (rudof compiled to WebAssembly), loaded from jsDelivr.

// An exact version: jsDelivr caches what a range (like 0.3) points to, so a
// range can keep serving an older release for days. The release workflow
// (release.yml) updates it, and the site is deployed again once the release is
// on npm (gh-pages.yml).
const RUDOF_VERSION = "0.3.24";
const RUDOF_MODULE = `https://cdn.jsdelivr.net/npm/@rudof/rudof@${RUDOF_VERSION}/web/rudof_wasm.js`;

// PlantUML, compiled to JavaScript with TeaVM, draws the diagrams. It is
// loaded the first time a diagram is shown.
const PLANTUML_VERSION = "1.2026.8";
const PLANTUML_BASE = `https://cdn.jsdelivr.net/npm/@plantuml/core@${PLANTUML_VERSION}/`;

const $ = (id) => document.getElementById(id);

// ---------------------------------------------------------------------------
// Tabs
//
// Two levels: the sections (Data, Validate, Query), and inside each one its
// own tabs (RDF and Property graph, ShEx, SHACL and PGSchema, SPARQL). The URL
// fragment names both, e.g. #validate/shacl.

const tabName = (tab) => tab.id.replace("tab-", "");
const tabsOf = (tablist) => [...tablist.querySelectorAll(':scope > [role="tab"]')];
const sectionTabs = tabsOf(document.querySelector('.tabs:not(.subtabs)'));

// The tab selected inside a section.
function selectedSubtab(section) {
  return tabsOf($(`panel-${tabName(section)}`).querySelector(".subtabs")).find(
    (t) => t.getAttribute("aria-selected") === "true",
  );
}

function selectTab(tab, focus = false) {
  for (const t of tabsOf(tab.parentElement)) {
    const selected = t === tab;
    t.setAttribute("aria-selected", String(selected));
    t.tabIndex = selected ? 0 : -1;
    $(t.getAttribute("aria-controls")).hidden = !selected;
  }
  if (focus) tab.focus();
  const section = sectionTabs.find((t) => t.getAttribute("aria-selected") === "true");
  history.replaceState(null, "", `#${tabName(section)}/${tabName(selectedSubtab(section))}`);
}

for (const tab of document.querySelectorAll('[role="tab"]')) {
  tab.addEventListener("click", () => selectTab(tab));
  tab.addEventListener("keydown", (e) => {
    const tabs = tabsOf(tab.parentElement);
    const i = tabs.indexOf(tab);
    const next = { ArrowRight: i + 1, ArrowLeft: i - 1, Home: 0, End: tabs.length - 1 }[e.key];
    if (next === undefined) return;
    e.preventDefault();
    selectTab(tabs[(next + tabs.length) % tabs.length], true);
  });
}

// Earlier links named only the tab: #shex, #shacl, #pgschema, #rdf.
const OLD_FRAGMENTS = { shex: "validate/shex", shacl: "validate/shacl", pgschema: "validate/pgschema", rdf: "data/rdf" };

// The URL fragment selects the tabs, on load and when it changes.
function selectTabFromHash() {
  const fragment = location.hash.slice(1);
  const [section, subtab] = (OLD_FRAGMENTS[fragment] ?? fragment).split("/");
  const sectionTab = sectionTabs.find((t) => tabName(t) === section);
  if (!sectionTab) return;
  const sub = subtab && $(`tab-${subtab}`);
  if (sub && $(`panel-${section}`).contains(sub)) selectTab(sub);
  selectTab(sectionTab);
}
selectTabFromHash();
window.addEventListener("hashchange", selectTabFromHash);

// ---------------------------------------------------------------------------
// Results

// `output` is text, an element (a table built from a report), or
// `{ plantuml }`: the source of a diagram, which is drawn.
function showResult(prefix, { verdict, kind, output, millis }) {
  const box = $(`${prefix}-result`);
  box.hidden = false;
  box.dataset.kind = kind;
  box.querySelector(".verdict").textContent = verdict;
  box.querySelector(".timing").textContent = millis === undefined ? "" : `${millis.toFixed(0)} ms`;
  const pre = box.querySelector(".output");
  const table = box.querySelector(".table-output");
  const diagram = box.querySelector(".diagram");
  const isElement = output instanceof Element;
  const isDiagram = typeof output === "object" && output !== null && "plantuml" in output;
  if (table) {
    table.hidden = !isElement;
    table.replaceChildren(...(isElement ? [output] : []));
  }
  if (diagram) diagram.hidden = !isDiagram;
  if (isDiagram) {
    pre.textContent = output.plantuml;
    pre.hidden = !diagram.classList.contains("show-source");
    drawDiagram(box, output.plantuml);
    return;
  }
  diagrams.set(box, null); // a diagram still being drawn is not shown
  pre.hidden = isElement;
  if (!isElement) pre.textContent = plain(output);
}

// Diagrams

let plantuml;

function loadPlantuml() {
  plantuml ??= (async () => {
    // Graphviz (Viz.js), which PlantUML uses for layout, is a classic script.
    await new Promise((resolve, reject) => {
      const script = Object.assign(document.createElement("script"), { src: `${PLANTUML_BASE}viz-global.js` });
      script.onload = resolve;
      script.onerror = () => reject(new Error("could not load Graphviz"));
      document.head.append(script);
    });
    return import(`${PLANTUML_BASE}plantuml.js`);
  })();
  plantuml.catch(() => (plantuml = undefined)); // try again next time
  return plantuml;
}

async function plantumlSvg(source) {
  const { renderToString } = await loadPlantuml();
  return new Promise((resolve, reject) =>
    renderToString(source.split(/\r\n|\r|\n/), resolve, (message) => reject(new Error(message))),
  );
}

// The diagram being drawn in each result box, so that only the last one is shown.
const diagrams = new WeakMap();

async function drawDiagram(box, source) {
  const image = box.querySelector(".diagram-image");
  const download = box.querySelector('[data-action="download"]');
  const token = {};
  diagrams.set(box, token);
  const note = (text, className = "diagram-status") => image.replaceChildren(Object.assign(document.createElement("p"), { className, textContent: text }));
  note(plantuml ? "Drawing the diagram…" : "Loading PlantUML (about 2 MB, only the first time)…");
  download.removeAttribute("href");
  try {
    const svg = await plantumlSvg(source);
    if (diagrams.get(box) !== token) return;
    // As an image, so that nothing in the SVG runs in the page
    if (box.dataset.svg) URL.revokeObjectURL(box.dataset.svg);
    box.dataset.svg = URL.createObjectURL(new Blob([svg], { type: "image/svg+xml" }));
    image.replaceChildren(Object.assign(new Image(), { src: box.dataset.svg, alt: "Diagram" }));
    download.href = box.dataset.svg;
  } catch (e) {
    if (diagrams.get(box) === token) note(`Could not draw the diagram: ${e.message ?? e}`, "diagram-status error");
  }
}

for (const diagram of document.querySelectorAll(".diagram")) {
  diagram.querySelector('[data-action="source"]').addEventListener("click", (e) => {
    const shown = diagram.classList.toggle("show-source");
    diagram.closest(".result").querySelector(".output").hidden = !shown;
    e.currentTarget.textContent = shown ? "Hide PlantUML source" : "Show PlantUML source";
  });
}

let crashed = false;

function showError(prefix, e) {
  // A panic in Rust aborts the WebAssembly module, which can't be used again.
  if (e instanceof WebAssembly.RuntimeError) {
    crashed = true;
    setStatus("error", "rudof stopped working: reload the page to continue");
    for (const b of document.querySelectorAll('button[id$="-validate"]')) b.disabled = true;
    showResult(prefix, {
      verdict: "Internal error",
      kind: "error",
      output: `rudof crashed (${e.message}). Please reload the page, and report the input at\nhttps://github.com/rudof-project/rudof/issues`,
    });
    return;
  }
  showResult(prefix, {
    verdict: e.name && e.name !== "Error" ? e.name : "Error",
    kind: "error",
    output: e.message ?? String(e),
  });
}

// Tables are made for terminals: remove their colors (ANSI escape codes) and
// hyperlinks (OSC 8), which browsers would show as stray characters.
const TERMINAL_ESCAPES = /\x1b\]8;[^\x1b\x07]*(?:\x1b\\|\x07)|\x1b\[[0-9;]*[A-Za-z]/g;

function plain(text) {
  return text.replace(TERMINAL_ESCAPES, "");
}

function count(n, word, plural = `${word}s`) {
  return `${n} ${n === 1 ? word : plural}`;
}

function setStatus(kind, text) {
  const status = $("status");
  status.className = `status ${kind}`;
  status.textContent = text;
}

// ---------------------------------------------------------------------------
// Validation

let rudofModule;
const sessions = {};

function session(name) {
  sessions[name] ??= new rudofModule.Rudof();
  return sessions[name];
}

function validateShex() {
  const rudof = session("shex");
  const start = performance.now();
  rudof.resetAll();
  rudof.readData($("shex-data").value, $("shex-data-format").value);
  rudof.readShex($("shex-schema").value, $("shex-schema-format").value);
  rudof.readShapemap($("shex-shapemap").value);
  const report = rudof.validateShex();
  const millis = performance.now() - start;
  const failed = report.entries.filter((e) => e.status !== "conformant").length;
  return {
    verdict: report.conforms
      ? `Conforms: ${count(report.entries.length, "node")} validated`
      : `Does not conform: ${failed} of ${count(report.entries.length, "node")} failed`,
    kind: report.conforms ? "ok" : "fail",
    millis,
  };
}

function serializeShex() {
  return session("shex").serializeShexValidationResults($("shex-result-format").value);
}

function validateShacl() {
  const rudof = session("shacl");
  const start = performance.now();
  rudof.resetAll();
  rudof.readData($("shacl-data").value, $("shacl-data-format").value);
  rudof.readShacl($("shacl-shapes").value, $("shacl-shapes-format").value);
  const report = rudof.validateShacl($("shacl-mode").value);
  const millis = performance.now() - start;
  const violations = report.entries.filter((e) => e.severity === "Violation").length;
  const others = report.entries.length - violations;
  return {
    verdict: report.conforms
      ? "Conforms"
      : `Does not conform: ${count(violations, "violation")}${others ? `, ${count(others, "other result")}` : ""}`,
    kind: report.conforms ? "ok" : "fail",
    millis,
  };
}

function serializeShacl() {
  return session("shacl").serializeShaclValidationResults($("shacl-result-format").value);
}

function validatePgschema() {
  const rudof = session("pgschema");
  const start = performance.now();
  rudof.resetAll();
  rudof.readData($("pgschema-data").value, "pg");
  rudof.readPgschema($("pgschema-schema").value);
  rudof.readTypemap($("pgschema-typemap").value);
  const report = rudof.validatePgschema();
  const millis = performance.now() - start;
  const failed = report.entries.filter((e) => !e.conforms).length;
  return {
    verdict: report.conforms
      ? `Conforms: ${count(report.entries.length, "node or edge", "nodes or edges")} validated`
      : `Does not conform: ${failed} of ${count(report.entries.length, "node or edge", "nodes or edges")} failed`,
    kind: report.conforms ? "ok" : "fail",
    millis,
    report,
  };
}

// A table of the report: one row per node or edge of the type map.
function pgschemaTable(report) {
  const el = (tag, props = {}, ...children) => {
    const e = Object.assign(document.createElement(tag), props);
    e.append(...children);
    return e;
  };
  const rows = report.entries.map((e) =>
    el(
      "tr",
      { className: e.conforms ? "ok" : "fail" },
      el("td", {}, el("code", {}, e.nodeId)),
      el("td", {}, el("code", {}, e.typeName)),
      el("td", { className: "verdict-cell" }, e.conforms ? "✓ conforms" : "✗ fails"),
      el("td", {}, el("div", { className: "details" }, e.details)),
    ),
  );
  return el(
    "table",
    { className: "report" },
    el("thead", {}, el("tr", {}, ...["Id", "Type", "Result", "Details"].map((h) => el("th", {}, h)))),
    el("tbody", {}, ...rows),
  );
}

function serializePgschema() {
  const format = $("pgschema-result-format").value;
  if (format === "table") return pgschemaTable(last.pgschema.report);
  return session("pgschema").serializePgschemaValidationResults(format);
}

const label = (id) => $(id).selectedOptions[0].text;

function rdfVerdict(triples) {
  const to = label("rdf-result-format").replace(" (PlantUML)", "");
  return `${count(triples, "triple")}, ${label("rdf-data-format")} → ${to}`;
}

function convertRdf() {
  const rudof = session("rdf");
  const start = performance.now();
  rudof.resetAll();
  rudof.readData($("rdf-data").value, $("rdf-data-format").value);
  // Counted in N-Triples, one triple per line.
  const triples = rudof.serializeData("ntriples").split("\n").filter((l) => l.trim()).length;
  const millis = performance.now() - start;
  return {
    verdict: rdfVerdict(triples),
    kind: "ok",
    millis,
    triples,
  };
}

function serializeRdf() {
  const format = $("rdf-result-format").value;
  let output = session("rdf").serializeData(format);
  if (format === "plantuml") output = { plantuml: output };
  // The verdict names the output format, which may have just changed.
  last.rdf.verdict = rdfVerdict(last.rdf.triples);
  return output;
}

// Property graph data, written back in YARS-PG or as JSON.
function convertPg() {
  const rudof = session("pg");
  const start = performance.now();
  rudof.resetAll();
  rudof.readData($("pg-data").value, "pg");
  let size = "";
  try {
    const graph = JSON.parse(rudof.serializeData("json"));
    size = `${count(graph.nodes.length, "node")} and ${count(graph.edges.length, "edge")}, `;
  } catch {
    // rudof versions before 0.3.23 don't write property graphs as JSON.
  }
  const millis = performance.now() - start;
  return { size, kind: "ok", millis, verdict: `${size}YARS-PG → ${label("pg-result-format")}` };
}

function serializePg() {
  last.pg.verdict = `${last.pg.size}YARS-PG → ${label("pg-result-format")}`;
  return session("pg").serializeData($("pg-result-format").value);
}

// SPARQL queries. The result formats depend on the kind of query: SELECT gives
// a table of solutions, ASK true or false, CONSTRUCT and DESCRIBE a graph.
const SPARQL_FORMATS = {
  select: [["table", "Table"], ["json", "JSON"], ["csv", "CSV"]],
  ask: [["text", "Text"]],
  graph: [
    ["turtle", "Turtle"],
    ["ntriples", "N-Triples"],
    ["rdfxml", "RDF/XML"],
    ["jsonld", "JSON-LD"],
    ["trig", "TriG"],
    ["n3", "N3"],
    ["nquads", "N-Quads"],
  ],
};

const SPARQL_EXAMPLES = {
  select: `PREFIX foaf: <http://xmlns.com/foaf/0.1/>

SELECT ?name ?age WHERE {
  ?person a foaf:Person ;
          foaf:name ?name .
  OPTIONAL { ?person foaf:age ?age }
}
ORDER BY ?name`,
  count: `PREFIX foaf: <http://xmlns.com/foaf/0.1/>

SELECT ?name (COUNT(?friend) AS ?friends) WHERE {
  ?person foaf:name ?name .
  OPTIONAL { ?person foaf:knows ?friend }
}
GROUP BY ?name
ORDER BY DESC(?friends)`,
  ask: `PREFIX foaf: <http://xmlns.com/foaf/0.1/>

ASK { ?person foaf:age ?age FILTER (?age < 18) }`,
  construct: `PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX : <http://example.org/>

CONSTRUCT { ?b :isKnownBy ?a } WHERE { ?a foaf:knows ?b }`,
  describe: `DESCRIBE <http://example.org/carol>`,
};

$("sparql-example").addEventListener("change", (e) => {
  if (!e.target.value) return;
  $("sparql-query").value = SPARQL_EXAMPLES[e.target.value];
  e.target.value = "";
  $("sparql-query").focus();
});

function setSparqlFormats(kind) {
  const select = $("sparql-result-format");
  const formats = SPARQL_FORMATS[kind];
  if ([...select.options].map((o) => o.value).join() === formats.map(([v]) => v).join()) return;
  select.replaceChildren(...formats.map(([value, text]) => new Option(text, value)));
}

function runSparql() {
  const rudof = session("sparql");
  const start = performance.now();
  rudof.resetAll();
  rudof.readData($("sparql-data").value, $("sparql-data-format").value);
  rudof.readQuery($("sparql-query").value);
  const results = rudof.runQuery();
  const millis = performance.now() - start;
  const prefixes = dataPrefixes(rudof);
  setSparqlFormats(results.kind);
  let verdict;
  if (results.kind === "select") verdict = count(results.rows.length, "result");
  else if (results.kind === "ask") verdict = `ASK: ${results.boolean}`;
  else verdict = count(results.graph.split("\n").filter((l) => l.trim()).length, "triple");
  return { verdict, kind: "ok", millis, results, prefixes };
}

// The prefixes declared in the loaded data, as [alias, IRI] pairs, read from
// its Turtle serialization.
function dataPrefixes(rudof) {
  return [...rudof.serializeData("turtle").matchAll(/^@prefix ([^:\s]*): <([^>]*)> \.$/gm)].map((m) => [m[1], m[2]]);
}

const XSD = "http://www.w3.org/2001/XMLSchema#";

// An RDF term (as rudof writes it: <iri>, "literal"@lang, "literal"^^<type>,
// _:blank) shown as a reader would write it: IRIs as prefixed names when a
// prefix of the data matches, literals by their value with a note for a
// language or an unusual datatype. The full term is the tooltip.
function termCell(term, prefixes) {
  const td = document.createElement("td");
  if (term == null) return td;
  const qualify = (iri) => {
    for (const [alias, ns] of [...prefixes, ["xsd", XSD]]) {
      if (iri.startsWith(ns) && /^[\w-]*$/.test(iri.slice(ns.length))) return `${alias}:${iri.slice(ns.length)}`;
    }
    return `<${iri}>`;
  };
  let text = term;
  let note = "";
  const iri = term.match(/^<(.*)>$/s);
  const literal = term.match(/^"(.*)"(?:@([\w-]+)|\^\^<(.*)>)?$/s);
  if (iri) {
    text = qualify(iri[1]);
  } else if (literal) {
    text = literal[1].replace(/\\(["\\])/g, "$1").replace(/\\n/g, "\n");
    const [, , lang, datatype] = literal;
    const plain = ["string", "integer", "decimal", "double", "boolean"].map((t) => XSD + t);
    if (lang) note = `@${lang}`;
    else if (datatype && !plain.includes(datatype)) note = qualify(datatype);
  }
  const code = Object.assign(document.createElement("code"), { textContent: text, title: term });
  td.append(code);
  if (note) td.append(" ", Object.assign(document.createElement("span"), { className: "term-note", textContent: note }));
  return td;
}

// Solutions of a SELECT query as a table; unbound values are left empty.
function sparqlTable({ variables, rows }, prefixes) {
  const el = (tag, props = {}, ...children) => {
    const e = Object.assign(document.createElement(tag), props);
    e.append(...children);
    return e;
  };
  return el(
    "table",
    { className: "report solutions" },
    el("thead", {}, el("tr", {}, ...variables.map((v) => el("th", {}, v)))),
    el(
      "tbody",
      {},
      ...rows.map((row) => el("tr", {}, ...row.map((value) => termCell(value, prefixes)))),
    ),
  );
}

function serializeSparql() {
  const { results, prefixes } = last.sparql;
  const format = $("sparql-result-format").value;
  if (results.kind === "select") {
    return format === "table" ? sparqlTable(results, prefixes) : session("sparql").serializeQueryResults(format);
  }
  if (results.kind === "ask") return String(results.boolean);
  // The graph of CONSTRUCT and DESCRIBE, written with the prefixes of the data.
  const declarations = prefixes.map(([alias, iri]) => `prefix ${alias}: <${iri}>\n`).join("");
  const graph = session("sparql-graph");
  graph.resetAll();
  graph.readData(declarations + results.graph, "turtle");
  return graph.serializeData(format);
}

// Schemas converted to other formats (and diagrams), in their own sessions so
// that validation results are kept.
const schemaInput = { shex: "shex-schema-format", shacl: "shacl-shapes-format" };

function schemaVerdict(language) {
  return `${label(schemaInput[language])} → ${label(`${language}-schema-result-format`)}`;
}

function readSchema(language) {
  const rudof = session(`${language}-schema`);
  const start = performance.now();
  rudof.resetAll();
  if (language === "shex") rudof.readShex($("shex-schema").value, $("shex-schema-format").value);
  else rudof.readShacl($("shacl-shapes").value, $("shacl-shapes-format").value);
  return { kind: "ok", millis: performance.now() - start, verdict: schemaVerdict(language) };
}

// SHACL shapes converted to ShEx (ShExJ), through Turtle.
function shaclToShex(format) {
  const turtle = session("shacl-schema").serializeShacl("turtle");
  return session("shacl-convert").convertSchemas(turtle, "shacl", "shex", "turtle", format);
}

function serializeSchema(language) {
  const rudof = session(`${language}-schema`);
  const format = $(`${language}-schema-result-format`).value;
  last[`${language}-schema`].verdict = schemaVerdict(language);
  if (language === "shex") {
    if (format === "plantuml") return { plantuml: rudof.serializeCurrentShex("plantuml") };
    if (format === "sparql") {
      return rudof.convertSchemas(rudof.serializeCurrentShex("shexj"), "shex", "sparql", "shexj", "internal");
    }
    return rudof.serializeCurrentShex(format);
  }
  // SHACL
  if (format === "shexc") return shaclToShex("shexc");
  if (format === "plantuml") {
    const shex = session("shacl-shex");
    shex.resetAll();
    shex.readShex(shaclToShex("shexj"), "shexj");
    return { plantuml: shex.serializeCurrentShex("plantuml") };
  }
  return rudof.serializeShacl(format);
}

const validators = {
  shex: { validate: validateShex, serialize: serializeShex },
  shacl: { validate: validateShacl, serialize: serializeShacl },
  pgschema: { validate: validatePgschema, serialize: serializePgschema },
  rdf: { validate: convertRdf, serialize: serializeRdf },
  pg: { validate: convertPg, serialize: serializePg },
  sparql: { validate: runSparql, serialize: serializeSparql },
};
// PGSchema has one syntax, PGSchemaC, which rudof can't write yet.
for (const language of ["shex", "shacl"]) {
  validators[`${language}-schema`] = { validate: () => readSchema(language), serialize: () => serializeSchema(language) };
}

// Data tabs: copy the result, or use it as the new input.
const CONVERTERS = ["rdf", "pg"];

function updateConvertButtons() {
  for (const prefix of CONVERTERS) {
    const converted = Boolean(last[prefix]) && !crashed;
    const format = $(`${prefix}-result-format`).value;
    $(`${prefix}-copy`).disabled = !converted;
    // Only data in a format that can be read back
    $(`${prefix}-use-as-input`).disabled = !converted || format === "plantuml" || (prefix === "pg" && format === "json");
  }
}

for (const prefix of CONVERTERS) {
  const output = () => $(`${prefix}-result`).querySelector(".output").textContent;
  $(`${prefix}-copy`).addEventListener("click", async (e) => {
    const button = e.currentTarget;
    try {
      await navigator.clipboard.writeText(output());
      button.textContent = "Copied";
    } catch {
      button.textContent = "Copy failed";
    }
    setTimeout(() => (button.textContent = "Copy"), 1500);
  });
  $(`${prefix}-use-as-input`).addEventListener("click", () => {
    $(`${prefix}-data`).value = output();
    if (prefix === "rdf") $("rdf-data-format").value = $("rdf-result-format").value;
    $(`${prefix}-data`).focus();
  });
}

// The last successful validation of each tab, so that changing the result
// format serializes it again without validating again.
const last = {};

function run(prefix) {
  if (!rudofModule || crashed) return;
  const { validate, serialize } = validators[prefix];
  try {
    last[prefix] = validate();
    const output = serialize();
    showResult(prefix, { ...last[prefix], output });
  } catch (e) {
    delete last[prefix];
    showError(prefix, e);
  }
  updateConvertButtons();
}

function reformat(prefix) {
  if (!last[prefix] || crashed) return;
  try {
    const output = validators[prefix].serialize();
    showResult(prefix, { ...last[prefix], output });
  } catch (e) {
    showError(prefix, e);
  }
  updateConvertButtons();
}

for (const prefix of Object.keys(validators)) {
  $(`${prefix}-validate`).addEventListener("click", () => run(prefix));
  $(`${prefix}-result-format`).addEventListener("change", () => reformat(prefix));
  $(`panel-${prefix}`)?.addEventListener("keydown", (e) => {
    if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      run(prefix);
    }
  });
}

// ---------------------------------------------------------------------------
// Loading

try {
  rudofModule = await import(RUDOF_MODULE);
  await rudofModule.default();
  // The version reported by the loaded module, not the one requested
  setStatus("ready", `rudof ${new rudofModule.Rudof().getVersion()} ready`);
  $("status").title = `@rudof/rudof from ${RUDOF_MODULE}`;
  for (const b of document.querySelectorAll('button[id$="-validate"]')) b.disabled = false;
} catch (e) {
  console.error(e);
  setStatus("error", `Could not load rudof: ${e.message ?? e}`);
}
