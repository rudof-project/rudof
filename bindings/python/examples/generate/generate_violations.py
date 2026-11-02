"""Break a generated graph on purpose and read the report of what a validator must flag.

From the PG-Schema of ``rudof_generate/examples/schemas/equivalence/references.pgs``, a
fifth of the entities receive one violation each. The generator writes two reports for
the same broken graph: one for the property graph, where a broken organisation fails the
edges that require it to conform, and one for its RDF reading, where it also fails the
persons who work for it. A validator of each language must report exactly the failing
nodes (and edges) of its report.
"""

import json
from collections import Counter
from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import DataGenerator, GeneratorConfig, OutputFormat, PgSchemaFormat, RDFFormat, Rudof, SchemaFormat

SCHEMA = (
    Path(__file__).resolve().parents[4]
    / "rudof_generate"
    / "examples"
    / "schemas"
    / "equivalence"
    / "references.pgs"
)


def kinds(path: Path) -> Counter[str]:
    return Counter(json.loads(line)["kind"] for line in path.read_text(encoding="utf-8").splitlines() if line)


def main() -> None:
    with TemporaryDirectory() as tmp_dir:
        tmp = Path(tmp_dir)
        config = GeneratorConfig()
        config.set_entity_count(300)
        config.set_seed(11)
        config.set_write_stats(False)
        config.set_output_format(OutputFormat.Pg)
        config.set_output_path(tmp / "data.pg")
        config.set_shapemap_path(tmp / "data.map")
        config.set_rdf_output(tmp / "data.ttl", tmp / "data.shapemap")
        config.set_violations(0.2, tmp / "labels.jsonl")
        config.set_rdf_labels(tmp / "rdf_labels.jsonl")
        DataGenerator(config).run_with_format(SCHEMA, SchemaFormat.PgSchema)

        pg, rdf = kinds(tmp / "labels.jsonl"), kinds(tmp / "rdf_labels.jsonl")
        print(f"property graph report: direct={pg['direct']} induced={pg['induced']}")
        print(f"RDF report: direct={rdf['direct']} induced={rdf['induced']}")

        expected = {
            json.loads(line)["node"]
            for line in (tmp / "labels.jsonl").read_text(encoding="utf-8").splitlines()
            if line and json.loads(line)["kind"] != "trap"
        }
        with Rudof() as rudof:
            rudof.read_data((tmp / "data.pg").read_text(encoding="utf-8"), RDFFormat.Pg)
            rudof.read_pgschema(SCHEMA.read_text(encoding="utf-8"), PgSchemaFormat.PgSchemaC)
            rudof.read_typemap((tmp / "data.map").read_text(encoding="utf-8"))
            reported = {v.node_id for v in rudof.validate_pgschema().violations}
        print(f"PG-Schema validator reports exactly the report: {reported == expected}")


if __name__ == "__main__":
    main()
