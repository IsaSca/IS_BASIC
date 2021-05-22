pub struct Lexer {
    text: String,
    pos: Int,
    current_char: char,
    text_vector: Vec<char> //Vector for easy indexing of the input text
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Token(text, -1, None, text.chars().collect())
    }

    pub fn advance(&self) {
        self.pos += 1;
        if self.pos < self.text_vector.len() {
            self.current_char = self.text_vector[self.pos];
        } else {
            self.current_char = None;
        }
        

    }
}
