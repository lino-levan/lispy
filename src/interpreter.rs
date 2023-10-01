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

fn evaluate(ast: Ast, state: &mut State) -> Ast {
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
                            true => match evaluate(operand, state) {
                                Ast::Number(number) => {
                                    string_result.push_str(number.to_string().as_str())
                                }
                                Ast::String(string) => string_result.push_str(string.as_str()),
                                _ => panic!("Expected number"),
                            },
                            false => match evaluate(operand, state) {
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
                                _ => panic!("Expected number"),
                            },
                        }
                    }

                    match is_string {
                        true => Ast::String(string_result),
                        false => Ast::Number(num_result),
                    }
                }
                "print" => {
                    for operand in operands {
                        match evaluate(operand, state) {
                            Ast::Number(number) => print!("{}", number),
                            Ast::String(string) => print!("{}", string),
                            Ast::Boolean(boolean) => print!("{}", boolean),
                            Ast::None => print!("None"),
                            _ => panic!("Expected numnber, string, or none"),
                        }
                    }
                    println!();

                    Ast::None
                }
                "var" => {
                    // TODO: consider allowing strings as variable names
                    let symbol = match operands[0].clone() {
                        Ast::Symbol(symbol) => symbol,
                        _ => panic!("Expected symbol"),
                    };

                    let value = evaluate(operands[1].clone(), state);

                    state.set(symbol, value.clone());

                    value
                }
                _ => panic!("Unknown operator {}", operator),
            }
        }
        Ast::Symbol(symbol) => state.get(&symbol),
        _ => ast,
    }
}

pub fn run(ast: Ast, state: &mut State) {
    evaluate(ast, state);
}
