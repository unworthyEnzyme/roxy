use crate::scanner::{Keywords, Literals, Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
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
    left: Box<Expr>,
    operator: BinaryOperator,
    right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
    operator: UnaryOperator,
    right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn literal(&mut self) -> Result<Literal, ()> {
        let token = self.tokens[self.current].clone();
        let token = match token {
            Token::Literal(Literals::String { value }) => Ok(Literal::String(value)),
            Token::Literal(Literals::Number(n)) => Ok(Literal::Number(n.value)),
            Token::Keyword(Keywords::True) => Ok(Literal::Boolean(true)),
            Token::Keyword(Keywords::False) => Ok(Literal::Boolean(false)),
            Token::Keyword(Keywords::Nil) => Ok(Literal::Nil),
            _ => Err(()),
        };
        self.current += 1;
        token
    }

    fn is_at_end(&mut self) -> bool {
        self.peek() == &Token::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current + 1]
    }
}

#[cfg(test)]
mod parser_tests {
    use super::Parser;
    use crate::{parser::Literal, scanner::Scanner};

    #[test]
    fn parsing_literals() {
        let source = r#"123.456 "a string literal" nil true false"#.to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let (_, tail) = tokens.split_last().unwrap();
        let mut parser = Parser::new(tail.to_vec().clone());
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
}
