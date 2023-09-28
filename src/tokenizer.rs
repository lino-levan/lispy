#[derive(Debug, Clone)]
pub enum Token {
    OpenParenthesis,
    CloseParenthesis,
    Symbol(String),
    String(String),
    Number(f64),
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut content = input.chars().peekable();
    let mut tokens = Vec::new();

    loop {
        if content.peek().is_none() {
            break;
        }

        let char = content.next().unwrap();

        match char {
            c if c.is_whitespace() => {}
            '(' => tokens.push(Token::OpenParenthesis),
            ')' => tokens.push(Token::CloseParenthesis),
            '"' => {
                let mut string = String::new();

                loop {
                    // TODO: Make this return an error if there is no closing quote
                    let token = content.next().unwrap();

                    if token == '"' {
                        break;
                    }

                    string.push(token);
                }

                tokens.push(Token::String(string));
            }
            '0'..='9' => {
                let mut number = String::new();

                number.push(char);

                loop {
                    // TODO: Make this return an error if there is no closing quote
                    let token = *content.peek().unwrap();

                    if token.is_whitespace() || token == '(' || token == ')' {
                        break;
                    }

                    // :eyebrow_raise:
                    number.push(content.next().unwrap());
                }

                // TODO: fix this
                tokens.push(Token::Number(
                    number.parse::<f64>().expect("Should be a number"),
                ));
            }
            _ => {
                let mut symbol = String::new();

                symbol.push(char);

                loop {
                    // TODO: Make this return an error
                    let token = *content.peek().unwrap();

                    if token.is_whitespace() || token == '(' || token == ')' {
                        break;
                    }

                    // :eyebrow_raise:
                    symbol.push(content.next().unwrap());
                }

                tokens.push(Token::Symbol(symbol));
            }
        }
    }

    tokens
}
