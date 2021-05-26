use std::fmt;

pub struct Error {
    error_name: String,
    details: String,
}

impl Error {
    pub fn new(error_name: String, details: String) -> Self {
        Self {
            error_name: error_name,
            details: details,
        }
    }
    pub fn illegal_char_error(&mut self) {
        self.error_name = "Illegal Character".to_string();
    }

} 

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if!self.details.is_empty() {
            write!(f, "({}, {})", self.error_name, "NULL")
        } else {
            write!(f,"({}, {})", self.error_name, self.details)
        }
    }

}