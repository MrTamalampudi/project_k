use crate::ast::{getter::Getter, teststep::TestStep, var_decl::VarDecl};

#[derive(Clone, Debug)]
pub enum TranslatorStack {
    TestStep(TestStep),
    Getter(Getter),
    VarDecl(VarDecl),
    String(String),
    Ident(String),
}
