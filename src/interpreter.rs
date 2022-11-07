use crate::parser::{BinaryOperator, Expr, Literal, UnaryOperator, Stmt};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct Interpreter {}

impl Interpreter {
    pub fn eval(expr: &Expr) -> Value {
        match expr {
            Expr::Binary(b) => {
                let left = Interpreter::eval(&b.left);
                let right = Interpreter::eval(&b.right);
                match b.operator {
                    BinaryOperator::Minus => {
                        if let (Value::Number(n1), Value::Number(n2)) = (left, right) {
                            Value::Number(n1 - n2)
                        } else {
                            panic!("You can only substract numbers")
                        }
                    }
                    BinaryOperator::Plus => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),
                        (Value::String(s1), Value::String(s2)) => {
                            Value::String(format!("{}{}", s1, s2))
                        }
                        _ => panic!("You can only add strings or numbers"),
                    },
                    BinaryOperator::Multiply => {
                        if let (Value::Number(n1), Value::Number(n2)) = (left, right) {
                            Value::Number(n1 * n2)
                        } else {
                            panic!("You can only multiply numbers")
                        }
                    }
                    BinaryOperator::Divide => {
                        if let (Value::Number(n1), Value::Number(n2)) = (left, right) {
                            Value::Number(n1 / n2)
                        } else {
                            panic!("You can only multiply numbers")
                        }
                    }
                    BinaryOperator::GreaterThan => {
                        if let (Value::Number(n1), Value::Number(n2)) = (left, right) {
                            Value::Boolean(n1 > n2)
                        } else {
                            panic!("You can only multiply numbers")
                        }
                    }
                    BinaryOperator::LessThan => {
                        if let (Value::Number(n1), Value::Number(n2)) = (left, right) {
                            Value::Boolean(n1 < n2)
                        } else {
                            panic!("You can only multiply numbers")
                        }
                    }
                    BinaryOperator::GreaterThanEqual => {
                        if let (Value::Number(n1), Value::Number(n2)) = (left, right) {
                            Value::Boolean(n1 >= n2)
                        } else {
                            panic!("You can only multiply numbers")
                        }
                    }
                    BinaryOperator::LessThanEqual => {
                        if let (Value::Number(n1), Value::Number(n2)) = (left, right) {
                            Value::Boolean(n1 <= n2)
                        } else {
                            panic!("You can only multiply numbers")
                        }
                    }
                    //What happens in the case of non-primitive values?
                    BinaryOperator::EqualEqual => Value::Boolean(left == right),
                    BinaryOperator::NotEqual => Value::Boolean(left != right),
                }
            }
            Expr::Grouping(g) => Interpreter::eval(&g.expr),
            Expr::Literal(l) => match l {
                Literal::String(s) => Value::String(s.to_string()),
                Literal::Number(n) => Value::Number(*n),
                Literal::Boolean(b) => Value::Boolean(*b),
                Literal::Nil => Value::Nil,
            },
            Expr::Unary(u) => {
                let right = Interpreter::eval(&u.right);
                match u.operator {
                    UnaryOperator::Minus => {
                        if let Value::Number(n) = right {
                            Value::Number(-n)
                        } else {
                            panic!("You can only negate a number")
                        }
                    }
                    UnaryOperator::Not => Value::Boolean(!Interpreter::is_truthy(&right)),
                }
            }
        }
    }
    fn is_truthy(right: &Value) -> bool {
        match right {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }
    pub fn interpret(statements: Vec<Stmt>) {
        for stmt in statements {
            Interpreter::execute(stmt);
        }
    }

    fn execute(stmt: Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let value = Interpreter::eval(&expr);
                match value {
                    Value::String(v) => println!("{}", v),
                    Value::Boolean(v) => println!("{}", v),
                    Value::Nil => println!("nil"),
                    Value::Number(v) => println!("{}", v),
                }
            }
            Stmt::Expression(expr) => {
                //This has no side-effect as far as i can see
                //so why do we do this?
                let _ = Interpreter::eval(&expr);
            }
            Stmt::Var { name, initializer } => todo!(),
            Stmt::Block(_) => todo!(),
            _ => todo!(),
        };
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::{Interpreter, Value};
    use crate::{
        parser::{Expr, Literal, Parser, Unary, UnaryOperator},
        scanner::Scanner,
    };

    #[test]
    fn number_literal() {
        let expr = Expr::Literal(Literal::Number(123.2));
        let value = Interpreter::eval(&expr);
        assert_eq!(value, Value::Number(123.2));
    }

    #[test]
    fn string_literal() {
        let expr = Expr::Literal(Literal::String("string".to_string()));
        let value = Interpreter::eval(&expr);
        assert_eq!(value, Value::String("string".to_string()));
    }

    #[test]
    fn bool_literal() {
        let expr = Expr::Literal(Literal::Boolean(false));
        let value = Interpreter::eval(&expr);
        assert_eq!(value, Value::Boolean(false));
    }

    #[test]
    fn nil_literal() {
        let expr = Expr::Literal(Literal::Nil);
        let value = Interpreter::eval(&expr);
        assert_eq!(value, Value::Nil);
    }

    #[test]
    fn unary_expr_number() {
        let expr = Expr::Unary(Unary {
            operator: UnaryOperator::Minus,
            right: Box::new(Expr::Literal(Literal::Number(42.0))),
        });

        let val = Interpreter::eval(&expr);
        assert_eq!(val, Value::Number(-42.0));
    }

    #[test]
    fn unary_expr_bool() {
        let expr = Expr::Unary(Unary {
            operator: UnaryOperator::Not,
            right: Box::new(Expr::Literal(Literal::Boolean(false))),
        });

        let val = Interpreter::eval(&expr);
        assert_eq!(val, Value::Boolean(true));
    }

    #[test]
    fn binary_expression() {
        let source = "(5 - (3 - 1)) + -1".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.clone());
        let expr = parser.expression();
        let val = Interpreter::eval(&expr);
        assert_eq!(val, Value::Number(2.0));
    }

    #[test]
    #[should_panic]
    fn incorrect_binary_expression() {
        let source = r#"2 * (3 / -"muffin")"#.to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.clone());
        let expr = parser.expression();
        let _ = Interpreter::eval(&expr);
    }
}
