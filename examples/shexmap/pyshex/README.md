# PyShEx ShExMap examples

Same manifest format as `../examples` (copied from shex.js), plus
`alternativeOutputDataURLs`: every output the input's distinct parses lead to, in the
order `bind_all` finds them.

| Example | Shows |
|---|---|
| ambiguous BP | Both of a reading's components match either constraint, so validation succeeds two ways and the systolic and diastolic values can swap. `bind` returns the first parse and reports `alternatives == 2`; `bind(strict=True)` refuses. |
| coded BP | The same reading with a code on each component: one parse, whatever the data order. |
| greedy trap | `:code .` before `:code ["primary"]`: giving each triple to the first constraint it fits takes "primary" for the wildcard and strands the second constraint. The partition search finds the only valid assignment. |
| inverse in | Observations point at their patient (`obs :subject patient`); the input schema starts from the patient and follows `^:subject` backwards. Each observation is one frame of bindings. |
| inverse out | The reverse trip: an inverse constraint in the output schema makes each new observation point at the patient. |
| EXTENDS | Abstract `Person` extended by `Patient` and `Clinician` in the input, and abstract `Entry` extended by `PatientEntry` and `StaffEntry` in the output. Binding follows the extension each member satisfies; materializing, the threads try both kinds of entry for every member and keep the one that uses that member's bindings. |
