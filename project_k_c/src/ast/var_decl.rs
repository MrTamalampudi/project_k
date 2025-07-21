use crate::ast::{getter::Getter, primitives::Primitives, teststep::TestStep};

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub type_: Primitives,
    pub rhs: VarRHS,
}

#[derive(Debug, Clone)]
pub enum VarRHS {
    Getter(Getter),
    Teststep(TestStep),
    String(String),
    Var(String),
}
