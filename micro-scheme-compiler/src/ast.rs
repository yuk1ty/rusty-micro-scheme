/// Inner data structure to represent Atom.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Represents numbers like `42`.
    Integer(i64),
    /// Represents floats like `3.14`.
    Float(f64),
    /// Represents symbols.
    Symbol(String),
    /// Represents string literals.
    String(String),
    /// Represents boolean.
    Bool(bool),
}

/// Represents S-expression.
#[derive(Debug, Clone, PartialEq)]
pub enum SExpr {
    Atom(Token),
    List(Vec<SExpr>),
    Quote(Box<SExpr>),
}

impl SExpr {
    /// Convenient constructor for `SExpr::Atom(Token::Symbol)`.
    pub fn symbol(op: String) -> SExpr {
        SExpr::Atom(Token::Symbol(op))
    }

    /// Convenient construct for `SExpr::Atom(Token::String)`.
    pub fn string(op: String) -> SExpr {
        SExpr::Atom(Token::String(op))
    }

    /// Convenient constructor for `SExpr::Atom(Token::Integer)`.
    pub fn integer(op: i64) -> SExpr {
        SExpr::Atom(Token::Integer(op))
    }

    /// Convenient constructor for `SExpr::Atom(Token::Float)`.
    pub fn float(op: f64) -> SExpr {
        SExpr::Atom(Token::Float(op))
    }

    /// Convenient constructor for `SExpr::Atom(Token::Bool)`
    pub fn bool(op: char) -> SExpr {
        SExpr::Atom(Token::Bool(op == 't'))
    }
}
