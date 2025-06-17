use crate::ast::primitives::Primitives;

pub struct VarDecl {
    pub identifier_name: String,
    pub type_: Primitives,
}
