# Small corpus

The small corpus exists to exercise individual ShEx features in isolation.

## Source: shexSpec/shexTest

All 10 cases are vendored from the [shexSpec/shexTest](https://github.com/shexSpec/shexTest) W3C validation test suite.

## Cases and coverage

| Case | Feature exercised |
|---|---|
| `1dot` | Minimal triple pattern (`<S1> { <p1> . }`) |
| `1iri` | NodeKind `IRI` |
| `1literalLength` | String facet `LENGTH` |
| `1literalPattern` | String facet `PATTERN` (regex) |
| `1card2` | Exact cardinality `{2}` |
| `1val1IRIREF` | Value set of IRIs |
| `1NOTIRI` | Negation (`NOT IRI`) |
| `1dotClosed` | `CLOSED` shape |
| `1dotRefOR3` | `OR` over shape references (multi-shape) |
| `recursion_example` | Recursive shape (`@<S>*`) with multi-node shapemap |
