mod token;
mod error;

pub struct Lexer {
    text: String,
    pos: usize,
    current_char: char,
}

impl Lexer {
    pub fn new(line: String) -> Self {
        Self {
            text: line,
            pos: 0,
            current_char: '\0',
        }
        
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.text.chars().count() {
            self.current_char = self.text.chars().nth(self.pos).unwrap();
        } else {
            self.current_char = '\0';
        }
    }

    pub fn make_tokens(&mut self) -> Vec<token::Token> {
        let mut tokens: Vec<token::Token> = Vec::new(); 
        while self.current_char != '\0' {
            // if token::TT_DIGIT.iter().any(|&i| i ==self.current_char) {
            //     tokens.push(self.make_number())
            // }
            match self.current_char {
                ' ' | '\t' => self.advance(),
                '+' => tokens.push(token::Token::new(token::TT_PLUS.to_string(), String::from(""))),
                '-' => tokens.push(token::Token::new(token::TT_MINUS.to_string(), String::from(""))),
                '*' => tokens.push(token::Token::new(token::TT_MUL.to_string(), String::from(""))),
                '/' => tokens.push(token::Token::new(token::TT_DIV.to_string(), String::from(""))),
                '(' => tokens.push(token::Token::new(token::TT_LPAREN.to_string(), String::from(""))),
                ')' => tokens.push(token::Token::new(token::TT_RPAREN.to_string(), String::from(""))),
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => tokens.push(self.make_number()),
                _ => {
                    let char = self.current_char;
                    let mut dead_vec: Vec<token::Token> = Vec::new();
                    dead_vec.push(token::Token::new(error::Error::new("Illegal Character".to_string(), char.to_string()).to_string(), char.to_string()));
                    self.advance();
                    return dead_vec
                    
                },
            }
        }
        return tokens;
    }

    pub fn make_number(&mut self) -> token::Token {
        let mut num_str = "".to_string();
        let mut dot_count = 0;
        let num_range = 0..9;
        while self.current_char != '\0' && num_range.contains(&self.current_char.to_digit(10).unwrap()) {
            if self.current_char == '.' {
                if dot_count == 1 {
                    break;
                } else {
                    dot_count += 1;
                    num_str.push_str(&".");
                }
            } else {
                num_str.push_str(&self.current_char.to_string());
            }

        }
        if dot_count == 0 {
            return token::Token::new(String::from(token::TT_INT), String::from(num_str))
        } else {
            return token::Token::new(String::from(token::TT_FLOAT), String::from(num_str))
        }
    }

}
