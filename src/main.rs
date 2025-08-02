mod token;

use logos::Logos;
use token::Token;

fn main() {
    let src = r#"
    for i in range do (
        println(i)
        if math.is_prime(i) then True
        else False
    )
    "#;

    let lex = Token::lexer(src);

    for result in lex {
        match result {
            Ok(token) => println!("{:#?}", token),
            Err(_) => panic!("Syntax Error"),
        }
    }
}
