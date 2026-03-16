use std::char;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Identifier,
    Assign,

    // Keywords
    Variable,
    Function,
    Return,
    If,
    Else,

    // Literals
    String,
    Number,
    True,
    False,

    // Comparison Operators
    Bang,
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,

    // Arithmetic Operators
    Add,
    Subtract,
    Divide,
    Multiply,

    // Separators / Punctuators
    LeftParenthesis,
    RightParenthesis,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    Comma,

    // Builtin
    Print,

    Illegal,
}

fn lookup_keyword(s: &String) -> Option<TokenKind> {
    match s.as_str() {
        "var" => Some(TokenKind::Variable),
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),
        "fn" => Some(TokenKind::Function),
        "return" => Some(TokenKind::Return),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "print" => Some(TokenKind::Print),
        _ => None,
    }
}
fn lookup_char(c: char) -> Option<TokenKind> {
    match c {
        // Comparison Operators
        '>' => Some(TokenKind::GreaterThan),
        '<' => Some(TokenKind::LessThan),
        // Arithmetic Operators
        '+' => Some(TokenKind::Add),
        '-' => Some(TokenKind::Subtract),
        '/' => Some(TokenKind::Divide),
        '*' => Some(TokenKind::Multiply),
        // Separators / Punctuators
        '(' => Some(TokenKind::LeftParenthesis),
        ')' => Some(TokenKind::RightParenthesis),
        '[' => Some(TokenKind::LeftBracket),
        ']' => Some(TokenKind::RightBracket),
        '{' => Some(TokenKind::LeftCurly),
        '}' => Some(TokenKind::RightCurly),
        ',' => Some(TokenKind::Comma),
        _ => None,
    }
}

fn is_break_char(c: char) -> bool {
    return c.is_ascii_whitespace() || c == ';';
}

#[derive(PartialEq, Debug, Clone)]
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

fn text_to_token(text: String) -> Option<Token> {
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
    let mut chars = corrected_text.chars().into_iter().peekable();

    while let Some(char) = chars.next() {
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

        if let Some(token) = text_to_token(current.clone()) {
            tokens.push(token);
            current.clear();
        }

        if let Some(kind) = lookup_char(char) {
            tokens.push(Token::new(char, kind));
            continue;
        }

        if char == '!' {
            if let Some(next_token) = chars.peek()
                && *next_token == '='
            {
                tokens.push(Token::new("!=", TokenKind::NotEqual));
                chars.next();
            } else {
                tokens.push(Token::new("!", TokenKind::Bang));
            }

            continue;
        }

        // Use lookahead for = char to process operators
        if char == '=' {
            let mut token = Token::new(char, TokenKind::Assign);

            match chars.peek() {
                Some(next_char) => match next_char {
                    '=' => {
                        token = Token::new("==", TokenKind::Equal);
                        chars.next();
                    }
                    _ => {}
                },
                None => {}
            };
            tokens.push(token);

            continue;
        }

        if is_break_char(char) {
            continue;
        }

        tokens.push(Token::new(char, TokenKind::Illegal));
        current.clear();
    }

    return tokens;
}

pub fn print_tokens(tokens: &Vec<Token>) {
    tokens
        .iter()
        .for_each(|x| println!("{:?}: {:}", x.kind, x.value));
}
