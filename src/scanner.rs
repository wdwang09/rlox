pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            source: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn reset(&mut self) {
        self.source.clear();
        self.start = 0;
        self.current = 0;
        self.line = 1;
    }

    pub fn scan(&mut self, source: String) {
        self.reset();
        for ch in source.chars() {
            self.source.push(ch);
        }
        let mut line = u32::MAX;
        loop {
            let token = self.scan_token();

            if token.line != line {
                print!("{:04} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }
            println!("{:?}", token);

            if matches!(token.token_type, TokenType::Eof) {
                break;
            }
        }
    }

    fn scan_token(&mut self) -> Token {
        self.skip_white_space();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }
        let c = self.advance();
        if Scanner::is_alpha(c) {
            return self.identifier();
        }
        if c.is_digit(10) {
            return self.number();
        }
        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),

            '!' | '=' | '<' | '>' => {
                let is_double_char = self.match_char('=');
                match c {
                    '!' => self.make_token(if is_double_char { TokenType::BangEqual } else { TokenType::Bang }),
                    '=' => self.make_token(if is_double_char { TokenType::EqualEqual } else { TokenType::Equal }),
                    '<' => self.make_token(if is_double_char { TokenType::LessEqual } else { TokenType::Less }),
                    '>' => self.make_token(if is_double_char { TokenType::GreaterEqual } else { TokenType::Greater }),
                    _ => panic!(),
                }
            }

            '"' => self.string(),

            _ => self.error_token("Unexpected character.")
        }
    }

    fn is_alpha(ch: char) -> bool {
        return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_';
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    fn peek(&self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let mut lexeme = String::new();
        for i in self.start..self.current {
            lexeme.push(self.source[i]);
        }
        Token {
            token_type,
            lexeme,
            line: self.line,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::Error,
            lexeme: message.to_string(),
            line: self.line,
        }
    }

    fn skip_white_space(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn identifier_type(&self) -> TokenType {
        static TOKEN_STR: [&str; 16] = [
            "and", "class", "else", "false",
            "for", "fun", "if", "nil", "or",
            "print", "return", "super", "this",
            "true", "var", "while"
        ];
        // const TOKEN_TYPES: [TokenType; 16] = [
        //     TokenType::And, TokenType::Class, TokenType::Else, TokenType::False,
        //     TokenType::For, TokenType::Fun, TokenType::If, TokenType::Nil, TokenType::Or,
        //     TokenType::Print, TokenType::Return, TokenType::Super, TokenType::This,
        //     TokenType::True, TokenType::Var, TokenType::While
        // ];
        let mut i: usize = 0;
        for ii in 0..TOKEN_STR.len() {
            let str: &str = TOKEN_STR[i];
            i = ii;
            if self.current - self.start != str.len() {
                return TokenType::Identifier;
            }
            let mut j = 0;
            for ch in str.chars() {
                if self.source[self.start + j] != ch {
                    break;
                }
                j += 1;
            }
        }
        if i == TOKEN_STR.len() {
            return TokenType::Identifier;
        }
        match TOKEN_STR[i] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    fn identifier(&mut self) -> Token {
        while Scanner::is_alpha(self.peek()) || self.peek() == '_' || self.peek().is_digit(10) {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }
        self.advance();
        self.make_token(TokenType::String)
    }
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}

#[derive(Debug)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}
