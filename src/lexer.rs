use crate::{
    errors::CompileError,
    source_map::SourceMap,
    token::{Keyword, Token, TokenType, TokenValue},
};

pub type LexResult<T> = Result<T, CompileError>;

pub struct Lexer<'a> {
    contexts: Vec<LexerContext>,
    source_map: &'a mut SourceMap,
    context_id: usize,
}

pub struct LexerContext {
    source_id: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    // Logical
    pub fn new(source_map: &'a mut SourceMap) -> Self {
        Lexer {
            contexts: vec![LexerContext {
                source_id: 0,
                pos: 0,
            }],
            source_map,
            context_id: 0,
        }
    }

    fn current_context(&mut self) -> &mut LexerContext {
        &mut self.contexts[self.context_id]
    }

    fn src(&self) -> &[u8] {
        self.source_map.files[self.contexts[self.context_id].source_id]
            .content
            .as_bytes()
    }

    fn pos(&self) -> usize {
        self.contexts[self.context_id].pos
    }

    fn set_pos(&mut self, pos: usize) {
        self.contexts[self.context_id].pos = pos;
    }

    fn current(&self) -> Option<u8> {
        self.src().get(self.pos()).copied()
    }

    fn peek(&self) -> Option<u8> {
        self.src().get(self.pos() + 1).copied()
    }

    fn peek_n(&self, n: usize) -> Option<&[u8]> {
        self.src().get(self.pos() + 1..self.pos() + 1 + n)
    }

    fn advance(&mut self) {
        self.set_pos(self.pos() + 1);
    }

    fn advance_and(&mut self, token: TokenType) -> LexResult<Token> {
        let start = self.pos();
        self.advance();

        Ok(Token {
            token_type: token,
            value: TokenValue::None,
            position: start,
            source_id: self.current_context().source_id,
        })
    }

    // Main Tokenizing Logic
    fn get_token(&mut self) -> LexResult<Token> {
        self.skip_whitespace();

        let c = match self.current() {
            Some(c) => c,
            None => {
                if self.contexts.len() > 1 {
                    self.contexts.pop();
                    self.context_id -= 1;
                    return self.get_token();
                }

                return Ok(Token {
                    token_type: TokenType::EOF,
                    value: TokenValue::None,
                    position: self.pos(),
                    source_id: self.current_context().source_id,
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
                position: self.pos(),
                source_id: self.current_context().source_id,
            }),
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
        } else if matches!(self.peek_n(7), Some(b"include")) {
            self.advance(); // skip '!'
            self.include()
        } else {
            panic!(
                "Lexing error: Unexpected character '{}' at {}",
                c as char,
                self.pos()
            )
        }
    }

    fn include(&mut self) -> LexResult<Token> {
        for _ in 0..7 {
            self.advance();
        }

        self.skip_whitespace();
        let file_name = self.string_literal()?;

        let new_id = self
            .source_map
            .add_from_file(&file_name)
            .map_err(|e| CompileError {
                message: format!("Failed to include file '{}': {}", file_name, e),
                position: self.pos(),
                source_id: self.current_context().source_id,
            })?;

        self.contexts.push(LexerContext {
            source_id: new_id,
            pos: 0,
        });
        self.context_id += 1;
        self.get_token()
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
        } else {
            self.advance_and(TokenType::LT)
        }
    }

    fn greater_than(&mut self) -> LexResult<Token> {
        if matches!(self.peek(), Some(b'=')) {
            self.advance();
            self.advance_and(TokenType::GTE)
        } else {
            self.advance_and(TokenType::GT)
        }
    }

    fn number(&mut self) -> LexResult<Token> {
        let start = self.pos();

        while matches!(self.current(), Some(b'0'..=b'9')) {
            self.advance();
        }

        let value = Self::parse_i32_ascii(&self.src()[start..self.pos()]);

        Ok(Token {
            token_type: TokenType::NUMBER,
            value: TokenValue::Number(value),
            position: start,
            source_id: self.current_context().source_id,
        })
    }

    fn literal(&mut self) -> LexResult<Token> {
        let start = self.pos();

        while matches!(
            self.current(),
            Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')
        ) {
            self.advance();
        }

        let identifier = String::from_utf8(self.src()[start..self.pos()].to_vec()).unwrap();

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
            source_id: self.current_context().source_id,
        })
    }

    fn string_literal(&mut self) -> LexResult<String> {
        // 1. Sicherstellen, dass wir wirklich an einem " stehen
        if self.current() != Some(b'"') {
            return Err(CompileError {
                message: format!("Expected '\"', found {:?}", self.current().map(|c| c as char)),
                position: self.pos(),
                source_id: self.current_context().source_id,
            });
        }
        
        self.advance();

        let start = self.pos();

        while let Some(c) = self.current() {
            if c == b'"' {
                let value = String::from_utf8(self.src()[start..self.pos()].to_vec()).unwrap();
                self.advance(); // Überspringe das schließende "
                return Ok(value);
            }
            self.advance();
        }

        Err(CompileError {
            message: "Unterminated string literal".to_string(),
            position: start,
            source_id: self.current_context().source_id,
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
