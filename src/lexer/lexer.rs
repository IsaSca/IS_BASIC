pub struct Lexer {
    text: String,
    pos: usize,
    current_char: char,
}

impl Lexer {
    fn new(line: String) -> Self {
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
}
