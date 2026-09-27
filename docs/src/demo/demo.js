// rudof online demo: ShEx and SHACL validation with the @rudof/rudof npm
// package (rudof compiled to WebAssembly), loaded from jsDelivr.

// Any 0.3.x release. Update the range when a release changes the API.
const RUDOF_VERSION = "0.3";
const RUDOF_MODULE = `https://cdn.jsdelivr.net/npm/@rudof/rudof@${RUDOF_VERSION}/web/rudof_wasm.js`;

const $ = (id) => document.getElementById(id);

// ---------------------------------------------------------------------------
// Tabs

const tabs = [...document.querySelectorAll('[role="tab"]')];

function selectTab(tab, focus = false) {
  for (const t of tabs) {
    const selected = t === tab;
    t.setAttribute("aria-selected", String(selected));
    t.tabIndex = selected ? 0 : -1;
    $(t.getAttribute("aria-controls")).hidden = !selected;
  }
  if (focus) tab.focus();
  history.replaceState(null, "", `#${tab.id.replace("tab-", "")}`);
}

for (const tab of tabs) {
  tab.addEventListener("click", () => selectTab(tab));
  tab.addEventListener("keydown", (e) => {
    const i = tabs.indexOf(tab);
    const next = { ArrowRight: i + 1, ArrowLeft: i - 1, Home: 0, End: tabs.length - 1 }[e.key];
    if (next === undefined) return;
    e.preventDefault();
    selectTab(tabs[(next + tabs.length) % tabs.length], true);
  });
}

// The URL fragment (#shex, #shacl, ...) selects a tab, on load and when it changes.
function selectTabFromHash() {
  const tab = tabs.find((t) => `#${t.id.replace("tab-", "")}` === location.hash);
  if (tab) selectTab(tab);
}
selectTabFromHash();
window.addEventListener("hashchange", selectTabFromHash);

// ---------------------------------------------------------------------------
// Results

function showResult(prefix, { verdict, kind, output, millis }) {
  const box = $(`${prefix}-result`);
  box.hidden = false;
  box.dataset.kind = kind;
  box.querySelector(".verdict").textContent = verdict;
  box.querySelector(".timing").textContent = millis === undefined ? "" : `${millis.toFixed(0)} ms`;
  // `output` is text, or an element (a table) built from the report.
  const pre = box.querySelector(".output");
  const table = box.querySelector(".table-output");
  const isElement = output instanceof Element;
  pre.hidden = isElement;
  if (table) {
    table.hidden = !isElement;
    table.replaceChildren(...(isElement ? [output] : []));
  }
  if (!isElement) pre.textContent = plain(output);
}

let crashed = false;

function showError(prefix, e) {
  // A panic in Rust aborts the WebAssembly module, which can't be used again.
  if (e instanceof WebAssembly.RuntimeError) {
    crashed = true;
    setStatus("error", "rudof stopped working: reload the page to continue");
    for (const b of document.querySelectorAll("button.primary")) b.disabled = true;
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
  const to = label("rdf-result-format").replace(" (diagram source)", "");
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
  const output = session("rdf").serializeData($("rdf-result-format").value);
  // The verdict names the output format, which may have just changed.
  last.rdf.verdict = rdfVerdict(last.rdf.triples);
  return output;
}

const validators = {
  shex: { validate: validateShex, serialize: serializeShex },
  shacl: { validate: validateShacl, serialize: serializeShacl },
  pgschema: { validate: validatePgschema, serialize: serializePgschema },
  rdf: { validate: convertRdf, serialize: serializeRdf },
};

// RDF tab: copy the result, or use it as the new input.
function updateRdfButtons() {
  const converted = Boolean(last.rdf) && !crashed;
  $("rdf-copy").disabled = !converted;
  $("rdf-use-as-input").disabled = !converted || $("rdf-result-format").value === "plantuml";
}

$("rdf-copy").addEventListener("click", async () => {
  const button = $("rdf-copy");
  try {
    await navigator.clipboard.writeText($("rdf-result").querySelector(".output").textContent);
    button.textContent = "Copied";
  } catch {
    button.textContent = "Copy failed";
  }
  setTimeout(() => (button.textContent = "Copy"), 1500);
});

$("rdf-use-as-input").addEventListener("click", () => {
  $("rdf-data").value = $("rdf-result").querySelector(".output").textContent;
  $("rdf-data-format").value = $("rdf-result-format").value;
  $("rdf-data").focus();
});

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
  updateRdfButtons();
}

function reformat(prefix) {
  if (!last[prefix] || crashed) return;
  try {
    const output = validators[prefix].serialize();
    showResult(prefix, { ...last[prefix], output });
  } catch (e) {
    showError(prefix, e);
  }
  updateRdfButtons();
}

for (const prefix of Object.keys(validators)) {
  $(`${prefix}-validate`).addEventListener("click", () => run(prefix));
  $(`${prefix}-result-format`).addEventListener("change", () => reformat(prefix));
  $(`panel-${prefix}`).addEventListener("keydown", (e) => {
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
  setStatus("ready", `rudof ${new rudofModule.Rudof().getVersion()} ready`);
  for (const b of document.querySelectorAll("button.primary")) b.disabled = false;
} catch (e) {
  console.error(e);
  setStatus("error", `Could not load rudof: ${e.message ?? e}`);
}
