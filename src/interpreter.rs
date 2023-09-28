use crate::ast::Ast;

fn evaluate(ast: Ast) -> Ast {
    match ast {
        Ast::Operation { operator, operands } => {
            // evaluate operands
            let operands = operands.into_iter().map(evaluate).collect::<Vec<Ast>>();

            // evaluate operator
            match operator.as_str() {
                "+" => {
                    let mut numbered = false;
                    let mut is_string = false;
                    let mut num_result = 0.0;
                    let mut string_result = String::new();

                    for operand in operands {
                        match is_string {
                            true => match operand {
                                Ast::Number(number) => {
                                    string_result.push_str(number.to_string().as_str())
                                }
                                Ast::String(string) => string_result.push_str(string.as_str()),
                                _ => panic!("Expected number"),
                            },
                            false => match operand {
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
                        match operand {
                            Ast::Number(number) => print!("{}", number),
                            Ast::String(string) => print!("{}", string),
                            _ => panic!("Expected number or string"),
                        }
                    }
                    println!();

                    Ast::None
                }
                _ => panic!("Unknown operator {}", operator),
            }
        }
        _ => ast,
    }
}

pub fn run(ast: Ast) {
    evaluate(ast);
}
