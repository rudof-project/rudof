# Large corpus

The large corpus exists to stress the pipeline with a real-world ShEx schema.

## Source: FHIR R5

The schema used is the [HL7 FHIR R5](https://www.hl7.org/fhir/downloads.html) ShEx representation.

The upstream file (from the official FHIR R5 release) requires two small adjustments to be self-contained:
1. **`IMPORT <Base.shex>` removed.** 
2. **`Base.shex`definitions inlined**:
    - `<Base>`
    - `<SimpleQuantity>`

Two variants are provided:

- **`fhir-r5-inlined.shex`**: the full schema in a single file.
- **`fhir-r5-split/`**: the same schema split across 7 files connected via `IMPORT`, exercising the import-resolution path.
