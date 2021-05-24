mod token;

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
            if token::TT_DIGIT.iter().any(|&i| i ==self.current_char.to_string()) {
                tokens.push(self.make_number())
            }
            match self.current_char {
                ' ' | '\t' => self.advance(),
                '+' => tokens.push(token::Token::new(token::TT_PLUS.to_string(), String::from(""))),
                '-' => tokens.push(token::Token::new(token::TT_MINUS.to_string(), String::from(""))),
                '*' => tokens.push(token::Token::new(token::TT_MUL.to_string(), String::from(""))),
                '/' => tokens.push(token::Token::new(token::TT_DIV.to_string(), String::from(""))),
                '(' => tokens.push(token::Token::new(token::TT_LPAREN.to_string(), String::from(""))),
                ')' => tokens.push(token::Token::new(token::TT_RPAREN.to_string(), String::from(""))),
                _ => self.advance(),
            }
        }
        return tokens;
    }

    pub fn make_number(&mut self) -> token::Token {
        let mut num_str = "".to_string();
        let mut dot_count = 0;
        while self.current_char != '\0' && token::TT_DIGIT.iter().any(|&i| i ==self.current_char.to_string()) {
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
