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
            if self.current_char == ' ' || self.current_char == '\t' {
                self.advance();
            } else {
                if self.current_char == '+' {
                    tokens.push(token::Token::new(token::TT_PLUS.to_string(), String::from("")))
                }
            }
        }
        return tokens;
    }

}
