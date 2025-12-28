use crate::{errors::CompileError, source::Source, token::{Keyword, Token, TokenType, TokenValue}};

pub type LexResult<T> = Result<T, CompileError>;

pub struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    // Logical
    pub fn new(source: &'a Source) -> Self {
        Lexer { src: source.content.as_bytes(), pos: 0 }
    }

    fn current(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos + 1).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn advance_and(&mut self, token: TokenType) -> LexResult<Token> {
        let start = self.pos;
        self.advance();

        Ok(Token {
            token_type: token,
            value: TokenValue::None,
            position: start,
        })
    }

    // Main Tokenizing Logic
    fn get_token(&mut self) -> LexResult<Token> {
        self.skip_whitespace();

        let c = match self.current() {
            Some(c) => c,
            None => {
                return Ok(Token {
                    token_type: TokenType::EOF,
                    value: TokenValue::None,
                    position: self.pos,
                });
            }
        };

        match c {
            b'#' => {
                self.skip_line_comment();
                self.get_token()
            }
            b'+' => self.advance_and(TokenType::PLUS),
            b'-' => self.advance_and(TokenType::MINUS),
            b'*' => self.advance_and(TokenType::ASTERISK),
            b'/' => self.advance_and(TokenType::SLASH),
            b'=' => self.equals(),
            b'<' => self.less_than(),
            b'>' => self.greater_than(),
            b'{' => self.advance_and(TokenType::LBRACE),
            b'}' => self.advance_and(TokenType::RBRACE),
            b'(' => self.advance_and(TokenType::LPAREN),
            b')' => self.advance_and(TokenType::RPAREN),
            b'[' => self.advance_and(TokenType::LBRACKET),
            b']' => self.advance_and(TokenType::RBRACKET),
            b',' => self.advance_and(TokenType::COMMA),
            b';' => self.advance_and(TokenType::SEMICOLON),
            b'!' => self.bang(),
            b'0'..=b'9' => self.number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.literal(),
            _ => Err(CompileError {
                    message: format!("unexpected character '{}'", c as char),
                    position: self.pos,
                })       
        }
    }

    pub fn read_all(&mut self) -> LexResult<Vec<Token>> {
        let mut tokens = Vec::<Token>::new();

        loop {
            let token = self.get_token()?;
            
            if let TokenType::EOF = token.token_type {
                tokens.push(token);
                break;
            }
            
            tokens.push(token);
        }

        Ok(tokens)
    }

    // Operators
    fn bang(&mut self) -> LexResult<Token> {
        let c = self.current().unwrap();
        if matches!(self.peek(), Some(b'=')) {
            self.advance();
            self.advance_and(TokenType::NEQ)
        } else {
            panic!("Lexing error: Unexpected character '{}' at {}", c as char, self.pos)
        }
    }

    fn equals(&mut self) -> LexResult<Token> {
        if matches!(self.peek(), Some(b'=')) {
            self.advance();
            self.advance_and(TokenType::EQ)
        } else {
            self.advance_and(TokenType::ASSIGN)
        }
    }

    fn less_than(&mut self) -> LexResult<Token> {
        if matches!(self.peek(), Some(b'=')) {
            self.advance();
            self.advance_and(TokenType::LTE)
        }
        else {
            self.advance_and(TokenType::LT)
        }
    }

    fn greater_than(&mut self) -> LexResult<Token> {
        if matches!(self.peek(), Some(b'=')) {
            self.advance();
            self.advance_and(TokenType::GTE)
        }
        else {
            self.advance_and(TokenType::GT)
        }
    }

    fn number(&mut self) -> LexResult<Token> {
        let start = self.pos;

        while matches!(self.current(), Some(b'0'..=b'9')) {
            self.advance();
        }

        let value = Self::parse_i32_ascii(&self.src[start..self.pos]);

        Ok(Token {
            token_type: TokenType::NUMBER,
            value: TokenValue::Number(value),
            position: start,
        })
    }

    fn literal(&mut self) -> LexResult<Token> {
        let start = self.pos;

        while matches!(self.current(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')) {
            self.advance();
        }

        let identifier = String::from_utf8(self.src[start..self.pos].to_vec()).unwrap();

        let (kind, value) = match identifier.as_str() {
            "let" => (TokenType::KEYWORD, TokenValue::Keyword(Keyword::LET)),
            "if" => (TokenType::KEYWORD, TokenValue::Keyword(Keyword::IF)),
            "while" => (TokenType::KEYWORD, TokenValue::Keyword(Keyword::WHILE)),
            _ => (TokenType::IDENTIFIER, TokenValue::Identifier(identifier)),
        };

        Ok(Token {
            token_type: kind,
            value,
            position: start,
        })
    }

    // Skipping
    fn skip_whitespace(&mut self) {
        while matches!(self.current(), Some(b' ' | b'\n' | b'\t' | b'\r')) {
            self.advance();
        }
    }

    fn skip_line_comment(&mut self) {
        self.advance();

        while let Some(c) = self.current() {
            if c == b'\n' {
                break;
            }
            self.advance();
        }
    }

    // Utility
    fn parse_i32_ascii(bytes: &[u8]) -> i32 {
        let mut value: i32 = 0;

        for &b in bytes {
            value = value * 10 + (b - b'0') as i32;
        }

        value
    }
}