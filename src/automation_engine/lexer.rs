use std::fmt;

const LITERALS: [char; 2] = ['=', '"'];
const BREAK_CHARS: [char; 3] = [' ', '\n', ';'];
const KEYWORDS: [&str; 3] = ["var", "true", "false"];

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
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

pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
}

fn text_to_token(text: &String) -> Option<Token> {
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

pub fn lexer(text: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current: String = String::new();

    for char in text.trim().chars() {
        if LITERALS.contains(&char) || BREAK_CHARS.contains(&char) {
            match text_to_token(&current) {
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
    match text_to_token(&current) {
        None => current.clear(),
        Some(token) => {
            tokens.push(token);
            current.clear()
        }
    }

    return tokens;
}
