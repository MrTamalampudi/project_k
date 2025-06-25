use crate::ast::primitives::Primitives;

pub struct VarDecl {
    pub name: String,
    pub type_: Primitives,
    pub value: IdentifierValue,
}

#[derive(Debug, Clone)]
pub enum IdentifierValue {
    String(String),
    Number(isize),
    Xpath(String),
    None,
}
