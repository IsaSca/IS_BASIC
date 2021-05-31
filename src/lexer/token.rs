use std::fmt;

//Operators
pub static TT_INT: &str    = "TT_INT";
pub static TT_FLOAT: &str  = "TT_FLOAT";
pub static TT_PLUS: &str   = "TT_PLUS";
pub static TT_MINUS: &str  = "TT_MINUS";
pub static TT_MUL: &str    = "TT_MUL";
pub static TT_DIV: &str    = "TT_DIV";
pub static TT_LPAREN: &str = "TT_LPAREN";
pub static TT_RPAREN: &str = "TT_RPAREN";

#[derive(Debug)]
pub struct Token {
    ttype: String,
    value: String
}

impl Token {
    pub fn new(ttype: String, value: String) -> Token {
        Token { ttype, value }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if!self.value.is_empty() {
            write!(f, "({}, {})", self.ttype, "NULL")
        } else {
            write!(f,"({}, {})", self.ttype, self.value)
        }
        
    }
}