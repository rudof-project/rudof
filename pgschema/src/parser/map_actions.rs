use super::map::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type IDENTIFIER = String;
pub fn identifier(_ctx: &Ctx, token: Token) -> IDENTIFIER {
    token.value.into()
}
pub type Map = Association1;
pub fn map_association1(_ctx: &Ctx, association1: Association1) -> Map {
    association1
}
pub type Association1 = Vec<Association>;
pub fn association1_c1(
    _ctx: &Ctx,
    mut association1: Association1,
    association: Association,
) -> Association1 {
    association1.push(association);
    association1
}
pub fn association1_association(_ctx: &Ctx, association: Association) -> Association1 {
    vec![association]
}
#[derive(Debug, Clone)]
pub struct Association {
    pub node_id: NodeId,
    pub notopt: NOTOpt,
    pub type_name: TypeName,
}
pub fn association_c1(
    _ctx: &Ctx,
    node_id: NodeId,
    notopt: NOTOpt,
    type_name: TypeName,
) -> Association {
    Association {
        node_id,
        notopt,
        type_name,
    }
}
pub type NOTOpt = Option<NotOptNoO>;
#[derive(Debug, Clone)]
pub enum NotOptNoO {
    NOT,
}
pub fn notopt_not(_ctx: &Ctx) -> NOTOpt {
    Some(NotOptNoO::NOT)
}
pub fn notopt_empty(_ctx: &Ctx) -> NOTOpt {
    None
}
pub type NodeId = IDENTIFIER;
pub fn node_id_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> NodeId {
    identifier
}
pub type TypeName = IDENTIFIER;
pub fn type_name_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> TypeName {
    identifier
}
