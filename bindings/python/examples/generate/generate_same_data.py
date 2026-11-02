"""Generate the same data from equivalent ShEx, SHACL and PG-Schema schemas.

The three schemas of ``rudof_generate/examples/schemas/equivalence/references`` describe
the same graph (a person works for an organisation located in a city). With the same
seed, the generator writes the same triples from each; the PG-Schema run also writes the
graph as a property graph, and every output is valid against its schema.
"""

from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import (
    DataGenerator,
    GeneratorConfig,
    OutputFormat,
    PgSchemaFormat,
    RDFFormat,
    ResultDataFormat,
    Rudof,
    SchemaFormat,
    ShaclFormat,
    ShaclValidationMode,
)

SCHEMAS = (
    Path(__file__).resolve().parents[4] / "rudof_generate" / "examples" / "schemas" / "equivalence"
)


def generate(schema: Path, fmt: SchemaFormat, out: Path, pg: bool = False) -> None:
    config = GeneratorConfig()
    config.set_entity_count(30)
    config.set_seed(7)
    config.set_write_stats(False)
    if pg:
        config.set_output_format(OutputFormat.Pg)
        config.set_output_path(out.with_suffix(".pg"))
        config.set_shapemap_path(out.with_suffix(".map"))
        config.set_rdf_output(out)
    else:
        config.set_output_format(OutputFormat.NTriples)
        config.set_output_path(out)
    DataGenerator(config).run_with_format(schema, fmt)


def triples(path: Path) -> set[str]:
    """The triples of an RDF file, as N-Triples lines."""
    with Rudof() as rudof:
        rudof.read_data(path.read_text(encoding="utf-8"), RDFFormat.Turtle)
        ntriples = rudof.serialize_data(ResultDataFormat.NTriples)
    return {line for line in ntriples.splitlines() if line.strip()}


def main() -> None:
    with TemporaryDirectory() as tmp_dir:
        tmp = Path(tmp_dir)
        generate(SCHEMAS / "references.pgs", SchemaFormat.PgSchema, tmp / "pg.ttl", pg=True)
        generate(SCHEMAS / "references.shex", SchemaFormat.ShEx, tmp / "shex.nt")
        generate(SCHEMAS / "references.ttl", SchemaFormat.Shacl, tmp / "shacl.nt")

        base = triples(tmp / "pg.ttl")
        print(f"triples: {len(base)}")
        print(f"ShEx graph equals PG-Schema graph: {triples(tmp / 'shex.nt') == base}")
        print(f"SHACL graph equals PG-Schema graph: {triples(tmp / 'shacl.nt') == base}")

        with Rudof() as rudof:
            rudof.read_data((tmp / "pg.pg").read_text(encoding="utf-8"), RDFFormat.Pg)
            rudof.read_pgschema((SCHEMAS / "references.pgs").read_text(encoding="utf-8"), PgSchemaFormat.PgSchemaC)
            rudof.read_typemap((tmp / "pg.map").read_text(encoding="utf-8"))
            print(f"property graph valid (PG-Schema): {rudof.validate_pgschema().conforms}")

        with Rudof() as rudof:
            rudof.read_shacl((SCHEMAS / "references.ttl").read_text(encoding="utf-8"), ShaclFormat.Turtle)
            rudof.read_data((tmp / "pg.ttl").read_text(encoding="utf-8"), RDFFormat.Turtle)
            print(f"RDF graph valid (SHACL): {rudof.validate_shacl(ShaclValidationMode.Native).conforms}")


if __name__ == "__main__":
    main()
