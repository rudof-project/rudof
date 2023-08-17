use std::{collections::HashMap, fmt::{Display, Formatter}};
use shex_ast::*;
use srdf::Object;

#[derive(Debug, Clone)]
pub struct ResultMap 
where 
{
  result_map: HashMap<Object, ShapeLabelIdx>
}

impl ResultMap
{
    pub fn new() -> ResultMap {
        ResultMap { result_map: HashMap::new() }
    }

    pub fn insert(&mut self, node: Object, shape: ShapeLabelIdx) {
        self.result_map.insert(node, shape);
    }
}

impl Display for ResultMap
{
    fn fmt(&self, dest: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        for (n,s) in &self.result_map {
            write!(dest, "{n}->{s}")?;
        };
        Ok(())
    }
}
