use shex_compact::ShExParser;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("examples/schema.shex");
    let schema = ShExParser::parse_buf(&path, None)?;
    
    println!("Schema: {:?}", schema);
    
    if let Some(shapes) = schema.shapes() {
        println!("Found {} shapes", shapes.len());
        for shape in shapes {
            println!("Shape: {:?}", shape.id);
            println!("Shape expression: {:?}", shape.shape_expr);
        }
    } else {
        println!("No shapes found");
    }
    
    Ok(())
}
