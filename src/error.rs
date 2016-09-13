use std::error::Error;

#[derive(Debug)]
struct GeneralError {
    message: String,
}

use std::fmt;
impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for GeneralError {
    fn description(&self) -> &str {
        self.message.as_ref()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<String> for GeneralError {
    fn from(str: String) -> Self {
        GeneralError { message: str }
    }
}
