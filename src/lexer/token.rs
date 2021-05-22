TT_INT = "TT_INT"
TT_FLOAT = "TT_FLOAT"
TT_PLUS = "TT_PLUS"
TT_MINUS = "TT_MINUS"
TT_MUL = "TT_MUL"
TT_DIV = "TT_DIV"
TT_LPAREN = "TT_LPAREN"
TT_RPAREN = "TT_RPAREN"



pub struct Token {
    ttype: String,
    value: String
}

impl Token {
    pub fn new(ttype: String, value: String) -> Self {
        Token { ttype, value }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if(!self.value.is_empty()) {
            write!(f, "({}, {})", self.ttype, "NULL")
        } else {
            write!(f,"({}, {})", self.ttype, self.value)
        }
        
    }
}