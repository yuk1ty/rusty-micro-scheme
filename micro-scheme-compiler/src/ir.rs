use crate::ast::{SExpr, Token};

pub enum Ir {
    Ldc(Ldc),
    Ldg(Ldg),
}

pub struct Ldc {
    pub expr: Box<SExpr>,
}

pub struct Ldg {
    // TODO only Token::Symbol comes, prepare the specific type
    // to represent that
    pub expr: Token,
}

pub fn ir_codegen(_expr: SExpr) -> Vec<Ir> {
    todo!()
}
