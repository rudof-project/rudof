use rustemo::Parser;

use crate::{
    parser::{map::MapParser, map_actions::Association},
    pgs_error::PgsError,
    type_map::{Association as PGAssociation, TypeMap},
};

pub struct MapBuilder {}

impl MapBuilder {
    pub fn new() -> Self {
        MapBuilder {}
    }
    pub fn parse_map(&self, input: &str) -> Result<TypeMap, PgsError> {
        let map_content = MapParser::new()
            .parse(input)
            .map_err(|e| PgsError::MapParserError {
                error: e.to_string(),
            })?;
        let mut type_map = TypeMap::new();
        get_type_map(map_content, &mut type_map)?;
        Ok(type_map)
    }
}

fn get_type_map(associations: Vec<Association>, type_map: &mut TypeMap) -> Result<(), PgsError> {
    for association in associations {
        get_association(association, type_map)?;
    }
    Ok(())
}

fn get_association(ass: Association, type_map: &mut TypeMap) -> Result<(), PgsError> {
    let node_id = ass.node_id;
    let type_name = ass.type_name;
    if let Some(_) = ass.notopt {
        type_map.add_association(PGAssociation::new(node_id, type_name).with_no_conform());
    } else {
        type_map.add_association(PGAssociation::new(node_id, type_name));
    }
    Ok(())
}
