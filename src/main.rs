use std::{fmt, fs::read_to_string};

fn main() {
    let dsl = read_to_string("./dsl/test.dsl").unwrap();
    println!("Found DSL:\n{dsl}");

    let tokens = lexer(dsl);

    for Token {
        token_type,
        token_value,
    } in tokens
    {
        println!("{token_type}: {token_value}")
    }
}

const KEYWORDS: [&str; 3] = ["var", "true", "false"];
const LITERALS: [char; 1] = ['='];
const BREAK_CHARS: [char; 2] = [' ', '\n'];

enum TokenType {
    Keyword,
    Literal,
    Value,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::Keyword => write!(f, "Keyword"),
            TokenType::Literal => write!(f, "Literal"),
            TokenType::Value => write!(f, "Value"),
        }
    }
}

struct Token {
    token_type: TokenType,
    token_value: String,
}

fn lexer(text: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current: String = String::new();

    for char in text.trim().chars() {
        if LITERALS.contains(&char) || BREAK_CHARS.contains(&char) {
            match process_text_to_token(&current) {
                None => current.clear(),
                Some(token) => {
                    tokens.push(token);
                    current.clear()
                }
            }

            if LITERALS.contains(&char) {
                tokens.push(Token {
                    token_type: TokenType::Literal,
                    token_value: char.to_string(),
                });
            }

            continue;
        }

        current.push(char);
    }
    match process_text_to_token(&current) {
        None => current.clear(),
        Some(token) => {
            tokens.push(token);
            current.clear()
        }
    }

    return tokens;
}

fn process_text_to_token(text: &String) -> Option<Token> {
    if text.len() <= 0 {
        return None;
    }

    let token = Token {
        token_type: if KEYWORDS.contains(&text.as_str()) {
            TokenType::Keyword
        } else {
            TokenType::Value
        },
        token_value: text.to_string(),
    };

    return Some(token);
}
