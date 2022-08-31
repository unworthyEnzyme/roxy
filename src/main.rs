use roxy::scanner::*;

fn main() {
    let source = "(){},-*;".to_string();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    println!("{:#?}", tokens);
}
