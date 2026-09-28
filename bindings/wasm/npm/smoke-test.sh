#!/usr/bin/env bash
# Packs pkg/ (see build.sh) with `npm pack`, installs the tarball in an empty
# project and uses it through CommonJS, ES modules and the web build, as the
# published package would be used. Prints the path of the tarball last.
set -euo pipefail

cd "$(dirname "$0")/.."
out="$(mktemp -d)"
tarball="$(cd pkg && npm pack --silent --pack-destination "$out")"
tarball="$out/$tarball"

project="$(mktemp -d)"
cd "$project"
npm init -y > /dev/null
npm install --silent --no-audit --no-fund "$tarball"

cat > data.js <<'EOF'
exports.data = `prefix : <http://example.org/>
:alice :name "Alice" ; :age 23 .
:bob   :name "Bob"   ; :age "unknown" .`;
exports.shex = `prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string ; :age xsd:integer }`;
exports.shapemap = ":alice@:Person, :bob@:Person";
exports.shacl = `prefix : <http://example.org/>
prefix sh: <http://www.w3.org/ns/shacl#>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:PersonShape a sh:NodeShape ;
  sh:targetSubjectsOf :name ;
  sh:property [ sh:path :age ; sh:datatype xsd:integer ] .`;
exports.check = (label, shex, shacl) => {
  const statuses = shex.entries.map((e) => `${e.node.split("/").pop()}=${e.status}`).sort();
  if (shex.conforms || statuses.join() !== "alice=conformant,bob=nonconformant" || shacl.conforms) {
    throw new Error(`${label}: unexpected results ${JSON.stringify({ shex, shacl })}`);
  }
  console.log(`${label}: ok`);
};
EOF

# CommonJS (Node.js build)
cat > cjs.cjs <<'EOF'
const { validateShex, validateShacl, Rudof } = require("@rudof/rudof");
const { data, shex, shapemap, shacl, check } = require("./data.js");
check("require", validateShex(data, shex, shapemap), validateShacl(data, shacl));
if (!/^\d+\.\d+\.\d+/.test(new Rudof().getVersion())) throw new Error("getVersion");
EOF
node cjs.cjs

# ES modules (Node.js build)
cat > esm.mjs <<'EOF'
import { validateShex, validateShacl } from "@rudof/rudof";
import inputs from "./data.js";
const { data, shex, shapemap, shacl, check } = inputs;
check("import", validateShex(data, shex, shapemap), validateShacl(data, shacl));
EOF
node esm.mjs

# The web build, as browsers and bundlers load it (here with the .wasm bytes)
cat > web.mjs <<'EOF'
import { readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import init, { validateShex, validateShacl } from "@rudof/rudof/web";
import inputs from "./data.js";
const require = createRequire(import.meta.url);
await init({ module_or_path: await readFile(require.resolve("@rudof/rudof/rudof_wasm_bg.wasm")) });
const { data, shex, shapemap, shacl, check } = inputs;
check("web", validateShex(data, shex, shapemap), validateShacl(data, shacl));
EOF
node web.mjs

# TypeScript declarations are where package.json says
for f in node/rudof_wasm.d.ts web/rudof_wasm.d.ts; do
    test -f "node_modules/@rudof/rudof/$f" || { echo "missing $f" >&2; exit 1; }
done

echo "$tarball"
