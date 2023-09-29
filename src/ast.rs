use std::iter::Iterator;

use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum Ast {
    None,
    Number(f64),
    Boolean(bool),
    String(String),
    Symbol(String),
    Operation {
        operator: String,
        operands: Vec<Ast>,
    },
}

// Stolen from https://users.rust-lang.org/t/peekable-argument-there-has-to-be-a-simpler-way/6959/7
// TODO: understand this nonsense
trait PeekableIterator: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: std::iter::Iterator> PeekableIterator for std::iter::Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        std::iter::Peekable::peek(self)
    }
}

fn parse(tokens: &mut dyn PeekableIterator<Item = &Token>) -> Ast {
    let token = tokens.next().unwrap();

    match token {
        Token::Number(number) => Ast::Number(number.clone()),
        Token::String(string) => Ast::String(string.clone()),
        Token::Symbol(symbol) => Ast::Symbol(symbol.clone()),
        Token::Boolean(boolean) => Ast::Boolean(boolean.clone()),
        Token::CloseParenthesis => {
            panic!("Unexpected close parenthesis")
        }
        Token::OpenParenthesis => {
            let operator = match tokens.next() {
                Some(Token::Symbol(symbol)) => symbol.clone(),
                _ => panic!("Expected symbol in first position of operation"),
            };

            let mut operands = Vec::new();

            'operators: loop {
                let token = *tokens.peek().unwrap();

                match token {
                    Token::CloseParenthesis => {
                        tokens.next();
                        break 'operators;
                    }
                    Token::OpenParenthesis => {
                        operands.push(parse(tokens));
                    }
                    _ => {
                        operands.push(parse(tokens));
                    }
                }
            }

            Ast::Operation { operator, operands }
        }
    }
}

pub fn generate(tokens: &Vec<Token>) -> Vec<Ast> {
    let mut ast = Vec::new();

    let mut tokens = tokens.iter().peekable();

    loop {
        if tokens.peek().is_none() {
            break;
        }

        let result = parse(&mut tokens);
        ast.push(result);
    }

    ast
}
