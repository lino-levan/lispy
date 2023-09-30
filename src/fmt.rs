use crate::ast::Ast;

fn format_part(ast: Ast, depth: usize) -> String {
    match ast {
        Ast::Number(number) => number.to_string(),
        Ast::String(string) => format!("\"{}\"", string),
        Ast::Symbol(symbol) => symbol,
        Ast::Boolean(boolean) => boolean.to_string(),
        Ast::None => "None".to_string(),
        Ast::Operation { operator, operands } => {
            let mut inline_result = String::new();
            let mut multiline_result = String::new();

            inline_result.push_str("(");
            inline_result.push_str(operator.as_str());

            multiline_result.push_str("(");
            multiline_result.push_str(operator.as_str());

            for operand in operands {
                let result = format_part(operand, depth + 1);

                inline_result.push_str(" ");
                inline_result.push_str(result.as_str());

                multiline_result.push_str("\n");
                multiline_result.push_str("\t".repeat(depth + 1).as_str());
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
