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

fn evaluate_operation(
    operator: Box<Ast>,
    operands: Vec<Ast>,
    state: &mut State,
) -> Result<Ast, Box<dyn Error>> {
    match *operator {
        Ast::Symbol(symbol) => match symbol.as_str() {
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
                let name = match operands[0].clone() {
                    Ast::Symbol(symbol) => Ok(symbol),
                    _ => Err("Expected symbol"),
                }?;

                let value = evaluate(operands[1].clone(), state)?;

                state.set(name, value.clone());

                Ok(value)
            }
            symbol if state.get(&symbol.to_string()) != Ast::None => {
                evaluate_operation(Box::new(state.get(&symbol.to_string())), operands, state)
            }
            _ => Err(format!("Undefined Symbol {:?}", symbol).into()),
        },
        Ast::List(list) => {
            if operands.len() > 1 {
                return Err(format!("Expected 1 operand, got {}", operands.len()).into());
            }

            let index = operands.get(0).unwrap();

            match index {
                Ast::Number(number) => {
                    let raw_index = *number;

                    if raw_index.fract() != 0.0 {
                        return Err(format!("Expected integer index, got {}", raw_index).into());
                    }

                    if raw_index < 0.0 {
                        return Err(format!("Expected positive index, got {}", raw_index).into());
                    }

                    let index = raw_index as usize;

                    if index >= list.len() {
                        return Err(format!(
                            "Index {} out of bounds for list of length {}",
                            number,
                            list.len()
                        )
                        .into());
                    }

                    Ok(list.get(index).unwrap().clone())
                }
                _ => Err(format!("Expected number as argument to list, got {:?}", index).into()),
            }
        }
        _ => Err(format!("Unknown operator {:?}", *operator).into()),
    }
}

pub fn evaluate(ast: Ast, state: &mut State) -> Result<Ast, Box<dyn Error>> {
    match ast {
        Ast::Operation { operator, operands } => evaluate_operation(operator, operands, state),
        Ast::Symbol(symbol) => Ok(state.get(&symbol)),
        _ => Ok(ast),
    }
}
