use crate::ast::Ast;

fn format_part(ast: Ast, depth: usize) -> String {
    match ast {
        Ast::Number(number) => number.to_string(),
        Ast::String(string) => format!("\"{}\"", string),
        Ast::Symbol(symbol) => symbol,
        Ast::Boolean(boolean) => boolean.to_string(),
        Ast::None => "None".to_string(),
        Ast::List(list) => {
            let mut result = String::new();

            result.push_str("(list");

            for item in list {
                result.push_str(" ");
                result.push_str(format_part(item, depth + 1).as_str());
            }

            result.push_str(")");

            result
        }
        Ast::Operation { operator, operands } => {
            let mut inline_result = String::new();
            let mut multiline_result = String::new();

            let operator = format_part(*operator, depth + 1);

            inline_result.push_str("(");
            inline_result.push_str(operator.as_str());

            multiline_result.push_str("(");
            multiline_result.push_str(operator.as_str());

            for (i, operand) in operands.into_iter().enumerate() {
                let result = format_part(operand.clone(), depth + 1);

                inline_result.push_str(" ");
                inline_result.push_str(result.as_str());

                match operator.as_str() {
                    "while" if i == 0 => {
                        multiline_result.push_str(" ");
                    }
                    _ => {
                        multiline_result.push_str("\n");
                        multiline_result.push_str("\t".repeat(depth + 1).as_str());
                    }
                }
                multiline_result.push_str(result.as_str());
            }

            inline_result.push_str(")");

            multiline_result.push_str("\n");
            multiline_result.push_str("\t".repeat(depth).as_str());
            multiline_result.push_str(")");

            if inline_result.len() > 40 {
                multiline_result
            } else {
                inline_result
            }
        }
    }
}

pub fn format(ast: Ast) -> String {
    format_part(ast, 0)
}
