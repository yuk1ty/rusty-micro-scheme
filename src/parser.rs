use chumsky::prelude::*;

/// Inner data structure to represent Atom.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Represents numbers like `42`.
    Integer(i64),
    /// Represents symbols.
    Symbol(String),
}

/// Represents S-expression.
#[derive(Debug, Clone, PartialEq)]
pub enum SExpr {
    Atom(Token),
    List(Vec<SExpr>),
}

impl SExpr {
    /// Convenient constructor for `SExpr::Atom(Token::Symbol)`.
    pub fn symbol(op: String) -> SExpr {
        SExpr::Atom(Token::Symbol(op))
    }

    /// Convenient constructor for `SExpr::Atom(Token::Integer)`.
    pub fn integer(op: i64) -> SExpr {
        SExpr::Atom(Token::Integer(op))
    }
}

pub fn parser() -> impl Parser<char, SExpr, Error = Simple<char>> {
    recursive(|sexpr| {
        sexpr
            .padded()
            .repeated()
            .map(SExpr::List)
            .delimited_by(just('('), just(')'))
            .or(symbol())
            .or(num())
    })
}

fn symbol() -> impl Parser<char, SExpr, Error = Simple<char>> {
    filter::<_, _, Simple<char>>(|c: &char| c.is_alphabetic())
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(SExpr::symbol)
}

fn num() -> impl Parser<char, SExpr, Error = Simple<char>> {
    text::int(10).from_str().unwrapped().map(SExpr::integer)
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use super::*;

    #[test]
    fn empty_paren() {
        assert_eq!(parser().parse("()").unwrap(), SExpr::List(vec![]));
    }

    #[test]
    fn nested_paren() {
        assert_eq!(
            parser().parse("((42))").unwrap(),
            SExpr::List(vec![SExpr::List(vec![SExpr::Atom(Token::Integer(42))])])
        );
    }

    #[test]
    fn simple_numbers() {
        assert_eq!(
            parser().parse("42").unwrap(),
            SExpr::Atom(Token::Integer(42))
        );
        assert_eq!(
            parser().parse("(42)").unwrap(),
            SExpr::List(vec![SExpr::Atom(Token::Integer(42))])
        );
    }

    #[test]
    fn symbols() {
        assert_eq!(
            parser().parse("(a)").unwrap(),
            SExpr::List(vec![SExpr::Atom(Token::Symbol("a".to_string()))])
        );
    }

    #[test]
    fn compound_symbols() {
        assert_eq!(
            parser().parse("(a 42 b)").unwrap(),
            SExpr::List(vec![
                SExpr::Atom(Token::Symbol("a".to_string())),
                SExpr::Atom(Token::Integer(42)),
                SExpr::Atom(Token::Symbol("b".to_string()))
            ])
        );
    }
}
