use std::{fmt::Debug, iter::Peekable};

use crate::prelude::IteratorExt;

#[derive(Debug, PartialEq)]
pub enum Token {
    /// Represents numbers like `42`.
    Integer(i64),
    /// Represents `(`.
    LParen,
    /// Represents `)`.
    RParen,
    /// Represents atom.
    Atom(char),
    /// Represents symbols.
    Symbol(String),
}

impl Token {
    fn new(ch: char) -> Token {
        match ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            x => Token::Atom(x),
        }
    }
}

pub fn tokenize<I>(iter: &mut Peekable<I>) -> Vec<Token>
where
    I: Iterator<Item = char>,
{
    while consume_whitespace(iter) {
        continue;
    }

    let mut tokens = Vec::new();
    while let Some(token) = proceed_one(iter) {
        tokens.push(token);
    }
    tokens
}

fn proceed_one<I>(iter: &mut Peekable<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    consume_lparen(iter)
        .or_else(|| consume_rparen(iter))
        .or_else(|| consume_symbol(iter))
}

fn consume_whitespace<I>(iter: &mut Peekable<I>) -> bool
where
    I: Iterator<Item = char>,
{
    if is_expected(iter, ' ') || is_expected(iter, '\n') || is_expected(iter, '\t') {
        iter.next();
        true
    } else {
        false
    }
}

fn is_expected<I>(iter: &mut Peekable<I>, op: char) -> bool
where
    I: Iterator<Item = char>,
{
    if let Some(&x) = iter.peek() {
        x == op
    } else {
        false
    }
}

fn consume_lparen<I>(iter: &mut Peekable<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    consume(iter, '(')
}

fn consume_rparen<I>(iter: &mut Peekable<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    consume(iter, ')')
}

fn consume_symbol<I>(iter: &mut Peekable<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    if iter.peek().is_none() {
        return None;
    }
    let value = IteratorExt::take_while(iter, |c| *c != ')').collect::<String>();
    consume_number(value.as_str()).or_else(|| Some(Token::Symbol(value)))
}

fn consume_number(value: &str) -> Option<Token> {
    value.parse::<i64>().map(Token::Integer).ok()
}

fn consume<I>(iter: &mut Peekable<I>, op: char) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    if !is_expected(iter, op) {
        return None;
    }
    iter.next();
    Some(Token::new(op))
}

#[cfg(test)]
mod tests {
    use crate::lexer::{tokenize, Token};

    #[test]
    fn empty_paren() {
        assert_eq!(
            tokenize(&mut "()".chars().peekable()),
            vec![Token::LParen, Token::RParen]
        );
    }

    #[test]
    fn nested_paren() {
        assert_eq!(
            tokenize(&mut "((42))".chars().peekable()),
            vec![
                Token::LParen,
                Token::LParen,
                Token::Integer(42),
                Token::RParen,
                Token::RParen
            ]
        );
    }

    #[test]
    fn return_42() {
        assert_eq!(
            tokenize(&mut "42".chars().peekable()),
            vec![Token::Integer(42)]
        );
        assert_eq!(
            tokenize(&mut "(42)".chars().peekable()),
            vec![Token::LParen, Token::Integer(42), Token::RParen]
        );
    }

    #[test]
    fn return_symbols() {
        assert_eq!(
            tokenize(&mut "(a)".chars().peekable()),
            vec![Token::LParen, Token::Symbol("a".to_string()), Token::RParen]
        );
    }
}
