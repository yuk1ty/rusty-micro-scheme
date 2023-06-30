use std::rc::Rc;

use anyhow::Result;

use crate::ast::{SExpr, Token};

#[derive(Debug)]
pub enum Ir {
    /// Represents `ldc` instruction. This piles quote/self-evaluated form on stack.
    Ldc(Ldc),
    /// Represents `ldg` instruction. This piles the global variable symbols on stack.
    Ldg(Ldg),
    /// Represents tokens excluding symbols.
    Raw(Token),
}

#[derive(Debug)]
pub struct Ldc {
    pub expr: Rc<SExpr>,
}

#[derive(Debug)]
pub struct Ldg {
    // TODO only Token::Symbol comes, prepare the specific type
    // to represent that
    pub expr: Token,
}

pub fn ir_codegen(expr: SExpr) -> Result<Vec<Ir>> {
    let mut ir_generated = Vec::new();
    eval(expr, &mut ir_generated)?;
    Ok(ir_generated)
}

fn eval(expr: SExpr, ir_generated: &mut Vec<Ir>) -> Result<()> {
    match expr {
        SExpr::Quote(expr) => {
            ir_generated.push(Ir::Ldc(Ldc { expr }));
        }
        SExpr::Atom(Token::Symbol(s)) => {
            ir_generated.push(Ir::Ldg(Ldg {
                expr: Token::Symbol(s),
            }));
        }
        SExpr::Atom(token) => {
            ir_generated.push(Ir::Raw(token));
        }
        SExpr::List(list) => {
            for expr in list {
                eval(expr, ir_generated)?;
            }
        }
    }
    Ok(())
}
