use anyhow::*;
use clap::Parser;
use clientele;
use pgschema::cli::{Cli, Command};
use pgschema::parser::{map_builder::MapBuilder, pg_builder::PgBuilder, pgs_builder::PgsBuilder};
use std::result::Result::Ok;

// src/main.rs
fn main() -> Result<()> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let cli = Cli::parse_from(args);

    match &cli.command {
        Some(Command::Pgs { schema }) => run_pgs(schema),
        Some(Command::Pg { graph }) => run_pg(graph),
        Some(Command::TypeMap { map }) => run_map(map),
        Some(Command::Validate { graph, schema, map }) => run_validate(graph, schema, map),
        None => {
            bail!("Command not specified, type `--help` to see list of commands")
        }
    }
}

fn run_pgs(schema: &str) -> Result<()> {
    let schema = get_schema(schema)?;
    println!("Property graph schema: {}", schema);
    Ok(())
}

fn run_pg(graph: &str) -> Result<()> {
    let pg = get_graph(graph)?;
    println!("Property graph: {}", pg);
    Ok(())
}

fn run_map(map: &str) -> Result<()> {
    let map = get_map(map)?;
    println!("Type map associations:\n{}", map);
    Ok(())
}

fn run_validate(graph_path: &str, schema_path: &str, map_path: &str) -> Result<()> {
    let schema = get_schema(schema_path)?;
    let graph = get_graph(graph_path)?;
    let map = get_map(map_path)?;
    let result = map.validate(&schema, &graph)?;
    println!("Validation result: {}", result);
    Ok(())
}

fn get_schema(path: &str) -> Result<pgschema::pgs::PropertyGraphSchema> {
    let schema_content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read schema file: {}", path))?;
    let schema: pgschema::pgs::PropertyGraphSchema =
        match PgsBuilder::new().parse_pgs(schema_content.as_str()) {
            Ok(schema) => schema,
            Err(e) => {
                bail!("Failed to parse schema: {}", e);
            }
        };
    Ok(schema)
}

fn get_graph(path: &str) -> Result<pgschema::pg::PropertyGraph> {
    let graph_content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read graph file: {}", path))?;
    let graph = match PgBuilder::new().parse_pg(graph_content.as_str()) {
        Ok(graph) => graph,
        Err(e) => {
            bail!("Failed to parse graph: {}", e);
        }
    };
    Ok(graph)
}

fn get_map(path: &str) -> Result<pgschema::type_map::TypeMap> {
    let map_content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read type map file: {}", path))?;
    let map: pgschema::type_map::TypeMap = match MapBuilder::new().parse_map(map_content.as_str()) {
        Ok(map) => map,
        Err(e) => {
            bail!("Failed to parse type map: {}", e);
        }
    };
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adult() {
        test_case(
            "tests/adult.pg",
            "tests/adult.pgs",
            "tests/adult.map",
            "tests/adult.result_map",
        );
    }

    #[test]
    fn user() {
        test_case(
            "tests/user.pg",
            "tests/user.pgs",
            "tests/user.map",
            "tests/user.result_map",
        );
    }

    #[test]
    fn course() {
        test_case(
            "tests/course.pg",
            "tests/course.pgs",
            "tests/course.map",
            "tests/course.result_map",
        );
    }

    #[test]
    fn person() {
        test_case(
            "tests/person.pg",
            "tests/person.pgs",
            "tests/person.map",
            "tests/person.result_map",
        );
    }

    #[test]
    fn product() {
        test_case(
            "tests/product.pg",
            "tests/product.pgs",
            "tests/product.map",
            "tests/product.result_map",
        );
    }

    #[test]
    fn any() {
        test_case(
            "tests/any.pg",
            "tests/any.pgs",
            "tests/any.map",
            "tests/any.result_map",
        );
    }

    #[test]
    fn simple() {
        test_case(
            "tests/simple.pg",
            "tests/simple.pgs",
            "tests/simple.map",
            "tests/simple.result_map",
        );
    }

    #[test]
    fn email() {
        // It checks regexes
        test_case(
            "tests/email.pg",
            "tests/email.pgs",
            "tests/email.map",
            "tests/email.result_map",
        );
    }

    #[test]
    fn edge() {
        // It checks regexes
        test_case(
            "tests/edge.pg",
            "tests/edge.pgs",
            "tests/edge.map",
            "tests/edge.result_map",
        );
    }

    #[test]
    fn basic() {
        // It checks regexes
        test_case(
            "tests/basic.pg",
            "tests/basic.pgs",
            "tests/basic.map",
            "tests/basic.result_map",
        );
    }

    #[test]
    fn datatypes() {
        // It checks datatypes
        test_case(
            "tests/datatypes.pg",
            "tests/datatypes.pgs",
            "tests/datatypes.map",
            "tests/datatypes.result_map",
        );
    }

    #[test]
    fn employee() {
        // It checks simple inheritance
        test_case(
            "tests/employee.pg",
            "tests/employee.pgs",
            "tests/employee.map",
            "tests/employee.result_map",
        );
    }

    fn test_case(pg_file: &str, pgs_file: &str, map_file: &str, expected_map_file: &str) {
        let pg = get_graph(pg_file).unwrap_or_else(|_| panic!("Failed to parse: {pg_file})"));
        let pgs = get_schema(pgs_file).unwrap_or_else(|_| panic!("Failed to parse: {pgs_file})"));
        let type_map = get_map(map_file).unwrap_or_else(|_| panic!("Failed to parse: {map_file})"));
        let expected_result = get_map(expected_map_file)
            .unwrap_or_else(|_| panic!("Failed to parse: {expected_map_file})"));
        let result = type_map.validate(&pgs, &pg).unwrap();
        let comparison = expected_result.compare_with_result(&result).unwrap();
        if comparison.is_empty() {
            // Test passed
        } else {
            panic!(
                "Validation failed: {}",
                comparison
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }
}
