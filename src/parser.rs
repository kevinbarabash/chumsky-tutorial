use chumsky::prelude::*;

use super::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Var(String),

    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),

    Call(String, Vec<Expr>),
    Let {
        name: String,
        rhs: Box<Expr>,
        then: Box<Expr>,
    },
    Fn {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
        then: Box<Expr>,
    },
}

pub fn parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    let ident = select! { Token::Ident(ident) => ident.clone() }.labelled("identifier");

    let expr = recursive(|expr| {
        let num = select! {
            Token::Num(n) => Expr::Num(n.parse().unwrap()),
        };

        let call = ident
            .then(
                expr.clone()
                    .separated_by(just(Token::Comma))
                    .allow_trailing() // Foo is Rust-like, so allow trailing commas to appear in arg lists
                    .delimited_by(just(Token::OpenParen), just(Token::CloseParen)),
            )
            .map(|(f, args)| Expr::Call(f, args));

        let atom = num
            .or(expr.delimited_by(just(Token::OpenParen), just(Token::CloseParen)))
            .or(call)
            .or(ident.map(Expr::Var));

        let op = just(Token::Minus).to(Expr::Neg);

        let unary = op.repeated()
            .then(atom)
            .foldr(|_op, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                just(Token::Times)
                    .to(Expr::Mul as fn(_, _) -> _)
                    .or(just(Token::Div).to(Expr::Div as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                just(Token::Plus)
                    .to(Expr::Add as fn(_, _) -> _)
                    .or(just(Token::Minus).to(Expr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum
    });

    let decl = recursive(|decl| {
        let r#let = just(Token::Let)
            .ignore_then(ident)
            .then_ignore(just(Token::Eq))
            .then(expr.clone())
            .then_ignore(just(Token::Semi))
            .then(decl.clone())
            .map(|((name, rhs), then)| Expr::Let {
                name,
                rhs: Box::new(rhs),
                then: Box::new(then),
            });

        // e.g. fn add a b = a + b;
        let r#fn = just(Token::Fn)
            .ignore_then(ident) // function name
            .then(ident.repeated()) // function args
            .then_ignore(just(Token::Eq))
            .then(expr.clone())
            .then_ignore(just(Token::Semi))
            .then(decl)
            .map(|(((name, args), body), then)| Expr::Fn {
                name,
                args,
                body: Box::new(body),
                then: Box::new(then),
            });

        r#let
            .or(r#fn)
            .or(expr)
    });

    decl.then_ignore(end())
}
