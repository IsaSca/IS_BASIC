use read_input::prelude::*;

#[path = "lexer/lexer.rs"] mod lexer;
#[path = "lexer/token.rs"] mod token;


fn main() {
    loop {
        let input: String = input().msg("is_bash > ").get();
        if input == "exit" {
            break;
        }
        println!("{}", input);

    }
}
