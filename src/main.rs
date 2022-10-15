use roxy::{interpreter::Interpreter, parser::Parser, scanner::*};

fn main() {
    let source = r#"print 1 + 2;"#.to_string();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens.clone());
    let stmts = parser.parse();
    Interpreter::interpret(stmts);
}
