use std::fmt;

#[derive(Debug)]
pub struct Error {
    desc: String,
}

impl Error {
    pub fn new(desc: &str) -> Error {
        Error {
            desc: String::from(desc),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl std::error::Error for Error {}
