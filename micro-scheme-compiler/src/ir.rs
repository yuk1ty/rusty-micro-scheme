use anyhow::Result;

use crate::ast::{SExpr, Token};

#[derive(Debug, PartialEq)]
pub enum Ir {
    /// Represents `ldc` instruction. This piles quote/self-evaluated form on stack.
    Ldc(Ldc),
    /// Represents `ldg` instruction. This piles the global variable symbols on stack.
    Ldg(Ldg),
    /// Stop the execution and return the value on top of the stack.
    Stop,
}

#[derive(Debug, PartialEq)]
pub struct Ldc {
    pub expr: Box<SExpr>,
}

#[derive(Debug, PartialEq)]
pub struct Ldg {
    // TODO only Token::Symbol comes, prepare the specific type
    // to represent that
    pub expr: Token,
}

pub fn ir_codegen(expr: SExpr) -> Result<Vec<Ir>> {
    let mut ir_generated = Vec::new();
    eval(expr, &mut ir_generated)?;
    ir_generated.push(Ir::Stop);
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
        expr @ SExpr::Atom(_) => {
            ir_generated.push(Ir::Ldc(Ldc {
                expr: Box::new(expr),
            }));
        }
        SExpr::List(list) => {
            for expr in list {
                eval(expr, ir_generated)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_self_evaluation_form_as_ldc() {
        let expr = SExpr::integer(1);
        let ir = ir_codegen(expr).unwrap();
        assert_eq!(
            ir,
            vec![
                Ir::Ldc(Ldc {
                    expr: Box::new(SExpr::integer(1))
                }),
                Ir::Stop
            ]
        );
    }

    #[test]
    fn stack_simple_quote_as_ldc() {
        let expr = SExpr::Quote(Box::new(SExpr::symbol("a".to_string())));
        let ir = ir_codegen(expr).unwrap();
        assert_eq!(
            ir,
            vec![
                Ir::Ldc(Ldc {
                    expr: Box::new(SExpr::symbol("a".to_string()))
                }),
                Ir::Stop
            ]
        );
    }
}
