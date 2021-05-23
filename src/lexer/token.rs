use std::fmt;

pub const TT_INT: &str    = "TT_INT";
pub const TT_FLOAT: &str  = "TT_FLOAT";
pub const TT_PLUS: &str   = "TT_PLUS";
pub const TT_MINUS: &str  = "TT_MINUS";
pub const TT_MUL: &str    = "TT_MUL";
pub const TT_DIV: &str    = "TT_DIV";
pub const TT_LPAREN: &str = "TT_LPAREN";
pub const TT_RPAREN: &str = "TT_RPAREN";

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