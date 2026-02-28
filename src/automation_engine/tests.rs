use crate::automation_engine::lexer::{self, TokenType};

#[test]
fn exploration() {
    let result = lexer::lexer(String::from("Help me"));

    assert_eq!(result.len(), 1, "Should produce {} tokens", 1);
    assert_eq!(result[0].token_type, TokenType::Value);
    assert_eq!(result[0].token_value, "Help");
}
