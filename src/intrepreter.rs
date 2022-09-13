use crate::parser::{Expr, Literal, UnaryOperator};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct Intrepreter {}

impl Intrepreter {
    //should this function take the ownership of `expr`?
    pub fn eval(expr: Expr) -> Value {
        match expr {
            Expr::Binary(b) => todo!(),
            Expr::Grouping(g) => Intrepreter::eval(*g.expr),
            Expr::Literal(l) => match l {
                Literal::String(s) => Value::String(s),
                Literal::Number(n) => Value::Number(n),
                Literal::Boolean(b) => Value::Boolean(b),
                Literal::Nil => Value::Nil,
            },
            Expr::Unary(u) => {
                let right = Intrepreter::eval(*u.right);
                match u.operator {
                    UnaryOperator::Minus => {
                        if let Value::Number(n) = right {
                            Value::Number(-n)
                        } else {
                            panic!("You can only negate a number")
                        }
                    }
                    UnaryOperator::Not => Value::Boolean(!Intrepreter::is_truthy(&right)),
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
}

#[cfg(test)]
mod interpreter_tests {
    use super::{Intrepreter, Value};
    use crate::parser::{Expr, Literal, Unary, UnaryOperator};

    #[test]
    fn number_literal() {
        let expr = Expr::Literal(Literal::Number(123.2));
        let value = Intrepreter::eval(expr);
        assert_eq!(value, Value::Number(123.2));
    }

    #[test]
    fn string_literal() {
        let expr = Expr::Literal(Literal::String("string".to_string()));
        let value = Intrepreter::eval(expr);
        assert_eq!(value, Value::String("string".to_string()));
    }

    #[test]
    fn bool_literal() {
        let expr = Expr::Literal(Literal::Boolean(false));
        let value = Intrepreter::eval(expr);
        assert_eq!(value, Value::Boolean(false));
    }

    #[test]
    fn nil_literal() {
        let expr = Expr::Literal(Literal::Nil);
        let value = Intrepreter::eval(expr);
        assert_eq!(value, Value::Nil);
    }

    #[test]
    fn unary_expr_number() {
        let expr = Expr::Unary(Unary {
            operator: UnaryOperator::Minus,
            right: Box::new(Expr::Literal(Literal::Number(42.0))),
        });

        let val = Intrepreter::eval(expr);
        assert_eq!(val, Value::Number(-42.0));
    }

    #[test]
    fn unary_expr_bool() {
        let expr = Expr::Unary(Unary {
            operator: UnaryOperator::Not,
            right: Box::new(Expr::Literal(Literal::Boolean(false))),
        });

        let val = Intrepreter::eval(expr);
        assert_eq!(val, Value::Boolean(true));
    }
}
