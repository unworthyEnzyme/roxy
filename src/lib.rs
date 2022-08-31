pub mod parser;

pub mod scanner {
    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenKind {
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
        StringLiteral(String),
        NumberLiteral(f64),
        Identifier(String),
        EOF,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Token {
        pub kind: TokenKind,
        pub line: usize,
        pub pos: usize,
    }

    #[derive(Debug, Clone)]
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
                '(' => self.add_token(Token {
                    kind: TokenKind::LeftParen,
                    line: self.line,
                    pos: self.current,
                }),
                ')' => self.add_token(Token {
                    kind: TokenKind::RightParen,
                    line: self.line,
                    pos: self.current,
                }),
                '{' => self.add_token(Token {
                    kind: TokenKind::LeftBrace,
                    line: self.line,
                    pos: self.current,
                }),
                '}' => self.add_token(Token {
                    kind: TokenKind::RightBrace,
                    line: self.line,
                    pos: self.current,
                }),
                ',' => self.add_token(Token {
                    kind: TokenKind::Comma,
                    line: self.line,
                    pos: self.current,
                }),
                '.' => self.add_token(Token {
                    kind: TokenKind::Dot,
                    line: self.line,
                    pos: self.current,
                }),
                '-' => self.add_token(Token {
                    kind: TokenKind::Minus,
                    line: self.line,
                    pos: self.current,
                }),
                '+' => self.add_token(Token {
                    kind: TokenKind::Plus,
                    line: self.line,
                    pos: self.current,
                }),
                ';' => self.add_token(Token {
                    kind: TokenKind::Semicolon,
                    line: self.line,
                    pos: self.current,
                }),
                '*' => self.add_token(Token {
                    kind: TokenKind::Star,
                    line: self.line,
                    pos: self.current,
                }),
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                '!' => {
                    if self.match_char('=') {
                        self.add_token(Token {
                            kind: TokenKind::BangEqual,
                            line: self.line,
                            pos: self.current,
                        })
                    } else {
                        self.add_token(Token {
                            kind: TokenKind::Bang,
                            line: self.line,
                            pos: self.current,
                        })
                    }
                }
                '=' => {
                    if self.match_char('=') {
                        self.add_token(Token {
                            kind: TokenKind::EqualEqual,
                            line: self.line,
                            pos: self.current,
                        })
                    } else {
                        self.add_token(Token {
                            kind: TokenKind::Equal,
                            line: self.line,
                            pos: self.current,
                        })
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        self.add_token(Token {
                            kind: TokenKind::LessEqual,
                            line: self.line,
                            pos: self.current,
                        })
                    } else {
                        self.add_token(Token {
                            kind: TokenKind::Less,
                            line: self.line,
                            pos: self.current,
                        })
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        self.add_token(Token {
                            kind: TokenKind::GreaterEqual,
                            line: self.line,
                            pos: self.current,
                        })
                    } else {
                        self.add_token(Token {
                            kind: TokenKind::Greater,
                            line: self.line,
                            pos: self.current,
                        })
                    }
                }
                '/' => {
                    if self.match_char('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(Token {
                            kind: TokenKind::Slash,
                            line: self.line,
                            pos: self.current,
                        })
                    }
                }
                '"' => self.string(),
                _ if Scanner::is_lox_digit(c) => self.number(),
                _ if Scanner::is_lox_alphabetic(c) => {
                    self.identifier();
                }
                _ => todo!(),
            }
        }
        pub fn scan_tokens(&mut self) -> &Vec<Token> {
            while !self.is_at_end() {
                self.start = self.current;
                self.scan_token();
            }
            self.tokens.push(Token {
                kind: TokenKind::EOF,
                line: self.line,
                pos: self.current,
            });
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
            self.add_token(Token {
                kind: TokenKind::StringLiteral(value),
                line: self.line,
                pos: self.current,
            });
        }

        fn number(&mut self) {
            while Scanner::is_lox_digit(self.peek()) {
                self.advance();
            }
            if self.peek() == '.' && Scanner::is_lox_digit(self.peek_next()) {
                self.advance();

                while Scanner::is_lox_digit(self.peek()) {
                    self.advance();
                }
            }
            let raw = String::from(&self.source[self.start..self.current]);
            let value = raw.parse::<f64>().unwrap();
            self.add_token(Token {
                kind: TokenKind::NumberLiteral(value),
                line: self.line,
                pos: self.current,
            })
        }

        fn peek_next(&self) -> char {
            if self.current + 1 >= self.source.len() {
                '\0'
            } else {
                self.source.chars().nth(self.current + 1).unwrap()
            }
        }

        fn identifier(&mut self) {
            while Scanner::is_lox_alphanumeric(self.peek()) {
                self.advance();
            }
            let text = &self.source[self.start..self.current];
            let token = match text {
                "and" => Token {
                    kind: TokenKind::And,
                    line: self.line,
                    pos: self.current,
                },
                "class" => Token {
                    kind: TokenKind::Class,
                    line: self.line,
                    pos: self.current,
                },
                "else" => Token {
                    kind: TokenKind::Else,
                    line: self.line,
                    pos: self.current,
                },
                "false" => Token {
                    kind: TokenKind::False,
                    line: self.line,
                    pos: self.current,
                },
                "for" => Token {
                    kind: TokenKind::For,
                    line: self.line,
                    pos: self.current,
                },
                "fun" => Token {
                    kind: TokenKind::Fun,
                    line: self.line,
                    pos: self.current,
                },
                "if" => Token {
                    kind: TokenKind::If,
                    line: self.line,
                    pos: self.current,
                },
                "nil" => Token {
                    kind: TokenKind::Nil,
                    line: self.line,
                    pos: self.current,
                },
                "or" => Token {
                    kind: TokenKind::Or,
                    line: self.line,
                    pos: self.current,
                },
                "print" => Token {
                    kind: TokenKind::Print,
                    line: self.line,
                    pos: self.current,
                },
                "return" => Token {
                    kind: TokenKind::Return,
                    line: self.line,
                    pos: self.current,
                },
                "super" => Token {
                    kind: TokenKind::Super,
                    line: self.line,
                    pos: self.current,
                },
                "this" => Token {
                    kind: TokenKind::This,
                    line: self.line,
                    pos: self.current,
                },
                "true" => Token {
                    kind: TokenKind::True,
                    line: self.line,
                    pos: self.current,
                },
                "var" => Token {
                    kind: TokenKind::Var,
                    line: self.line,
                    pos: self.current,
                },
                "while" => Token {
                    kind: TokenKind::While,
                    line: self.line,
                    pos: self.current,
                },
                _ => Token {
                    kind: TokenKind::Identifier(String::from(text)),
                    line: self.line,
                    pos: self.current,
                },
            };
            self.add_token(token)
        }
        fn is_lox_digit(c: char) -> bool {
            c >= '0' && c <= '9'
        }
        fn is_lox_alphabetic(c: char) -> bool {
            (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
        }
        fn is_lox_alphanumeric(c: char) -> bool {
            Scanner::is_lox_alphabetic(c) || Scanner::is_lox_digit(c)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::{Scanner, Token, TokenKind};
    #[test]
    fn single_character_tokens() {
        let source = "(){},-*;".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner
            .scan_tokens()
            .iter()
            .map(|token| Token {
                kind: token.kind.clone(),
                // Because i don't want to track positions when writing tests.
                line: 1,
                pos: 0,
            })
            .collect::<Vec<Token>>();
        assert_eq!(
            *tokens,
            vec![
                Token {
                    kind: TokenKind::LeftParen,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::RightParen,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::LeftBrace,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::RightBrace,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Comma,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Minus,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Star,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Semicolon,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::EOF,
                    line: 1,
                    pos: 0
                }
            ]
        )
    }
    #[test]
    fn ignore_whitespaec() {
        let source = r#" ( ) { },    -
        *;  "#
            .to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner
            .scan_tokens()
            .iter()
            .map(|token| Token {
                kind: token.kind.clone(),
                // Because i don't want to track positions when writing tests.
                line: 1,
                pos: 0,
            })
            .collect::<Vec<Token>>();
        assert_eq!(
            *tokens,
            vec![
                Token {
                    kind: TokenKind::LeftParen,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::RightParen,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::LeftBrace,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::RightBrace,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Comma,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Minus,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Star,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Semicolon,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::EOF,
                    line: 1,
                    pos: 0
                },
            ]
        );
    }
    #[test]
    fn operators() {
        let source = "! != - - = == < <= > >= */".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner
            .scan_tokens()
            .iter()
            .map(|token| Token {
                kind: token.kind.clone(),
                // Because i don't want to track positions when writing tests.
                line: 1,
                pos: 0,
            })
            .collect::<Vec<Token>>();
        assert_eq!(
            *tokens,
            vec![
                Token {
                    kind: TokenKind::Bang,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::BangEqual,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Minus,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Minus,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Equal,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::EqualEqual,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Less,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::LessEqual,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Greater,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::GreaterEqual,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Star,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Slash,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::EOF,
                    line: 1,
                    pos: 0
                },
            ]
        );
    }
    #[test]
    fn comments() {
        let source = r#"// this is a comment
        (), // another comment
        "#
        .to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner
            .scan_tokens()
            .iter()
            .map(|token| Token {
                kind: token.kind.clone(),
                // Because i don't want to track positions when writing tests.
                line: 1,
                pos: 0,
            })
            .collect::<Vec<Token>>();
        assert_eq!(
            *tokens,
            vec![
                Token {
                    kind: TokenKind::LeftParen,
                    line: 1,
                    pos: 0,
                },
                Token {
                    kind: TokenKind::RightParen,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Comma,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::EOF,
                    line: 1,
                    pos: 0
                }
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
        let tokens = scanner
            .scan_tokens()
            .iter()
            .map(|token| Token {
                kind: token.kind.clone(),
                // Because i don't want to track positions when writing tests.
                line: 1,
                pos: 0,
            })
            .collect::<Vec<Token>>();
        assert_eq!(
            *tokens,
            vec![
                Token {
                    kind: TokenKind::StringLiteral("This is a string literal".to_string()),
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::StringLiteral(
                        "This is a
multiline string
literal"
                            .to_string()
                    ),
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::EOF,
                    line: 1,
                    pos: 0
                }
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
                Token {
                    kind: TokenKind::NumberLiteral(123.456),
                    line: 1,
                    pos: 7 //Why does this equal to 7? There seems to be a bug.
                },
                Token {
                    kind: TokenKind::EOF,
                    line: 1,
                    pos: 7
                }
            ]
        )
    }
    #[test]
    fn keywords() {
        let source =
            "and class false fun if nil or print return super this true var while".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner
            .scan_tokens()
            .iter()
            .map(|token| Token {
                kind: token.kind.clone(),
                // Because i don't want to track positions when writing tests.
                line: 1,
                pos: 0,
            })
            .collect::<Vec<Token>>();
        assert_eq!(
            *tokens,
            vec![
                Token {
                    kind: TokenKind::And,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Class,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::False,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Fun,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::If,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Nil,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Or,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Print,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Return,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Super,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::This,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::True,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Var,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::While,
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::EOF,
                    line: 1,
                    pos: 0
                }
            ]
        )
    }
    #[test]
    fn identifiers() {
        let source = "variable iffy classy snake_case_variable".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner
            .scan_tokens()
            .iter()
            .map(|token| Token {
                kind: token.kind.clone(),
                // Because i don't want to track positions when writing tests.
                line: 1,
                pos: 0,
            })
            .collect::<Vec<Token>>();
        assert_eq!(
            *tokens,
            vec![
                Token {
                    kind: TokenKind::Identifier("variable".to_string()),
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Identifier("iffy".to_string()),
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Identifier("classy".to_string()),
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::Identifier("snake_case_variable".to_string()),
                    line: 1,
                    pos: 0
                },
                Token {
                    kind: TokenKind::EOF,
                    line: 1,
                    pos: 0
                },
            ]
        );
    }
}
