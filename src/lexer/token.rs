use std::fmt;

const TT_INT: &str    = "TT_INT";
const TT_FLOAT: &str  = "TT_FLOAT";
const TT_PLUS: &str   = "TT_PLUS";
const TT_MINUS: &str  = "TT_MINUS";
const TT_MUL: &str    = "TT_MUL";
const TT_DIV: &str    = "TT_DIV";
const TT_LPAREN: &str = "TT_LPAREN";
const TT_RPAREN: &str = "TT_RPAREN";



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