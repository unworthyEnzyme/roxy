pub mod scanner {
    #[derive(Debug, PartialEq, Eq)]
    pub enum Punctuations {
        Bang,
        BangEqual,
        Equal,
        EqualEqual,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,
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
    }
    #[derive(Debug, PartialEq, Eq)]
    pub enum Keywords {
        And,
        Class,
        Else,
        False,
        Fun,
        For,
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
    }
    #[derive(Debug)]
    pub struct NumberLiteral {
        pub value: f64,
        pub raw: String,
    }
    impl PartialEq for NumberLiteral {
        fn eq(&self, other: &Self) -> bool {
            self.raw == other.raw
        }
    }
    impl Eq for NumberLiteral {}

    #[derive(Debug, PartialEq, Eq)]
    pub enum Literals {
        String { value: String },
        Number(NumberLiteral),
    }
    #[derive(Debug, PartialEq, Eq)]
    pub enum Token {
        Punctuation(Punctuations),
        Keyword(Keywords),
        Identifier { lexeme: String },
        Literal(Literals),
        EOF,
    }

    pub struct Scanner {
        source: String,
        tokens: Vec<Token>,
        start: usize,
        current: usize,
        line: usize,
    }

    impl Scanner {
        pub fn new(source: String) -> Scanner {
            Scanner {
                source,
                tokens: Vec::new(),
                start: 0,
                current: 0,
                line: 1,
            }
        }
        fn scan_token(&mut self) {
            let c = self.advance();
            match c {
                '(' => self.add_token(Token::Punctuation(Punctuations::LeftParen)),
                ')' => self.add_token(Token::Punctuation(Punctuations::RightParen)),
                '{' => self.add_token(Token::Punctuation(Punctuations::LeftBrace)),
                '}' => self.add_token(Token::Punctuation(Punctuations::RightBrace)),
                ',' => self.add_token(Token::Punctuation(Punctuations::Comma)),
                '.' => self.add_token(Token::Punctuation(Punctuations::Dot)),
                '-' => self.add_token(Token::Punctuation(Punctuations::Minus)),
                '+' => self.add_token(Token::Punctuation(Punctuations::Plus)),
                ';' => self.add_token(Token::Punctuation(Punctuations::Semicolon)),
                '*' => self.add_token(Token::Punctuation(Punctuations::Star)),
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                '!' => {
                    if self.match_char('=') {
                        self.add_token(Token::Punctuation(Punctuations::BangEqual))
                    } else {
                        self.add_token(Token::Punctuation(Punctuations::Bang))
                    }
                }
                '=' => {
                    if self.match_char('=') {
                        self.add_token(Token::Punctuation(Punctuations::EqualEqual))
                    } else {
                        self.add_token(Token::Punctuation(Punctuations::Equal))
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        self.add_token(Token::Punctuation(Punctuations::LessEqual))
                    } else {
                        self.add_token(Token::Punctuation(Punctuations::Less))
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        self.add_token(Token::Punctuation(Punctuations::GreaterEqual))
                    } else {
                        self.add_token(Token::Punctuation(Punctuations::Greater))
                    }
                }
                '/' => {
                    if self.match_char('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(Token::Punctuation(Punctuations::Slash))
                    }
                }
                '"' => self.string(),
                _ if c.is_digit(10) => self.number(),
                _ => todo!(),
            }
        }
        pub fn scan_tokens(&mut self) -> &Vec<Token> {
            while !self.is_at_end() {
                self.start = self.current;
                self.scan_token();
            }
            self.tokens.push(Token::EOF);
            &self.tokens
        }
        fn advance(&mut self) -> char {
            self.current += 1;
            self.source.chars().nth(self.current - 1).unwrap()
        }

        fn add_token(&mut self, token: Token) {
            self.tokens.push(token);
        }

        fn is_at_end(&self) -> bool {
            self.current >= self.source.len()
        }

        fn match_char(&mut self, c: char) -> bool {
            if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != c {
                return false;
            }
            self.current += 1;
            true
        }

        fn peek(&self) -> char {
            if self.is_at_end() {
                return '\0';
            }
            self.source.chars().nth(self.current).unwrap()
        }

        fn string(&mut self) {
            while self.peek() != '"' && !self.is_at_end() {
                if self.peek() != '\n' {
                    self.line += 1;
                }
                self.advance();
            }
            if self.is_at_end() {
                panic!("[line {}] Error: Unterminated string literal", self.line);
            }

            self.advance();
            /*
             We shouldn't have to copy this substring.
             Either i can use &str in the Literals::String type or
             because i know i won't be using this slice anywhere else
             i think i can use unsafe block to solve this problem.
            */
            let value = String::from(&self.source[self.start + 1..self.current - 1]);
            self.add_token(Token::Literal(Literals::String { value }));
        }

        fn number(&mut self) {
            while char::is_digit(self.peek(), 10) {
                self.advance();
            }
            if self.peek() == '.' && char::is_digit(self.peek_next(), 10) {
                self.advance();

                while char::is_digit(self.peek(), 10) {
                    self.advance();
                }
            }
            let raw = String::from(&self.source[self.start..self.current]);
            let value = raw.parse::<f64>().unwrap();
            self.add_token(Token::Literal(Literals::Number(NumberLiteral {
                raw,
                value,
            })))
        }

        fn peek_next(&self) -> char {
            if self.current + 1 >= self.source.len() {
                '\0'
            } else {
                self.source.chars().nth(self.current + 1).unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::scanner::*;
    #[test]
    fn single_character_tokens() {
        let source = "(){},-*;".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(
            *tokens,
            vec![
                Token::Punctuation(Punctuations::LeftParen),
                Token::Punctuation(Punctuations::RightParen),
                Token::Punctuation(Punctuations::LeftBrace),
                Token::Punctuation(Punctuations::RightBrace),
                Token::Punctuation(Punctuations::Comma),
                Token::Punctuation(Punctuations::Minus),
                Token::Punctuation(Punctuations::Star),
                Token::Punctuation(Punctuations::Semicolon),
                Token::EOF
            ]
        );
    }
    #[test]
    fn ignore_whitespaec() {
        let source = r#" ( ) { },    -
        *;  "#
            .to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(
            *tokens,
            vec![
                Token::Punctuation(Punctuations::LeftParen),
                Token::Punctuation(Punctuations::RightParen),
                Token::Punctuation(Punctuations::LeftBrace),
                Token::Punctuation(Punctuations::RightBrace),
                Token::Punctuation(Punctuations::Comma),
                Token::Punctuation(Punctuations::Minus),
                Token::Punctuation(Punctuations::Star),
                Token::Punctuation(Punctuations::Semicolon),
                Token::EOF
            ]
        );
    }
    #[test]
    fn operators() {
        let source = "! != - - = == < <= > >= */".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(
            *tokens,
            vec![
                Token::Punctuation(Punctuations::Bang),
                Token::Punctuation(Punctuations::BangEqual),
                Token::Punctuation(Punctuations::Minus),
                Token::Punctuation(Punctuations::Minus),
                Token::Punctuation(Punctuations::Equal),
                Token::Punctuation(Punctuations::EqualEqual),
                Token::Punctuation(Punctuations::Less),
                Token::Punctuation(Punctuations::LessEqual),
                Token::Punctuation(Punctuations::Greater),
                Token::Punctuation(Punctuations::GreaterEqual),
                Token::Punctuation(Punctuations::Star),
                Token::Punctuation(Punctuations::Slash),
                Token::EOF
            ]
        );
    }
    #[test]
    fn comments() {
        let source = r#"// this is a comment
        ! != - - = == <
        <= > >= */ ( ) { },    -
        *;
        "#
        .to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(
            *tokens,
            vec![
                Token::Punctuation(Punctuations::Bang),
                Token::Punctuation(Punctuations::BangEqual),
                Token::Punctuation(Punctuations::Minus),
                Token::Punctuation(Punctuations::Minus),
                Token::Punctuation(Punctuations::Equal),
                Token::Punctuation(Punctuations::EqualEqual),
                Token::Punctuation(Punctuations::Less),
                Token::Punctuation(Punctuations::LessEqual),
                Token::Punctuation(Punctuations::Greater),
                Token::Punctuation(Punctuations::GreaterEqual),
                Token::Punctuation(Punctuations::Star),
                Token::Punctuation(Punctuations::Slash),
                Token::Punctuation(Punctuations::LeftParen),
                Token::Punctuation(Punctuations::RightParen),
                Token::Punctuation(Punctuations::LeftBrace),
                Token::Punctuation(Punctuations::RightBrace),
                Token::Punctuation(Punctuations::Comma),
                Token::Punctuation(Punctuations::Minus),
                Token::Punctuation(Punctuations::Star),
                Token::Punctuation(Punctuations::Semicolon),
                Token::EOF
            ]
        )
    }
    #[test]
    fn string_literal() {
        let source = r#""This is a string literal"
        "This is a
multiline string
literal""#
            .to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(
            *tokens,
            vec![
                Token::Literal(Literals::String {
                    value: "This is a string literal".to_string(),
                }),
                Token::Literal(Literals::String {
                    value: r#"This is a
multiline string
literal"#
                        .to_string()
                }),
                Token::EOF
            ]
        )
    }
    #[test]
    #[should_panic]
    fn unterminated_string_literal() {
        let source = r#""This is an unterminated string literal"#.to_string();
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
    }

    #[test]
    fn number_literal() {
        let source = "123.456".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(
            *tokens,
            vec![
                Token::Literal(Literals::Number(NumberLiteral {
                    raw: "123.456".to_string(),
                    value: 123.456
                })),
                Token::EOF
            ]
        )
    }
}
