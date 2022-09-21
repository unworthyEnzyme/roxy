use crate::scanner::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var { name: String, initializer: Expr },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
    Grouping(Grouping),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOperator {
    EqualEqual,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: BinaryOperator,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn literal(&mut self) -> Result<Literal, ()> {
        let token = self.tokens[self.current].clone();
        let token = match token.kind {
            TokenKind::StringLiteral(s) => Ok(Literal::String(s)),
            TokenKind::NumberLiteral(n) => Ok(Literal::Number(n)),
            TokenKind::True => Ok(Literal::Boolean(true)),
            TokenKind::False => Ok(Literal::Boolean(false)),
            TokenKind::Nil => Ok(Literal::Nil),
            _ => Err(()),
        };
        self.current += 1;
        token
    }

    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_tokens(vec![TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let operator = match operator.kind {
                TokenKind::BangEqual => BinaryOperator::NotEqual,
                TokenKind::EqualEqual => BinaryOperator::EqualEqual,
                _ => panic!("only != and == is allowed"),
            };
            let right = self.comparison();
            expr = Expr::Binary(Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right.clone()),
            })
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_tokens(vec![
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous();
            let operator = match operator.kind {
                TokenKind::Greater => BinaryOperator::GreaterThan,
                TokenKind::GreaterEqual => BinaryOperator::GreaterThanEqual,
                TokenKind::Less => BinaryOperator::LessThan,
                TokenKind::LessEqual => BinaryOperator::LessThanEqual,
                _ => panic!("only >, >=, < and <= is allowed as an operator"),
            };
            let right = self.term();
            expr = Expr::Binary(Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right.clone()),
            })
        }
        expr
    }

    fn match_tokens(&mut self, tokens: Vec<TokenKind>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn check(&self, token: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == token
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_tokens(vec![TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous();
            let operator = match operator.kind {
                TokenKind::Minus => BinaryOperator::Minus,
                TokenKind::Plus => BinaryOperator::Plus,
                _ => panic!("Only - and + operators are allowed"),
            };
            let right = self.factor();
            expr = Expr::Binary(Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right.clone()),
            })
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_tokens(vec![TokenKind::Slash, TokenKind::Star]) {
            let operator = self.previous();
            let operator = match operator.kind {
                TokenKind::Slash => BinaryOperator::Divide,
                TokenKind::Star => BinaryOperator::Multiply,
                _ => panic!("only / and * is allowed as an operator"),
            };
            let right = self.unary();
            expr = Expr::Binary(Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right.clone()),
            })
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(vec![TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            print!("{:?}", operator);
            let operator = match operator.kind {
                TokenKind::Bang => UnaryOperator::Not,
                TokenKind::Minus => UnaryOperator::Minus,
                _ => panic!("Only ! and - operator is allowed"),
            };
            let right = self.unary();
            return Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(vec![TokenKind::False]) {
            Expr::Literal(Literal::Boolean(false))
        } else if self.match_tokens(vec![TokenKind::True]) {
            Expr::Literal(Literal::Boolean(true))
        } else if self.match_tokens(vec![TokenKind::Nil]) {
            Expr::Literal(Literal::Nil)
        } else if matches!(
            self.tokens[self.current].clone().kind,
            TokenKind::NumberLiteral(_),
        ) | matches!(
            self.tokens[self.current].clone().kind,
            TokenKind::StringLiteral(_),
        ) {
            let token = self.tokens[self.current].clone();
            return match token.kind {
                TokenKind::NumberLiteral(n) => {
                    self.advance();
                    Expr::Literal(Literal::Number(n))
                }
                TokenKind::StringLiteral(s) => {
                    self.advance();
                    Expr::Literal(Literal::String(s))
                }
                _ => panic!("Only strings or numbers allowed"),
            };
        } else {
            match self.match_tokens(vec![TokenKind::LeftParen]) {
                true => {
                    let expr = self.expression();
                    self.consume(TokenKind::RightParen, "Expect ')' after expression");
                    Expr::Grouping(Grouping {
                        expr: Box::new(expr),
                    })
                }
                false => panic!("is this part unreachable?"),
            }
        }
    }

    fn consume(&mut self, token: TokenKind, err_msg: &str) -> Token {
        if self.check(token) {
            self.advance()
        } else {
            panic!("{:#?} {}", self.peek(), err_msg);
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }

    fn statement(&mut self) -> Stmt {
        match self.tokens[self.current].kind {
            TokenKind::Print => {
                self.advance();
                self.print_statement()
            }
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after value.");
        Stmt::Print(value)
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after expression.");
        Stmt::Expression(expr)
    }

    fn declaration(&mut self) -> Stmt {
        if self.match_tokens(vec![TokenKind::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Stmt {
        // i am not sure how to remove this `.clone()` call.
        let name = self.tokens[self.current].clone();
        self.advance();
        let mut initializer = Expr::Literal(Literal::Nil);
        if self.match_tokens(vec![TokenKind::Equal]) {
            initializer = self.expression();
        }

        self.consume(
            TokenKind::Semicolon,
            "Expect ';' after variable declaration",
        );
        if let TokenKind::Identifier(name) = name.kind {
            Stmt::Var { name, initializer }
        } else {
            panic!("Variable declarations require an identifier")
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use super::{Expr, Parser, Stmt};
    use crate::{parser::Literal, scanner::Scanner};

    #[test]
    fn parsing_literals() {
        let source = r#"123.456 "a string literal" nil true false"#.to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let (_, tail) = tokens.split_last().unwrap();
        let mut parser = Parser::new(tail.to_vec());
        let mut literals: Vec<Literal> = vec![];
        for _ in tail {
            literals.push(parser.literal().unwrap())
        }
        assert_eq!(
            literals,
            vec![
                Literal::Number(123.456),
                Literal::String("a string literal".to_string()),
                Literal::Nil,
                Literal::Boolean(true),
                Literal::Boolean(false)
            ]
        );
    }

    #[test]
    fn binary_expr() {
        let source = r#"-1 - 2 * (4 - 2)"#.to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.clone());
        let expr = parser.expression();
        println!("{:#?}", expr);
    }

    #[test]
    fn var_declaration() {
        let source = r#"var age = 26;"#.to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.clone());
        let program = parser.parse();
        assert_eq!(
            program,
            vec![Stmt::Var {
                name: "age".to_string(),
                initializer: Expr::Literal(Literal::Number(26.0))
            }]
        )
    }
}
