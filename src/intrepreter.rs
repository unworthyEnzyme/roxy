use crate::parser::{Expr, Literal};

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
    fn eval(expr: Expr) -> Value {
        match expr {
            Expr::Binary(b) => todo!(),
            Expr::Grouping(g) => todo!(),
            Expr::Literal(l) => match l {
                Literal::String(s) => Value::String(s),
                Literal::Number(n) => Value::Number(n),
                Literal::Boolean(b) => Value::Boolean(b),
                Literal::Nil => Value::Nil,
            },
            Expr::Unary(u) => todo!(),
        }
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::{Intrepreter, Value};
    use crate::parser::{Expr, Literal};

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
}
