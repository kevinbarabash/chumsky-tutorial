use chumsky::prelude::*;

mod eval;
mod lexer;
mod parser;

use eval::*;
use lexer::*;
use parser::*;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    match lexer().parse(src.clone()) {
        Ok(tokens) => {
            match parser().parse(tokens) {
                Ok(ast) => match eval(&ast, &mut Vec::new(), &mut Vec::new()) {
                    Ok(output) => println!("Result: {}", output),
                    Err(eval_err) => {
                        // dbg!(ast);
                        println!("Evaluation error: {}", eval_err);
                    }
                },
                Err(parse_errs) => parse_errs.into_iter().for_each(|e| {
                    let span = e.span();
                    // TODO: convert these offsets into line number and column
                    println!("Span: from {} to {}", span.start, span.end);
                    println!("Parse error: {}", e)
                }),
            }
        }
        Err(lex_errs) => lex_errs
            .into_iter()
            .for_each(|e| println!("Lex error: {}", e)),
    };
}
