use crate::ast::Ast;

fn evaluate(ast: Ast) -> Ast {
    match ast {
        Ast::Operation { operator, operands } => {
            // evaluate operands
            let operands = operands.into_iter().map(evaluate).collect::<Vec<Ast>>();

            // evaluate operator
            match operator.as_str() {
                "+" => {
                    let mut result = 0.0;

                    for operand in operands {
                        match operand {
                            Ast::Number(number) => result += number,
                            _ => panic!("Expected number"),
                        }
                    }

                    Ast::Number(result)
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
