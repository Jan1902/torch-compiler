#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TokenType {
    // Special
    EOF,

    // Literals
    NUMBER,
    IDENTIFIER,
    KEYWORD,

    // Blocks
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    COMMA,
    SEMICOLON,

    // Operators
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EQ,
    NEQ,
    LT,
    LTE,
    GT,
    GTE,
    ASSIGN,
}

#[derive(Clone, Debug)]
pub enum TokenValue {
    None,
    Number(i32),
    Identifier(String),
    Keyword(Keyword),
}

#[derive(Debug, Clone)]
pub enum Keyword {
    LET,
    IF,
    WHILE,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: TokenValue,
    pub position: usize,
    pub source_id: usize,
}