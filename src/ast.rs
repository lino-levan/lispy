use std::error::Error;
use std::iter::Iterator;

use crate::tokenizer::Token;
use crate::util::PeekableIterator;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    None,
    Number(f64),
    Boolean(bool),
    String(String),
    Symbol(String),
    List(Vec<Ast>),
    Operation {
        operator: String,
        operands: Vec<Ast>,
    },
}

impl Ast {
    pub fn print(&self) {
        match self {
            Ast::Number(number) => print!("{}", number),
            Ast::String(string) => print!("{}", string),
            Ast::Boolean(boolean) => print!("{}", boolean),
            Ast::List(list) => {
                print!("[");

                for (pos, item) in list.iter().enumerate() {
                    if pos != 0 {
                        print!(", ");
                    }
                    item.print();
                }

                print!("]");
            }
            Ast::None => print!("None"),
            _ => print!("[UNSERIALIZABLE]"),
        }
    }
}

fn parse(tokens: &mut dyn PeekableIterator<Item = &Token>) -> Result<Ast, Box<dyn Error>> {
    let token = tokens.next().unwrap();

    match token {
        Token::Number(number) => Ok(Ast::Number(number.clone())),
        Token::String(string) => Ok(Ast::String(string.clone())),
        Token::Symbol(symbol) => Ok(Ast::Symbol(symbol.clone())),
        Token::Boolean(boolean) => Ok(Ast::Boolean(boolean.clone())),
        Token::CloseParenthesis => Err("Unexpected close parenthesis".into()),
        Token::OpenParenthesis => {
            let operator = match tokens.next() {
                Some(Token::Symbol(symbol)) => Ok(symbol.clone()),
                _ => Err("Expected symbol in first position of operation"),
            }?;

            let mut operands = Vec::new();

            'operators: loop {
                let token = *tokens.peek().unwrap();

                match token {
                    Token::CloseParenthesis => {
                        tokens.next();
                        break 'operators;
                    }
                    Token::OpenParenthesis => {
                        operands.push(parse(tokens)?);
                    }
                    _ => {
                        operands.push(parse(tokens)?);
                    }
                }
            }

            match operator.as_str() {
                "list" => return Ok(Ast::List(operands)),
                _ => Ok(Ast::Operation { operator, operands }),
            }
        }
    }
}

pub fn generate(tokens: &Vec<Token>) -> Result<Vec<Ast>, Box<dyn Error>> {
    let mut ast = Vec::new();

    let mut tokens = tokens.iter().peekable();

    loop {
        if tokens.peek().is_none() {
            break;
        }

        let result = parse(&mut tokens)?;
        ast.push(result);
    }

    Ok(ast)
}
