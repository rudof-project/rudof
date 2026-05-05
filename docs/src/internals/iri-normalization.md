# IRI normalization

Several commands (`node`, `shex-validate`) and MCP tools (`node_info`, `validate_shex`) accept a node or shape label as a plain string. Internally these strings must be parsed by the ShapeMap parser, which requires IRIs to be enclosed in angle brackets: `<http://example.org/Alice>`.

To avoid burdening users with this detail, `rudof` applies a normalization step before handing the string to the parser. Two modes are available, controlled by the `--strict-iris` CLI flag or the `strict_iris` MCP field.

## Lax mode (default)

A string is treated as a bare IRI and wrapped in `<>` if all four conditions hold:

1. It does not already start with `<`.
2. It does not start with `_:` (blank node).
3. It does not start with `{` (triple-pattern node selector such as `{FOCUS a :Person}`).
4. It contains the substring `://`.

All other strings (prefixed names, already-bracketed IRIs) are passed through unchanged.

### Known failure cases

The `://` check is a heuristic. There are RFC 3987 absolute IRI schemes that do not contain `://`:

| Scheme | Example | What happens |
|---|---|---|
| `urn:` | `urn:isbn:0451450523` | Not wrapped → parse error |
| `mailto:` | `mailto:user@example.com` | Not wrapped → parse error |
| `data:` | `data:text/plain;base64,SGVsbG8=` | Not wrapped → parse error |

There is also a false-positive risk: a prefixed local name whose local part contains `://` (e.g. `ex:path//resource`) would be incorrectly treated as a full IRI.

For any of these inputs, users must wrap the IRI explicitly: `<urn:isbn:0451450523>`.

## Strict mode

No normalization is applied. Every IRI must be passed with explicit angle brackets. A bare IRI produces a clear parser error, making the problem visible immediately rather than producing a silent wrong result.

## Planned improvements

The `://` heuristic is a known limitation. Planned future work includes:

- Replace the heuristic with a proper RFC 3987 absolute IRI detector that recognises all registered schemes regardless of whether they use an authority component (and therefore `://`).
- Deprecate lax mode once the improved detector is in place, and eventually make strict mode the default.

Contributions towards these improvements are welcome. The relevant code lives in `rudof_lib/src/utils/utils.rs` (`normalize_iri_str`) and `rudof_lib/src/formats/node.rs` (`IriNormalizationMode`).
