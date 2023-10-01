use std::error::Error;

use crate::ast::Ast;

pub struct State {
    pub variables: Vec<(String, Ast)>,
}

impl State {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    /// Get the value of a variable or return Ast::None
    pub fn get(&self, symbol: &String) -> Ast {
        for (name, value) in self.variables.iter().rev() {
            if name == symbol {
                return value.clone();
            }
        }

        Ast::None
    }

    pub fn set(&mut self, symbol: String, value: Ast) {
        self.variables.push((symbol, value));
    }
}

pub fn evaluate(ast: Ast, state: &mut State) -> Result<Ast, Box<dyn Error>> {
    match ast {
        Ast::Operation { operator, operands } => {
            match operator.as_str() {
                "+" => {
                    let mut numbered = false;
                    let mut is_string = false;
                    let mut num_result = 0.0;
                    let mut string_result = String::new();

                    for operand in operands {
                        match is_string {
                            true => match evaluate(operand, state)? {
                                Ast::Number(number) => {
                                    string_result.push_str(number.to_string().as_str())
                                }
                                Ast::String(string) => string_result.push_str(string.as_str()),
                                _ => {
                                    return Err("Unexpected type as input to +".into());
                                }
                            },
                            false => match evaluate(operand, state)? {
                                Ast::Number(number) => {
                                    numbered = true;
                                    num_result += number
                                }
                                Ast::String(string) => {
                                    is_string = true;
                                    if numbered {
                                        string_result.push_str(num_result.to_string().as_str());
                                    }
                                    string_result.push_str(string.as_str());
                                }
                                _ => {
                                    return Err("Unexpected type as input to +".into());
                                }
                            },
                        }
                    }

                    match is_string {
                        true => Ok(Ast::String(string_result)),
                        false => Ok(Ast::Number(num_result)),
                    }
                }
                "<" => {
                    if operands.len() == 0 {
                        return Ok(Ast::Boolean(true));
                    }

                    let mut last_number = match evaluate(operands[0].clone(), state)? {
                        Ast::Number(number) => Ok(number),
                        _ => Err("Expected number"),
                    }?;

                    for operand in operands[1..].iter() {
                        let number = match evaluate(operand.clone(), state)? {
                            Ast::Number(number) => Ok(number),
                            _ => Err("Expected number"),
                        }?;

                        if number <= last_number {
                            return Ok(Ast::Boolean(false));
                        }

                        last_number = number;
                    }

                    Ok(Ast::Boolean(true))
                }
                "while" => {
                    let condition = operands[0].clone();

                    loop {
                        match evaluate(condition.clone(), state)? {
                            Ast::Boolean(false) => break,
                            _ => (),
                        }

                        for operand in operands[1..].iter() {
                            evaluate(operand.clone(), state)?;
                        }
                    }

                    Ok(Ast::None)
                }
                "print" => {
                    for operand in operands {
                        evaluate(operand, state)?.print();
                    }
                    println!();

                    Ok(Ast::None)
                }
                "var" => {
                    // TODO: consider allowing strings as variable names
                    let symbol = match operands[0].clone() {
                        Ast::Symbol(symbol) => Ok(symbol),
                        _ => Err("Expected symbol"),
                    }?;

                    let value = evaluate(operands[1].clone(), state)?;

                    state.set(symbol, value.clone());

                    Ok(value)
                }
                _ => Err(format!("Unknown operator {}", operator).into()),
            }
        }
        Ast::Symbol(symbol) => Ok(state.get(&symbol)),
        _ => Ok(ast),
    }
}
