use std::rc::Rc;

use crate::ast::SExpr;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, SExpr, Error = Simple<char>> {
    recursive(|sexpr| {
        sexpr
            .padded()
            .repeated()
            .map(SExpr::List)
            .delimited_by(just('('), just(')'))
            .or(quote())
            .or(atom())
    })
}

fn atom() -> impl Parser<char, SExpr, Error = Simple<char>> {
    symbol().or(string()).or(bool()).or(num())
}

fn string() -> impl Parser<char, SExpr, Error = Simple<char>> {
    just('"')
        .chain(filter(|c: &char| *c != '"').repeated())
        .chain(just('"'))
        .collect::<String>()
        .map(SExpr::string)
        .labelled("parsing strings")
}

fn bool() -> impl Parser<char, SExpr, Error = Simple<char>> {
    just('#')
        .then(just('t').or(just('f')))
        .map(|(_, s)| SExpr::bool(s))
}

fn symbol() -> impl Parser<char, SExpr, Error = Simple<char>> {
    filter::<_, _, Simple<char>>(|c: &char| c.is_alphabetic())
        .labelled("symbol")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(SExpr::symbol)
}

fn num() -> impl Parser<char, SExpr, Error = Simple<char>> {
    let int = text::int(10).map(|s: String| SExpr::integer(s.parse::<i64>().unwrap()));
    let float = text::int(10)
        .chain(just('.'))
        .chain::<char, _, _>(text::digits(10))
        .collect::<String>()
        .map(|s| SExpr::float(s.parse::<f64>().unwrap()));
    choice((float, int))
}

fn quote() -> impl Parser<char, SExpr, Error = Simple<char>> {
    // TODO: The quoted list is insufficient. For example, this can't parse ('(a '(b c))) for now.
    just("'")
        .or(just("quote").padded())
        .ignore_then(recursive(|sexpr| {
            sexpr
                .padded()
                .repeated()
                .map(SExpr::List)
                .delimited_by(just('('), just(')'))
                .or(atom())
        }))
        .map(|s| SExpr::Quote(Rc::new(s)))
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::ast::Token;

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
        assert_eq!(
            parser().parse("(3.14)").unwrap(),
            SExpr::List(vec![SExpr::Atom(Token::Float(3.14))])
        );
    }

    #[test]
    fn strings() {
        assert_eq!(
            parser().parse(r#"("aiueo")"#).unwrap(),
            SExpr::List(vec![SExpr::Atom(Token::String("\"aiueo\"".to_string()))])
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
            parser().parse("(a 42 b \"foo\")").unwrap(),
            SExpr::List(vec![
                SExpr::Atom(Token::Symbol("a".to_string())),
                SExpr::Atom(Token::Integer(42)),
                SExpr::Atom(Token::Symbol("b".to_string())),
                SExpr::Atom(Token::String("\"foo\"".to_string())),
            ])
        );
    }

    #[test]
    fn bool() {
        assert_eq!(
            parser().parse("(#t #f)").unwrap(),
            SExpr::List(vec![
                SExpr::Atom(Token::Bool(true)),
                SExpr::Atom(Token::Bool(false)),
            ])
        );
    }

    #[test]
    fn quote() {
        assert_eq!(
            parser().parse("'(a b c)").unwrap(),
            SExpr::Quote(Rc::new(SExpr::List(vec![
                SExpr::Atom(Token::Symbol("a".to_string())),
                SExpr::Atom(Token::Symbol("b".to_string())),
                SExpr::Atom(Token::Symbol("c".to_string())),
            ])))
        );
    }
}
