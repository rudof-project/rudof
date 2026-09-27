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

const initial = tabs.find((t) => `#${t.id.replace("tab-", "")}` === location.hash);
if (initial) selectTab(initial);

// ---------------------------------------------------------------------------
// Results

function showResult(prefix, { verdict, kind, output, millis }) {
  const box = $(`${prefix}-result`);
  box.hidden = false;
  box.dataset.kind = kind;
  box.querySelector(".verdict").textContent = verdict;
  box.querySelector(".timing").textContent = millis === undefined ? "" : `${millis.toFixed(0)} ms`;
  box.querySelector(".output").textContent = plain(output);
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

function count(n, word) {
  return `${n} ${word}${n === 1 ? "" : "s"}`;
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

const validators = {
  shex: { validate: validateShex, serialize: serializeShex },
  shacl: { validate: validateShacl, serialize: serializeShacl },
};

// The last successful validation of each tab, so that changing the result
// format serializes it again without validating again.
const last = {};

function run(prefix) {
  if (!rudofModule || crashed) return;
  const { validate, serialize } = validators[prefix];
  try {
    const summary = validate();
    last[prefix] = summary;
    showResult(prefix, { ...summary, output: serialize() });
  } catch (e) {
    delete last[prefix];
    showError(prefix, e);
  }
}

function reformat(prefix) {
  if (!last[prefix] || crashed) return;
  try {
    showResult(prefix, { ...last[prefix], output: validators[prefix].serialize() });
  } catch (e) {
    showError(prefix, e);
  }
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
