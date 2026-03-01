use std::char;

#[derive(PartialEq, Eq, Debug)]
pub enum TokenKind {
    Identifier,

    // Keywords
    Variable,
    Function,
    Return,

    // Literals
    True,
    False,
    Number,
    String,

    // Operators
    Assign,

    // Separators / Punctuators
    LeftParenthesis,
    RightParenthesis,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,

    Illegal,
}

fn lookup_keyword(s: &String) -> Option<TokenKind> {
    match s.as_str() {
        "var" => Some(TokenKind::Variable),
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),
        _ => None,
    }
}
fn lookup_char(c: char) -> Option<TokenKind> {
    match c {
        // Operators
        '=' => Some(TokenKind::Assign),
        // Separators / Punctuators
        '(' => Some(TokenKind::LeftParenthesis),
        ')' => Some(TokenKind::RightParenthesis),
        '[' => Some(TokenKind::LeftBracket),
        ']' => Some(TokenKind::RightBracket),
        '{' => Some(TokenKind::LeftCurly),
        '}' => Some(TokenKind::RightCurly),
        _ => None,
    }
}

fn is_break_char(c: char) -> bool {
    return c.is_ascii_whitespace() || c == ';';
}

#[derive(PartialEq, Eq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new<V: Into<String>>(value: V, kind: TokenKind) -> Self {
        Self {
            value: value.into(),
            kind,
        }
    }
}

fn is_digit(c: char) -> bool {
    return c.is_ascii_digit() || c == '.';
}

fn is_letter(c: char) -> bool {
    return c.is_ascii_alphabetic() || c == '_';
}

fn validate_identifier(c: char, current_token: &String) -> bool {
    if !is_digit(c) && !is_letter(c) {
        return false;
    }

    match current_token.chars().nth(0) {
        Some(first_char) => {
            if is_digit(first_char) {
                return false;
            }
        }
        None => {
            if is_digit(c) {
                return false;
            }
        }
    }

    return true;
}

fn validate_number(c: char, current_token: &String) -> bool {
    // Has to be a digit or '.' char. If c == '.' the last char can't also be '.'
    return is_digit(c) && (c != '.' || current_token.chars().last() != Some('.'));
}

fn is_valid_number(text: &String) -> bool {
    let decimal_separator_count = text.chars().filter(|x| *x == '.').count();

    return decimal_separator_count <= 1 && text.chars().all(|x| is_digit(x));
}

fn is_valid_string(text: &String) -> bool {
    if !text.starts_with('"') || !text.ends_with('"') {
        return false;
    }

    return true;
}

fn is_valid_identifier(text: &String) -> bool {
    // First char has to be considered a letter
    if let Some(first_char) = text.chars().nth(0)
        && !is_letter(first_char)
    {
        return false;
    }

    return text.chars().all(|x| is_letter(x) || x.is_ascii_digit());
}

fn text_to_token(text: &String) -> Option<Token> {
    if text.is_empty() {
        return None;
    }

    match lookup_keyword(&text) {
        Some(kind) => return Some(Token::new(text, kind)),
        None => {
            let is_string = is_valid_string(&text);
            let value = if is_string {
                text.as_str()[1..text.len() - 1].to_string()
            } else {
                text.clone()
            };
            let kind = if is_valid_number(&text) {
                TokenKind::Number
            } else if is_valid_identifier(&text) {
                TokenKind::Identifier
            } else if is_valid_string(&text) {
                TokenKind::String
            } else {
                TokenKind::Illegal
            };
            return Some(Token::new(value, kind));
        }
    }
}

pub fn lexer(text: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current: String = String::new();

    // Add a new line to ensure final char gets parsed
    let corrected_text = text.trim().to_string() + "\n";
    for char in corrected_text.chars() {
        if let Some(kind) = lookup_char(char) {
            tokens.push(Token::new(char, kind));
            continue;
        }

        if is_digit(char) || is_letter(char) || char == '"' {
            current.push(char);
            continue;
        }

        // If char is a space and we are currently processing a likely string
        if char == ' '
            && let Some(first_char) = current.chars().nth(0)
            && first_char == '"'
        {
            current.push(char);
            continue;
        }

        if is_break_char(char) {
            if let Some(token) = text_to_token(&current) {
                tokens.push(token);
            }
            current.clear();
            continue;
        }

        tokens.push(Token::new(char, TokenKind::Illegal));
        current.clear();
    }

    tokens
        .iter()
        .for_each(|x| println!("{:?}: {:}", x.kind, x.value));

    return tokens;
}
