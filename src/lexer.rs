use std::fmt;
use chumsky::prelude::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Token {
    Num(String), // We use string instead of f64 b/c f64 isn't hashable
    Ident(String),
    Minus,
    Plus,
    Times,
    Div,
    Fn, // keyword
    Let,
    OpenParen,
    CloseParen,
    Eq,
    Comma,
    Semi,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Num(num) => write!(f, "{}", num),
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Times => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::Eq => write!(f, "="),
            Token::Comma => write!(f, ","),
            Token::Semi => write!(f, ";"),
        }
    }
}

// TODO: add a span in the future
pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    // TODO: support parsing numbers that start with '.'
    // let real = text::int(10)
    //     .chain(just('.'))
    //     .chain::<char, _, _>(text::digits(10))
    //     .collect::<String>()
    //     .map(|s: String| Token::Num(s.parse().unwrap()));

    // let nat = text::int(10)
    //     .chain(just('.'))
    //     .chain::<char, _, _>(text::digits(10))
    //     .collect::<String>()
    //     .map(|s: String| Token::Num(s.parse().unwrap()));

    let int = text::int::<char, Simple<char>>(10)
        .map(|s: String| Token::Num(s.parse().unwrap()));

    let real = text::int(10)
        .chain(just('.'))
        .chain::<char, _, _>(text::digits(10))
        .collect::<String>()
        .map(|s: String| Token::Num(s.parse().unwrap()));

    let op = choice((
        just('+').to(Token::Plus),
        just('-').to(Token::Minus),
        just('*').to(Token::Times),
        just('/').to(Token::Div),
    )).padded();

    let punct = choice((
        just(',').to(Token::Comma),
        just("=").to(Token::Eq),
        just('(').to(Token::OpenParen),
        just(')').to(Token::CloseParen),
        just(';').to(Token::Semi),
    )).padded();

    let ident = text::ident::<char, Simple<char>>().padded();

    let word = ident.map(|s: String| match s.as_str() {
        "fn" => Token::Fn,
        "let" => Token::Let,
        _ => Token::Ident(s.clone())
    });

    let token = choice((word, real, int, op, punct));

    token.repeated().padded().then_ignore(end())
}

// TODO: prevent the parsing of keywords as identifiers

// TODO: implement a parser that generates tokens from chars
// pub fn parser() -> impl Parser<char, Token
