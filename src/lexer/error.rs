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

} 

pub struct IllegalCharacterError {
    error: String,
    details: String
}

impl IllegalCharacterError {
    pub fn new(error: Error) -> Self {
        Self {
            error: "Illegal Character".to_string(),
            details: error.details
        }
        
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