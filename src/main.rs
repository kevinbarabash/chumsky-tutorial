use chumsky::prelude::*;

mod eval;
mod lexer;
mod parser;
mod syntax;

use parser::*;
use eval::*;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    match parser().parse(src) {
        Ok(ast) => match eval(&ast, &mut Vec::new(), &mut Vec::new()) {
            Ok(output) => println!("{}", output),
            Err(eval_err) => {
                dbg!(ast);
                println!("Evaluation error: {}", eval_err);
            }
        },
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }
}
