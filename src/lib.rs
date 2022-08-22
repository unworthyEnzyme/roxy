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
        value: f64,
        raw: String,
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
}
